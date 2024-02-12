// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use regex::Regex;

pub struct Context {
    pub host_regex: Regex,
    pub github_template: String,
}

impl Context {
    pub fn is_rfd_number_valid(&self, rfd_number: u16) -> bool {
        0 < rfd_number
    }
}
