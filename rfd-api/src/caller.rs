// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeSet;
use v_model::permissions::Caller;

use crate::permissions::RfdPermission;

pub trait CallerExt {
    fn allow_rfds(&self) -> BTreeSet<i32>;
}

impl CallerExt for Caller<RfdPermission> {
    fn allow_rfds(&self) -> BTreeSet<i32> {
        let mut allowed = BTreeSet::new();
        for permission in self.permissions.iter() {
            match permission {
                RfdPermission::GetRfd(number) => {
                    allowed.insert(*number);
                }
                RfdPermission::GetRfds(numbers) => allowed.extend(numbers),
                _ => (),
            }
        }

        allowed
    }
}
