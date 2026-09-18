// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use clap::{Parser, ValueEnum};
use newline_converter::dos2unix;
use progenitor::{GenerationSettings, Generator, TagStyle};
use regex::Regex;
use semver::Version;
use serde_json::Value;

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "build tasks")]
enum Xtask {
    #[command(about = "bump the global version number and open a pull request")]
    #[command(arg_required_else_help = true)]
    Bump {
        #[clap(long, help = "Allow non-main git branch or dirty tree")]
        dirty: bool,
        place: VersionPlace,
    },
    #[command(about = "generate RFD sdk")]
    Generate,
}

#[derive(Clone, ValueEnum)]
enum VersionPlace {
    Major,
    Minor,
    Patch,
}

fn main() -> Result<(), String> {
    let xtask = Xtask::parse();

    match xtask {
        Xtask::Bump { dirty, place } => bump(&place, dirty),
        Xtask::Generate => generate(),
    }
}

fn bump(place: &VersionPlace, dirty: bool) -> Result<(), String> {
    let root_path = workspace_root();
    ensure_release_state(&root_path, dirty)?;

    let old_version = read_workspace_version(&root_path)?;
    let bump_result = bump_on_pr_branch(&root_path, &old_version, place)?;

    let undo_command = [
        format!("git checkout {}", shell_quote(&bump_result.original_branch)),
        format!("git branch -D {}", shell_quote(&bump_result.bump_branch)),
    ]
    .join(" && ");
    let publish_command = [bump_result.push_cmd, bump_result.pr_cmd].join(" && \\\n    ");

    print_next_steps(&undo_command, &publish_command);

    Ok(())
}

fn print_next_steps(undo_cmd: &str, good_cmd: &str) {
    println!();
    println!("If you would like to undo:");
    println!("  {undo_cmd}");
    println!();
    println!("If this looks good:");
    println!("  {good_cmd}");
    println!();
}

struct BumpPrBranch {
    bump_branch: String,
    original_branch: String,
    push_cmd: String,
    pr_cmd: String,
}

fn bump_on_pr_branch(
    root_path: &Path,
    old_version: &Version,
    place: &VersionPlace,
) -> Result<BumpPrBranch, String> {
    let original_branch = git_output(root_path, ["branch", "--show-current"])?;
    let new_version = old_version.clone().up(place);
    let bump_branch = format!("bump_v{}", new_version);
    git_status(root_path, ["checkout", "-b", &bump_branch])?;

    println!("Bumping version number from {old_version} to {new_version}");
    bump_package_versions(root_path, &new_version)?;
    generate()?;

    println!();
    let commit_message = format!("Bump to v{}", new_version);
    git_status(
        root_path,
        [
            "add",
            "Cargo.toml",
            "Cargo.lock",
            "rfd-api-spec.json",
            "rfd-sdk/src/generated",
            "rfd-cli/src/generated",
            "rfd-ts",
        ],
    )?;
    git_status(root_path, ["commit", "-m", commit_message.as_str()])?;
    git_status(root_path, ["checkout", &original_branch])?;
    let quoted_bump_branch = shell_quote(&bump_branch);
    let quoted_commit_message = shell_quote(&commit_message);
    let push_command = format!("git push -q -u origin {quoted_bump_branch}");
    let pr_command = [
        "gh pr create --web --base main".to_string(),
        format!("--head {quoted_bump_branch}"),
        format!("--title {quoted_commit_message}"),
    ]
    .join(" ");

    Ok(BumpPrBranch {
        bump_branch,
        original_branch,
        push_cmd: push_command,
        pr_cmd: pr_command,
    })
}

fn bump_package_versions(root_path: &Path, version: &Version) -> Result<(), String> {
    update_workspace_version(root_path, version)?;
    for node_package in NODE_PACKAGES {
        update_node_package_version(root_path, node_package, version)?;
    }

    println!("Running cargo check to update Cargo.lock...");
    let status = Command::new("cargo")
        .args(["check", "-q"])
        .current_dir(root_path)
        .status()
        .map_err(|e| format!("Failed to run cargo check: {}", e))?;
    if !status.success() {
        return Err("cargo check failed".to_string());
    }

    Ok(())
}

/// Node packages whose version tracks the workspace version.
const NODE_PACKAGES: &[&str] = &["rfd-ts"];

fn ensure_release_state(root_path: &Path, dirty: bool) -> Result<(), String> {
    let branch = git_output(root_path, ["branch", "--show-current"])?;
    if branch != "main" && !dirty {
        return Err(format!(
            "release must be run from main, currently on {}",
            branch
        ));
    }

    let status = git_output(root_path, ["status", "--porcelain", "--untracked-files=no"])?;
    if !status.is_empty() && !dirty {
        return Err("release requires no modified tracked files".to_string());
    }

    git_status(
        root_path,
        ["fetch", "origin", "main:refs/remotes/origin/main", "--tags"],
    )?;

    let local_main = git_output(root_path, ["rev-parse", "main"])?;
    let origin_main = git_output(root_path, ["rev-parse", "origin/main"])?;
    if local_main != origin_main {
        return Err([
            "Your local main does not match origin/main.".to_string(),
            format!("main:        {local_main:.7}"),
            format!("origin/main: {origin_main:.7}"),
            "Probably need to `git pull`".to_string(),
        ]
        .join("\n"));
    }

    Ok(())
}

fn read_workspace_version(root_path: &Path) -> Result<Version, String> {
    let cargo_toml = root_path.join("Cargo.toml");
    let contents = fs::read_to_string(cargo_toml).map_err(|e| e.to_string())?;
    let version_pattern = Regex::new(r#"(?m)^version = "(.*)"$"#).unwrap();
    let version_line = version_pattern
        .captures(&contents)
        .ok_or("could not find workspace package version")?;
    version_line
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .map_err(|e| format!("failed to parse workspace version: {}", e))
}

fn update_workspace_version(root_path: &Path, version: &Version) -> Result<(), String> {
    let cargo_toml = root_path.join("Cargo.toml");
    let contents = fs::read_to_string(&cargo_toml).map_err(|e| e.to_string())?;
    let version_pattern = Regex::new(r#"(?m)^version = "(.*)"$"#).unwrap();
    let version_line = version_pattern
        .captures(&contents)
        .ok_or("could not find workspace package version")?;
    let old_version_line = version_line.get(0).unwrap().as_str();
    let new_version_line = format!(r#"version = "{}""#, version);
    let new_contents = contents.replace(old_version_line, &new_version_line);
    fs::write(cargo_toml, new_contents).map_err(|e| e.to_string())?;
    println!("Updated workspace to {}", version);
    Ok(())
}

fn update_node_package_version(
    root_path: &Path,
    node_package: &str,
    version: &Version,
) -> Result<(), String> {
    let package_dir = root_path.join(node_package);
    let package_json = package_dir.join("package.json");
    let contents = fs::read_to_string(&package_json).map_err(|e| e.to_string())?;
    let parsed: Value = serde_json::from_str(&contents).map_err(|e| e.to_string())?;
    let old_version = parsed
        .get("version")
        .and_then(|version| version.as_str())
        .ok_or_else(|| format!("could not find version in {}/package.json", node_package))?;

    // Rewrite the version line in place rather than reserializing the document, so that
    // formatting and key order of the hand maintained package.json are preserved.
    let contents = replace_json_version_lines(&contents, old_version, version, 1)?;
    fs::write(&package_json, contents).map_err(|e| e.to_string())?;

    println!("Updated {} to {}", node_package, version);

    // package.json alone leaves package-lock.json holding the old version, so refresh the
    // lockfile without touching node_modules.
    println!(
        "Running `npm install --package-lock-only` in {}",
        node_package
    );
    let status = Command::new("npm")
        .args([
            "install",
            "--package-lock-only",
            "--silent",
            "--no-audit",
            "--no-fund",
        ])
        .current_dir(&package_dir)
        .status()
        .map_err(|e| {
            format!(
                "failed to run npm install --package-lock-only in {}: {}",
                node_package, e
            )
        })?;
    if !status.success() {
        return Err(format!(
            "npm install --package-lock-only failed in {}",
            node_package
        ));
    }

    Ok(())
}

fn replace_json_version_lines(
    contents: &str,
    old_version: &str,
    new_version: &Version,
    expected_replacements: usize,
) -> Result<String, String> {
    let version_pattern = Regex::new(&format!(
        r#"(?m)^(\s+"version":\s+"){}(".*)$"#,
        regex::escape(old_version)
    ))
    .unwrap();
    let replacements = version_pattern.find_iter(contents).count();
    if replacements < expected_replacements {
        return Err(format!(
            "expected at least {} version entries for {}, found {}",
            expected_replacements, old_version, replacements
        ));
    }
    Ok(version_pattern
        .replacen(
            contents,
            expected_replacements,
            format!("${{1}}{}${{2}}", new_version),
        )
        .to_string())
}

fn git_output<const N: usize>(root_path: &Path, args: [&str; N]) -> Result<String, String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(root_path)
        .output()
        .map_err(|e| format!("failed to run git: {}", e))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn git_status<const N: usize>(root_path: &Path, args: [&str; N]) -> Result<(), String> {
    let status = Command::new("git")
        .args(args)
        .current_dir(root_path)
        .status()
        .map_err(|e| format!("failed to run git: {}", e))?;
    if !status.success() {
        return Err(format!("git {} failed", args.join(" ")));
    }
    Ok(())
}

fn npm<const N: usize>(dir: &Path, args: [&str; N]) -> Result<(), String> {
    let status = Command::new("npm")
        .args(args)
        .current_dir(dir)
        .status()
        .map_err(|e| format!("failed to run npm: {}", e))?;
    if !status.success() {
        return Err(format!("npm {} failed", args.join(" ")));
    }
    Ok(())
}

fn workspace_root() -> PathBuf {
    let xtask_path = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    xtask_path.parent().unwrap().to_path_buf()
}

fn shell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', r#"'\''"#))
}

trait Bump {
    fn up(self, place: &VersionPlace) -> Self;
}

impl Bump for Version {
    fn up(mut self, place: &VersionPlace) -> Self {
        match place {
            VersionPlace::Major => {
                self.major += 1;
                self.minor = 0;
                self.patch = 0;
            }
            VersionPlace::Minor => {
                self.minor += 1;
                self.patch = 0;
            }
            VersionPlace::Patch => {
                self.patch += 1;
            }
        }

        self
    }
}

fn generate() -> Result<(), String> {
    let root_path = workspace_root();
    let mut spec_path = root_path.clone();
    spec_path.push("rfd-api-spec.json");

    // Regenerate the OpenAPI spec from the API server itself, so the SDKs below are always
    // derived from the canonical description. CI runs `cargo xtask generate` and diffs the
    // result to catch hand edits and stale checked-in output.
    print!("generating spec ... ");
    std::io::stdout().flush().unwrap();
    let output = Command::new("cargo")
        .args(["run", "--quiet", "-p", "rfd-api", "--", "describe"])
        .current_dir(&root_path)
        .output()
        .map_err(|err| format!("failed to run rfd-api describe: {}", err))?;
    if !output.status.success() {
        return Err(format!(
            "rfd-api describe failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    fs::write(&spec_path, &output.stdout).map_err(|e| e.to_string())?;
    println!("done.");

    let file = File::open(spec_path).unwrap();
    let spec = serde_json::from_reader(file).unwrap();
    let mut generator = Generator::new(
        GenerationSettings::default()
            .with_interface(progenitor::InterfaceStyle::Builder)
            .with_tag(TagStyle::Separate)
            .with_derive("schemars::JsonSchema"),
    );

    // // TODO I'd like to generate a hash as well to have a way to check if the
    // // spec has changed since the last generation.

    // // SDK
    print!("generating sdk ... ");
    std::io::stdout().flush().unwrap();

    let code = generator.generate_tokens(&spec).unwrap().to_string();
    let contents = format_code(code);

    let mut out_path = root_path.clone();
    out_path.push("rfd-sdk");
    out_path.push("src");
    out_path.push("generated");
    out_path.push("sdk.rs");

    std::fs::write(out_path, contents).unwrap();
    println!("done.");

    // CLI
    print!("generating cli ... ");
    std::io::stdout().flush().unwrap();
    let code = generator.cli(&spec, "rfd_sdk").unwrap().to_string();
    let contents = format_code(format!("{}\n\n{}", "use rfd_sdk::*;", code));

    let mut out_path = root_path.clone();
    out_path.push("rfd-cli");
    out_path.push("src");
    out_path.push("generated");
    out_path.push("cli.rs");

    std::fs::write(out_path, contents).unwrap();
    println!("done.");

    // Typescript SDK
    let xtask_dir = root_path.join("xtask");
    print!("installing typescript sdk generator ... ");
    std::io::stdout().flush().unwrap();
    npm(
        &xtask_dir,
        ["install", "--silent", "--no-audit", "--no-fund"],
    )?;
    println!("done.");

    print!("generating typescript sdk ... ");
    std::io::stdout().flush().unwrap();
    npm(&xtask_dir, ["run", "--silent", "generate-ts"])?;
    println!("done.");

    // The generator's parseIfDate heuristic doesn't include `_at` suffixes,
    // but our API uses fields like created_at, updated_at, expires_at.
    let util_path = root_path.join("rfd-ts/src/util.ts");
    let util_contents = fs::read_to_string(&util_path).map_err(|e| e.to_string())?;
    let patched = util_contents.replace(
        r#"k?.endsWith("_expiration")"#,
        "k?.endsWith(\"_expiration\") ||\n      k?.endsWith(\"_at\")",
    );
    if patched == util_contents {
        return Err("Failed to patch util.ts: could not find _expiration pattern".to_string());
    }
    fs::write(&util_path, patched).map_err(|e| e.to_string())?;

    print!("formatting typescript sdk ... ");
    Command::new("dprint")
        .arg("fmt")
        .current_dir(&root_path)
        .output()
        .map_err(|err| err.to_string())?;
    println!("done.");

    Ok(())
}

fn format_code(code: String) -> String {
    let contents = format!(
        "// The contents of this file are generated; do not modify them.\n\n{}",
        code,
    );
    let contents = rustfmt_wrapper::rustfmt_config(
        rustfmt_wrapper::config::Config {
            format_strings: Some(true),
            normalize_doc_attributes: Some(true),
            wrap_comments: Some(true),
            ..Default::default()
        },
        contents,
    )
    .expect("rustfmt_wrapper: ensure a nightly toolchain is installed.");
    let contents = dos2unix(&contents);

    // Add newlines after end-braces at <= two levels of indentation. Rustfmt's
    // `blank_lines_lower_bound` is broken.
    let regex = regex::regex!(r#"(})(\n\s{0,8}[^} ])"#);
    let contents = regex.replace_all(&contents, "$1\n$2");

    let regex = regex::regex!(r#"(\n\s*///)(\S)"#);
    regex.replace_all(&contents, "$1 $2").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn workspace_members(root: &Path) -> Vec<String> {
        let contents = fs::read_to_string(root.join("Cargo.toml")).unwrap();
        let parsed: toml::Table = contents.parse().unwrap();
        parsed["workspace"]["members"]
            .as_array()
            .unwrap()
            .iter()
            .map(|member| member.as_str().unwrap().to_string())
            .collect()
    }

    /// Every workspace member must inherit the workspace version, so that a single
    /// `cargo xtask bump` keeps the whole workspace on one version.
    #[test]
    fn package_versions_are_consistent() {
        let root = workspace_root();
        for package in workspace_members(&root) {
            let contents = fs::read_to_string(root.join(&package).join("Cargo.toml")).unwrap();
            let parsed: toml::Table = contents.parse().unwrap();
            let workspace = parsed["package"]["version"]
                .as_table()
                .and_then(|version| version.get("workspace"))
                .and_then(|workspace| workspace.as_bool());
            assert_eq!(
                workspace,
                Some(true),
                "{package} should use `version.workspace = true`"
            );
        }
    }

    /// The node packages are bumped alongside the workspace, and their lockfiles record the
    /// package version twice. All of them must agree with the workspace version.
    #[test]
    fn node_package_versions_match_workspace() {
        let root = workspace_root();
        let version = read_workspace_version(&root).unwrap().to_string();

        for package in NODE_PACKAGES {
            let package_dir = root.join(package);

            let package_json: Value = serde_json::from_str(
                &fs::read_to_string(package_dir.join("package.json")).unwrap(),
            )
            .unwrap();
            assert_eq!(
                package_json.get("version").and_then(|v| v.as_str()),
                Some(version.as_str()),
                "{package}/package.json version should match the workspace version"
            );

            let lock: Value = serde_json::from_str(
                &fs::read_to_string(package_dir.join("package-lock.json")).unwrap(),
            )
            .unwrap();
            assert_eq!(
                lock.get("version").and_then(|v| v.as_str()),
                Some(version.as_str()),
                "{package}/package-lock.json version should match the workspace version"
            );
            assert_eq!(
                lock.pointer("/packages//version").and_then(|v| v.as_str()),
                Some(version.as_str()),
                "{package}/package-lock.json root package version should match the workspace version"
            );
        }
    }
}
