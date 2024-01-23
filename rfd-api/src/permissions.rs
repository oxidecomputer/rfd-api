// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use partial_struct::partial;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;
use uuid::Uuid;
use w_api_permissions::Permissions;

use crate::ApiPermissions;

#[derive(Debug, Error)]
pub enum ApiPermissionError {
    #[error("Scope is invalid: {0}")]
    InvalidScope(String),
}

#[partial(ApiPermissionResponse, attributes(#[serde(tag = "kind", content = "value")]))]
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub enum ApiPermission {
    // User information permissions
    CreateApiUserToken(Uuid),
    CreateApiUserTokenSelf,
    CreateApiUserTokenAssigned,
    CreateApiUserTokenAll,
    GetApiUser(Uuid),
    GetApiUserSelf,
    GetApiUserAssigned,
    GetApiUserAll,
    GetApiUserToken(Uuid),
    GetApiUserTokenSelf,
    GetApiUserTokenAssigned,
    GetApiUserTokenAll,
    DeleteApiUserToken(Uuid),
    DeleteApiUserTokenSelf,
    DeleteApiUserTokenAssigned,
    DeleteApiUserTokenAll,
    CreateApiUser,
    UpdateApiUser(Uuid),
    UpdateApiUserSelf,
    UpdateApiUserAssigned,
    UpdateApiUserAll,

    // User provider permissions
    CreateUserApiProviderLinkToken,

    // Group permissions,
    GetGroupsJoined,
    GetGroupsAll,
    CreateGroup,
    UpdateGroup(Uuid),
    AddToGroup(Uuid),
    RemoveFromGroup(Uuid),
    ManageGroupMembership(Uuid),
    ManageGroupMemberships(BTreeSet<Uuid>),
    ManageGroupMembershipAssigned,
    ManageGroupMembershipAll,
    DeleteGroup(Uuid),
    ManageGroup(Uuid),
    ManageGroups(BTreeSet<Uuid>),
    ManageGroupsAssigned,
    ManageGroupsAll,

    // Mapper permissions
    ListMappers,
    CreateMapper,
    UpdateMapper(Uuid),
    DeleteMapper(Uuid),
    ManageMapper(Uuid),
    ManageMappers(BTreeSet<Uuid>),
    ManageMappersAssigned,
    ManageMappersAll,

    // RFD access permissions
    GetRfd(i32),
    GetRfds(BTreeSet<i32>),
    GetRfdsAssigned,
    GetRfdsAll,
    GetDiscussion(i32),
    GetDiscussions(BTreeSet<i32>),
    GetDiscussionsAssigned,
    GetDiscussionsAll,
    SearchRfds,

    // OAuth client manage permissions
    CreateOAuthClient,
    GetOAuthClient(Uuid),
    GetOAuthClients(BTreeSet<Uuid>),
    GetOAuthClientsAssigned,
    GetOAuthClientsAll,
    UpdateOAuthClient(Uuid),
    UpdateOAuthClients(BTreeSet<Uuid>),
    UpdateOAuthClientsAssigned,
    UpdateOAuthClientsAll,
    DeleteOAuthClient(Uuid),
    DeleteOAuthClients(BTreeSet<Uuid>),
    DeleteOAuthClientsAssigned,
    DeleteOAuthClientsAll,

    // Internal permissions
    CreateAccessToken,

    // Removed
    #[serde(other)]
    Removed,
}

impl ApiPermission {
    pub fn as_scope(&self) -> &str {
        match self {
            ApiPermission::CreateApiUserToken(_) => "user:token:w",
            ApiPermission::CreateApiUserTokenSelf => "user:token:w",
            ApiPermission::CreateApiUserTokenAssigned => "user:token:w",
            ApiPermission::CreateApiUserTokenAll => "user:token:w",
            ApiPermission::GetApiUser(_) => "user:info:r",
            ApiPermission::GetApiUserSelf => "user:info:r",
            ApiPermission::GetApiUserAssigned => "user:info:r",
            ApiPermission::GetApiUserAll => "user:info:r",
            ApiPermission::GetApiUserToken(_) => "user:token:r",
            ApiPermission::GetApiUserTokenSelf => "user:token:r",
            ApiPermission::GetApiUserTokenAssigned => "user:token:r",
            ApiPermission::GetApiUserTokenAll => "user:token:r",
            ApiPermission::DeleteApiUserToken(_) => "user:token:w",
            ApiPermission::DeleteApiUserTokenSelf => "user:token:w",
            ApiPermission::DeleteApiUserTokenAssigned => "user:token:w",
            ApiPermission::DeleteApiUserTokenAll => "user:token:w",
            ApiPermission::CreateApiUser => "user:info:w",
            ApiPermission::UpdateApiUser(_) => "user:info:w",
            ApiPermission::UpdateApiUserSelf => "user:info:w",
            ApiPermission::UpdateApiUserAssigned => "user:info:w",
            ApiPermission::UpdateApiUserAll => "user:info:w",

            ApiPermission::CreateUserApiProviderLinkToken => "user:provider:w",

            ApiPermission::GetGroupsJoined => "group:r",
            ApiPermission::GetGroupsAll => "group:r",
            ApiPermission::CreateGroup => "group:w",
            ApiPermission::UpdateGroup(_) => "group:w",
            ApiPermission::AddToGroup(_) => "group:membership:w",
            ApiPermission::RemoveFromGroup(_) => "group:membership:w",
            ApiPermission::ManageGroupMembership(_) => "group:membership:w",
            ApiPermission::ManageGroupMemberships(_) => "group:membership:w",
            ApiPermission::ManageGroupMembershipAssigned => "group:membership:w",
            ApiPermission::ManageGroupMembershipAll => "group:membership:w",
            ApiPermission::DeleteGroup(_) => "group:w",
            ApiPermission::ManageGroup(_) => "group:w",
            ApiPermission::ManageGroups(_) => "group:w",
            ApiPermission::ManageGroupsAssigned => "group:w",
            ApiPermission::ManageGroupsAll => "group:w",

            ApiPermission::ListMappers => "mapper:r",
            ApiPermission::CreateMapper => "mapper:w",
            ApiPermission::UpdateMapper(_) => "mapper:w",
            ApiPermission::DeleteMapper(_) => "mapper:w",
            ApiPermission::ManageMapper(_) => "mapper:w",
            ApiPermission::ManageMappers(_) => "mapper:w",
            ApiPermission::ManageMappersAssigned => "mapper:w",
            ApiPermission::ManageMappersAll => "mapper:w",

            ApiPermission::GetRfd(_) => "rfd:content:r",
            ApiPermission::GetRfds(_) => "rfd:content:r",
            ApiPermission::GetRfdsAssigned => "rfd:content:r",
            ApiPermission::GetRfdsAll => "rfd:content:r",
            ApiPermission::GetDiscussion(_) => "rfd:discussion:r",
            ApiPermission::GetDiscussions(_) => "rfd:discussion:r",
            ApiPermission::GetDiscussionsAll => "rfd:discussion:r",
            ApiPermission::GetDiscussionsAssigned => "rfd:discussion:r",
            ApiPermission::SearchRfds => "search",

            ApiPermission::CreateOAuthClient => "oauth:client:w",
            ApiPermission::GetOAuthClient(_) => "oauth:client:r",
            ApiPermission::GetOAuthClients(_) => "oauth:client:r",
            ApiPermission::GetOAuthClientsAssigned => "oauth:client:r",
            ApiPermission::GetOAuthClientsAll => "oauth:client:r",
            ApiPermission::UpdateOAuthClient(_) => "oauth:client:w",
            ApiPermission::UpdateOAuthClients(_) => "oauth:client:w",
            ApiPermission::UpdateOAuthClientsAssigned => "oauth:client:w",
            ApiPermission::UpdateOAuthClientsAll => "oauth:client:w",
            ApiPermission::DeleteOAuthClient(_) => "oauth:client:w",
            ApiPermission::DeleteOAuthClients(_) => "oauth:client:w",
            ApiPermission::DeleteOAuthClientsAssigned => "oauth:client:w",
            ApiPermission::DeleteOAuthClientsAll => "oauth:client:w",

            ApiPermission::CreateAccessToken => "",

            ApiPermission::Removed => "",
        }
    }

    pub fn from_scope_arg(scope_arg: &str) -> Result<ApiPermissions, ApiPermissionError> {
        Self::from_scope(scope_arg.split(' '))
    }

    pub fn from_scope<S>(
        scope: impl Iterator<Item = S>,
    ) -> Result<ApiPermissions, ApiPermissionError>
    where
        S: AsRef<str>,
    {
        let mut permissions = ApiPermissions::new();

        for entry in scope {
            match entry.as_ref() {
                "user:info:r" => {
                    permissions.insert(ApiPermission::GetApiUserSelf);
                    permissions.insert(ApiPermission::GetApiUserAll);
                }
                "user:info:w" => {
                    permissions.insert(ApiPermission::UpdateApiUserSelf);
                    permissions.insert(ApiPermission::UpdateApiUserAssigned);
                    permissions.insert(ApiPermission::UpdateApiUserAll);
                }
                "user:provider:w" => {
                    permissions.insert(ApiPermission::CreateUserApiProviderLinkToken);
                }
                "user:token:r" => {
                    permissions.insert(ApiPermission::GetApiUserTokenSelf);
                    permissions.insert(ApiPermission::GetApiUserTokenAssigned);
                    permissions.insert(ApiPermission::GetApiUserTokenAll);
                }
                "user:token:w" => {
                    permissions.insert(ApiPermission::CreateApiUserTokenSelf);
                    permissions.insert(ApiPermission::CreateApiUserTokenAssigned);
                    permissions.insert(ApiPermission::CreateApiUserTokenAll);
                    permissions.insert(ApiPermission::DeleteApiUserTokenSelf);
                    permissions.insert(ApiPermission::DeleteApiUserTokenAssigned);
                    permissions.insert(ApiPermission::DeleteApiUserTokenAll);
                }
                "group:r" => {
                    permissions.insert(ApiPermission::GetGroupsJoined);
                    permissions.insert(ApiPermission::GetGroupsAll);
                }
                "group:w" => {
                    permissions.insert(ApiPermission::CreateGroup);
                    permissions.insert(ApiPermission::ManageGroupsAssigned);
                    permissions.insert(ApiPermission::ManageGroupsAll);
                }
                "group:membership:w" => {
                    permissions.insert(ApiPermission::ManageGroupMembershipAssigned);
                    permissions.insert(ApiPermission::ManageGroupMembershipAll);
                }
                "mapper:r" => {
                    permissions.insert(ApiPermission::ListMappers);
                }
                "mapper:w" => {
                    permissions.insert(ApiPermission::CreateMapper);
                    permissions.insert(ApiPermission::ManageMappersAssigned);
                    permissions.insert(ApiPermission::ManageMappersAll);
                }
                "rfd:content:r" => {
                    permissions.insert(ApiPermission::GetRfdsAssigned);
                    permissions.insert(ApiPermission::GetRfdsAll);
                }
                "rfd:discussion:r" => {
                    permissions.insert(ApiPermission::GetDiscussionsAssigned);
                    permissions.insert(ApiPermission::GetDiscussionsAll);
                }
                "search" => {
                    permissions.insert(ApiPermission::SearchRfds);
                }
                "oauth:client:r" => {
                    permissions.insert(ApiPermission::GetOAuthClientsAssigned);
                    permissions.insert(ApiPermission::GetOAuthClientsAll);
                }
                "oauth:client:w" => {
                    permissions.insert(ApiPermission::CreateOAuthClient);
                    permissions.insert(ApiPermission::UpdateOAuthClientsAssigned);
                    permissions.insert(ApiPermission::UpdateOAuthClientsAll);
                    permissions.insert(ApiPermission::DeleteOAuthClientsAssigned);
                    permissions.insert(ApiPermission::DeleteOAuthClientsAll);
                }
                other => return Err(ApiPermissionError::InvalidScope(other.to_string())),
            }
        }

        Ok(permissions)
    }
}

pub trait PermissionStorage {
    fn contract(&self, owner: &Uuid) -> Self;
    fn expand(&self, owner: &Uuid, owner_permissions: Option<&ApiPermissions>) -> Self;
}

impl PermissionStorage for Permissions<ApiPermission> {
    fn contract(&self, owner: &Uuid) -> Self {
        let mut contracted = Vec::new();

        let mut manage_group_memberships = BTreeSet::<Uuid>::new();
        let mut manage_groups = BTreeSet::<Uuid>::new();
        let mut rfds = BTreeSet::<i32>::new();
        let mut discussions = BTreeSet::<i32>::new();
        let mut read_oauth_clients = BTreeSet::<Uuid>::new();
        let mut update_oauth_clients = BTreeSet::<Uuid>::new();
        let mut delete_oauth_clients = BTreeSet::<Uuid>::new();

        for p in self.iter() {
            match p {
                ApiPermission::GetApiUser(id) => contracted.push(if id == owner {
                    ApiPermission::GetApiUserSelf
                } else {
                    ApiPermission::GetApiUser(*id)
                }),
                ApiPermission::CreateApiUserToken(id) => contracted.push(if id == owner {
                    ApiPermission::CreateApiUserTokenSelf
                } else {
                    ApiPermission::CreateApiUserToken(*id)
                }),
                ApiPermission::GetApiUserToken(id) => contracted.push(if id == owner {
                    ApiPermission::GetApiUserTokenSelf
                } else {
                    ApiPermission::GetApiUserToken(*id)
                }),
                ApiPermission::DeleteApiUserToken(id) => contracted.push(if id == owner {
                    ApiPermission::DeleteApiUserTokenSelf
                } else {
                    ApiPermission::DeleteApiUserToken(*id)
                }),
                ApiPermission::UpdateApiUser(id) => contracted.push(if id == owner {
                    ApiPermission::UpdateApiUserSelf
                } else {
                    ApiPermission::UpdateApiUser(*id)
                }),

                ApiPermission::ManageGroupMembership(id) => {
                    manage_group_memberships.insert(*id);
                }
                ApiPermission::ManageGroup(id) => {
                    manage_groups.insert(*id);
                }

                ApiPermission::GetRfd(number) => {
                    rfds.insert(*number);
                }
                ApiPermission::GetDiscussion(number) => {
                    discussions.insert(*number);
                }
                ApiPermission::GetOAuthClient(id) => {
                    read_oauth_clients.insert(*id);
                }
                ApiPermission::UpdateOAuthClient(id) => {
                    update_oauth_clients.insert(*id);
                }
                ApiPermission::DeleteOAuthClient(id) => {
                    delete_oauth_clients.insert(*id);
                }

                other => contracted.push(other.clone()),
            }
        }

        contracted.push(ApiPermission::ManageGroupMemberships(
            manage_group_memberships,
        ));
        contracted.push(ApiPermission::ManageGroups(manage_groups));
        contracted.push(ApiPermission::GetRfds(rfds));
        contracted.push(ApiPermission::GetDiscussions(discussions));
        contracted.push(ApiPermission::GetOAuthClients(read_oauth_clients));
        contracted.push(ApiPermission::UpdateOAuthClients(update_oauth_clients));
        contracted.push(ApiPermission::DeleteOAuthClients(delete_oauth_clients));

        contracted.into()
    }

    fn expand(&self, owner: &Uuid, owner_permissions: Option<&ApiPermissions>) -> Self {
        let mut expanded = Vec::new();

        for p in self.iter() {
            match p {
                ApiPermission::GetApiUserSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::GetApiUser(*owner))
                }
                ApiPermission::CreateApiUserTokenSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::CreateApiUserToken(*owner))
                }
                ApiPermission::GetApiUserTokenSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::GetApiUserToken(*owner))
                }
                ApiPermission::DeleteApiUserTokenSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::DeleteApiUserToken(*owner))
                }
                ApiPermission::UpdateApiUserSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::UpdateApiUser(*owner))
                }

                ApiPermission::ManageGroupMemberships(ids) => {
                    for id in ids {
                        expanded.push(ApiPermission::ManageGroupMembership(*id))
                    }
                }
                ApiPermission::ManageGroups(ids) => {
                    for id in ids {
                        expanded.push(ApiPermission::ManageGroup(*id))
                    }
                }

                ApiPermission::GetRfds(numbers) => {
                    for number in numbers {
                        expanded.push(ApiPermission::GetRfd(*number))
                    }
                }
                ApiPermission::GetDiscussions(numbers) => {
                    for number in numbers {
                        expanded.push(ApiPermission::GetDiscussion(*number))
                    }
                }
                ApiPermission::GetOAuthClients(ids) => {
                    for id in ids {
                        expanded.push(ApiPermission::GetOAuthClient(*id))
                    }
                }
                ApiPermission::UpdateOAuthClients(ids) => {
                    for id in ids {
                        expanded.push(ApiPermission::UpdateOAuthClient(*id))
                    }
                }
                ApiPermission::DeleteOAuthClients(ids) => {
                    for id in ids {
                        expanded.push(ApiPermission::DeleteOAuthClient(*id))
                    }
                }

                ApiPermission::ManageGroupMembershipAssigned => {
                    expanded.push(p.clone());

                    if let Some(owner_permissions) = owner_permissions {
                        for p in owner_permissions.iter() {
                            match p {
                                ApiPermission::ManageGroupMembership(id) => {
                                    expanded.push(ApiPermission::ManageGroupMembership(*id))
                                }
                                _ => (),
                            }
                        }
                    }
                }
                ApiPermission::ManageGroupsAssigned => {
                    expanded.push(p.clone());

                    if let Some(owner_permissions) = owner_permissions {
                        for p in owner_permissions.iter() {
                            match p {
                                ApiPermission::ManageGroup(id) => {
                                    expanded.push(ApiPermission::ManageGroup(*id))
                                }
                                _ => (),
                            }
                        }
                    }
                }
                ApiPermission::GetRfdsAssigned => {
                    expanded.push(p.clone());

                    if let Some(owner_permissions) = owner_permissions {
                        for p in owner_permissions.iter() {
                            match p {
                                ApiPermission::GetRfd(number) => {
                                    expanded.push(ApiPermission::GetRfd(*number))
                                }
                                _ => (),
                            }
                        }
                    }
                }
                ApiPermission::GetOAuthClientsAssigned => {
                    expanded.push(p.clone());

                    if let Some(owner_permissions) = owner_permissions {
                        for p in owner_permissions.iter() {
                            match p {
                                ApiPermission::GetOAuthClient(id) => {
                                    expanded.push(ApiPermission::GetOAuthClient(*id))
                                }
                                _ => (),
                            }
                        }
                    }
                }
                ApiPermission::UpdateOAuthClientsAssigned => {
                    expanded.push(p.clone());

                    if let Some(owner_permissions) = owner_permissions {
                        for p in owner_permissions.iter() {
                            match p {
                                ApiPermission::UpdateOAuthClient(id) => {
                                    expanded.push(ApiPermission::UpdateOAuthClient(*id))
                                }
                                _ => (),
                            }
                        }
                    }
                }
                ApiPermission::DeleteOAuthClientsAssigned => {
                    expanded.push(p.clone());

                    if let Some(owner_permissions) = owner_permissions {
                        for p in owner_permissions.iter() {
                            match p {
                                ApiPermission::DeleteOAuthClient(id) => {
                                    expanded.push(ApiPermission::DeleteOAuthClient(*id))
                                }
                                _ => (),
                            }
                        }
                    }
                }

                other => expanded.push(other.clone()),
            }
        }

        expanded.into()
    }
}
