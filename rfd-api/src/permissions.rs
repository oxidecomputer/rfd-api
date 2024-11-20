// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use v_api::permissions::VPermission;
use v_api_permission_derive::v_api;

#[v_api(From(VPermission))]
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema,
)]
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
    #[v_api(scope(to = "rfd:content:r", from = "rfd:content:r"))]
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
    #[v_api(scope(to = "rfd:content:w", from = "rfd:content:w"))]
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
    #[v_api(scope(to = "rfd:visibility:w", from = "rfd:visibility:w"))]
    ManageRfdsVisibilityAll,
    #[v_api(
        contract(kind = append, variant = GetDiscussions),
        scope(to = "rfd:discussion:w")
    )]
    GetDiscussion(i32),
    #[v_api(
        contract(kind = extend, variant = GetDiscussions),
        expand(kind = iter, variant = GetDiscussion)
        scope(to = "rfd:discussion:w")
    )]
    GetDiscussions(BTreeSet<i32>),
    #[v_api(
        expand(kind = alias, variant = GetDiscussion, source = actor),
        scope(to = "rfd:discussion:w", from = "rfd:discussion:w")
    )]
    GetDiscussionsAssigned,
    #[v_api(scope(to = "rfd:discussion:w", from = "rfd:discussion:w"))]
    GetDiscussionsAll,
    #[v_api(scope(to = "search", from = "search"))]
    SearchRfds,
}
