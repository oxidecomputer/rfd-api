// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use anyhow::Result;
use clap::Parser;

use crate::context::Context;

/// Print the version
#[derive(Debug, Parser)]
#[clap(name = "version")]
pub struct VersionCmd {}

impl VersionCmd {
    pub async fn run(&self, _ctx: &mut Context) -> Result<()> {
        println!(
            "{} ({}, {})",
            env!("CARGO_PKG_VERSION"),
            env!("RFD_CLI_GIT_HASH"),
            env!("RFD_CLI_BUILD_TYPE"),
        );

        Ok(())
    }
}
