use std::collections::BTreeSet;

use rfd_model::permissions::Caller;

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
