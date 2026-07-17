// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    // Without these, cargo only reruns this script when build.rs itself changes, so a cached
    // `target/` restored for a different commit (e.g. via Swatinem/rust-cache in CI) would keep
    // reporting the git hash of whichever commit last actually executed this script.
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/logs/HEAD");

    let short_hash = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let is_dirty = std::process::Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=no"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    let hash = if is_dirty {
        format!("{short_hash}-dirty")
    } else {
        short_hash
    };

    let is_tagged = std::process::Command::new("git")
        .args(["describe", "--exact-match", "--tags", "HEAD"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let in_ci = std::env::var_os("CI").is_some() || std::env::var_os("GITHUB_ACTIONS").is_some();

    let build_type = if is_tagged && in_ci { "release" } else { "dev" };

    println!("cargo:rustc-env=RFD_PROCESSOR_GIT_HASH={hash}");
    println!("cargo:rustc-env=RFD_PROCESSOR_BUILD_TYPE={build_type}");
}
