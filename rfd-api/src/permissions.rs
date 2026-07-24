// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use strum::EnumIter;
use v_api::permissions::VPermission;
use v_api_permission_derive::v_api;

#[v_api(From(VPermission))]
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, EnumIter)]
pub enum RfdPermission {
    #[v_api(
        contract(kind = append, variant = GetRfds),
        scope(to = "rfd:content:r")
    )]
    GetRfd(i32),
    #[v_api(
        contract(kind = extend, variant = GetRfds),
        expand(kind = iter, variant = GetRfd)
        scope(to = "rfd:content:r")
    )]
    GetRfds(BTreeSet<i32>),
    #[v_api(
        expand(kind = alias, variant = GetRfd, source = actor),
        scope(to = "rfd:content:r", from = "rfd:content:r")
    )]
    GetRfdsAssigned,
    #[v_api(
        implies(variant = GetRfd),
        implies(variant = GetRfds),
        implies(variant = GetRfdsAssigned),
        scope(to = "rfd:content:r", from = "rfd:content:r")
    )]
    GetRfdsAll,
    #[v_api(scope(to = "rfd:content:w", from = "rfd:content:w"))]
    CreateRfd,
    #[v_api(
        contract(kind = append, variant = UpdateRfds),
        scope(to = "rfd:content:w")
    )]
    UpdateRfd(i32),
    #[v_api(
        contract(kind = extend, variant = UpdateRfds),
        expand(kind = iter, variant = UpdateRfd)
        scope(to = "rfd:content:w")
    )]
    UpdateRfds(BTreeSet<i32>),
    #[v_api(
        expand(kind = alias, variant = UpdateRfd, source = actor),
        scope(to = "rfd:content:w", from = "rfd:content:w")
    )]
    UpdateRfdsAssigned,
    #[v_api(
        implies(variant = UpdateRfd),
        implies(variant = UpdateRfds),
        implies(variant = UpdateRfdsAssigned),
        scope(to = "rfd:content:w", from = "rfd:content:w")
    )]
    UpdateRfdsAll,
    #[v_api(
        contract(kind = append, variant = ManageRfdsVisibility),
        scope(to = "rfd:visibility:w")
    )]
    ManageRfdVisibility(i32),
    #[v_api(
        contract(kind = extend, variant = ManageRfdsVisibility),
        expand(kind = iter, variant = ManageRfdVisibility)
        scope(to = "rfd:visibility:w")
    )]
    ManageRfdsVisibility(BTreeSet<i32>),
    #[v_api(
        expand(kind = alias, variant = ManageRfdVisibility, source = actor),
        scope(to = "rfd:visibility:w", from = "rfd:visibility:w")
    )]
    ManageRfdsVisibilityAssigned,
    #[v_api(
        implies(variant = ManageRfdVisibility),
        implies(variant = ManageRfdsVisibility),
        implies(variant = ManageRfdsVisibilityAssigned),
        scope(to = "rfd:visibility:w", from = "rfd:visibility:w")
    )]
    ManageRfdsVisibilityAll,
    #[v_api(
        contract(kind = append, variant = GetDiscussions),
        scope(to = "rfd:discussion:r")
    )]
    GetDiscussion(i32),
    #[v_api(
        contract(kind = extend, variant = GetDiscussions),
        expand(kind = iter, variant = GetDiscussion)
        scope(to = "rfd:discussion:r")
    )]
    GetDiscussions(BTreeSet<i32>),
    #[v_api(
        expand(kind = alias, variant = GetDiscussion, source = actor),
        scope(to = "rfd:discussion:r", from = "rfd:discussion:r")
    )]
    GetDiscussionsAssigned,
    #[v_api(
        implies(variant = GetDiscussion),
        implies(variant = GetDiscussions),
        implies(variant = GetDiscussionsAssigned),
        scope(to = "rfd:discussion:r", from = "rfd:discussion:r")
    )]
    GetDiscussionsAll,
    #[v_api(scope(to = "search", from = "search"))]
    SearchRfds,
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use strum::IntoEnumIterator;
    use v_model::permissions::{PermissionStorage, Permissions};

    use super::RfdPermission;

    #[test]
    fn all_variants_imply_granular_variants() {
        assert!(RfdPermission::implies(
            &RfdPermission::GetRfdsAll,
            &RfdPermission::GetRfd(591)
        ));
        assert!(RfdPermission::implies(
            &RfdPermission::GetRfdsAll,
            &RfdPermission::GetRfds(BTreeSet::from([1, 2, 3]))
        ));
        assert!(RfdPermission::implies(
            &RfdPermission::GetRfdsAll,
            &RfdPermission::GetRfdsAssigned
        ));

        assert!(RfdPermission::implies(
            &RfdPermission::UpdateRfdsAll,
            &RfdPermission::UpdateRfd(591)
        ));
        assert!(RfdPermission::implies(
            &RfdPermission::ManageRfdsVisibilityAll,
            &RfdPermission::ManageRfdVisibility(591)
        ));
        assert!(RfdPermission::implies(
            &RfdPermission::GetDiscussionsAll,
            &RfdPermission::GetDiscussion(591)
        ));
    }

    #[test]
    fn granular_variants_do_not_imply_all_variants() {
        assert!(!RfdPermission::implies(
            &RfdPermission::GetRfd(591),
            &RfdPermission::GetRfdsAll
        ));
        assert!(!RfdPermission::implies(
            &RfdPermission::GetRfds(BTreeSet::from([591])),
            &RfdPermission::GetRfdsAll
        ));
    }

    #[test]
    fn all_variants_do_not_imply_across_families() {
        assert!(!RfdPermission::implies(
            &RfdPermission::GetRfdsAll,
            &RfdPermission::UpdateRfd(591)
        ));
        assert!(!RfdPermission::implies(
            &RfdPermission::GetRfdsAll,
            &RfdPermission::GetDiscussion(591)
        ));
    }

    #[test]
    fn set_variants_imply_contained_elements() {
        assert!(RfdPermission::implies(
            &RfdPermission::GetRfds(BTreeSet::from([1, 591])),
            &RfdPermission::GetRfd(591)
        ));
        assert!(!RfdPermission::implies(
            &RfdPermission::GetRfds(BTreeSet::from([1])),
            &RfdPermission::GetRfd(591)
        ));
        assert!(RfdPermission::implies(
            &RfdPermission::GetRfds(BTreeSet::from([1, 2, 591])),
            &RfdPermission::GetRfds(BTreeSet::from([1, 591]))
        ));
    }

    // Regression test: at login v-api re-applies mapped groups, and
    // add_api_user_to_group requires the builtin registration caller to pass
    // can_grant_all(group.permissions). The registration caller is built from
    // RfdPermission::iter() (see main.rs), which holds the *All variants. It
    // must be able to grant groups composed of granular per-RFD permissions,
    // otherwise every login matching such a mapper fails with a 403.
    #[test]
    fn registration_permissions_can_grant_granular_group_permissions() {
        let registration: Permissions<RfdPermission> =
            RfdPermission::iter().collect::<Vec<_>>().into();

        let customer_group: Permissions<RfdPermission> = vec![
            RfdPermission::GetRfd(216),
            RfdPermission::GetRfd(591),
            RfdPermission::GetRfds(BTreeSet::from([7, 343, 584])),
        ]
        .into();

        assert!(registration.can_grant_all(&customer_group));
    }
}
