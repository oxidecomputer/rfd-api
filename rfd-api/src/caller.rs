// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeSet;
use w_api_permissions::Caller;

use crate::permissions::ApiPermission;

pub trait CallerExt {
    fn allow_rfds(&self) -> BTreeSet<i32>;
}

impl CallerExt for Caller<ApiPermission> {
    fn allow_rfds(&self) -> BTreeSet<i32> {
        let mut allowed = BTreeSet::new();
        for permission in self.permissions.iter() {
            match permission {
                ApiPermission::GetRfd(number) => {
                    allowed.insert(*number);
                }
                ApiPermission::GetRfds(numbers) => allowed.extend(numbers),
                _ => (),
            }
        }

        allowed
    }
}
