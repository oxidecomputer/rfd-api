// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Shared `build.rs` logic for embedding a git hash and build type into a
//! crate via `cargo:rustc-env`. Intended to be used as a `[build-dependencies]`
//! entry by workspace member crates.

use std::process::Command;

/// Emits `cargo:rustc-env={env_prefix}_GIT_HASH` and
/// `cargo:rustc-env={env_prefix}_BUILD_TYPE`, along with the
/// `cargo:rerun-if-changed` directives needed to keep the git hash accurate
/// when `target/` is restored from a cache for a different commit (e.g. via
/// Swatinem/rust-cache in CI).
///
/// Assumes the calling crate's manifest directory is one level below the
/// workspace root (i.e. the git directory is at `../.git`).
pub fn emit(env_prefix: &str) {
    println!("cargo:rerun-if-changed=../.git/HEAD");
    println!("cargo:rerun-if-changed=../.git/logs/HEAD");

    let short_hash = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let is_dirty = Command::new("git")
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

    let is_tagged = Command::new("git")
        .args(["describe", "--exact-match", "--tags", "HEAD"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let in_ci = std::env::var_os("CI").is_some() || std::env::var_os("GITHUB_ACTIONS").is_some();

    let build_type = if is_tagged && in_ci { "release" } else { "dev" };

    println!("cargo:rustc-env={env_prefix}_GIT_HASH={hash}");
    println!("cargo:rustc-env={env_prefix}_BUILD_TYPE={build_type}");
}
