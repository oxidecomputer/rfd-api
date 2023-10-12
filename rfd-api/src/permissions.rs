use rfd_model::{permissions::Permissions, ApiUser, NewApiUser};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

pub trait PermissionStorage {
    fn contract(&self, user: &impl UserPermissionHolder) -> Self;
    fn expand(&self, user: &impl UserPermissionHolder) -> Self;
}

pub trait UserPermissionHolder {
    fn id(&self) -> &Uuid;
    fn permissions(&self) -> &Permissions<ApiPermission>;
}

impl UserPermissionHolder for ApiUser<ApiPermission> {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn permissions(&self) -> &Permissions<ApiPermission> {
        &self.permissions
    }
}

impl UserPermissionHolder for NewApiUser<ApiPermission> {
    fn id(&self) -> &Uuid {
        &self.id
    }

    fn permissions(&self) -> &Permissions<ApiPermission> {
        &self.permissions
    }
}

impl PermissionStorage for Permissions<ApiPermission> {
    fn contract(&self, user: &impl UserPermissionHolder) -> Self {
        let mut contracted = Vec::new();

        let mut rfds = BTreeSet::<i32>::new();
        let mut discussions = BTreeSet::<i32>::new();
        let mut read_oauth_clients = BTreeSet::<Uuid>::new();
        let mut update_oauth_clients = BTreeSet::<Uuid>::new();
        let mut delete_oauth_clients = BTreeSet::<Uuid>::new();

        for p in self.iter() {
            match p {
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

                ApiPermission::GetApiUser(id) => contracted.push(if id == user.id() {
                    ApiPermission::GetApiUserSelf
                } else {
                    ApiPermission::GetApiUser(*id)
                }),
                ApiPermission::CreateApiUserToken(id) => contracted.push(if id == user.id() {
                    ApiPermission::CreateApiUserTokenSelf
                } else {
                    ApiPermission::CreateApiUserToken(*id)
                }),
                ApiPermission::GetApiUserToken(id) => contracted.push(if id == user.id() {
                    ApiPermission::GetApiUserTokenSelf
                } else {
                    ApiPermission::GetApiUserToken(*id)
                }),
                ApiPermission::DeleteApiUserToken(id) => contracted.push(if id == user.id() {
                    ApiPermission::DeleteApiUserTokenSelf
                } else {
                    ApiPermission::DeleteApiUserToken(*id)
                }),
                ApiPermission::UpdateApiUser(id) => contracted.push(if id == user.id() {
                    ApiPermission::UpdateApiUserSelf
                } else {
                    ApiPermission::UpdateApiUser(*id)
                }),

                other => contracted.push(other.clone()),
            }
        }

        contracted.push(ApiPermission::GetRfds(rfds));
        contracted.push(ApiPermission::GetDiscussions(discussions));
        contracted.push(ApiPermission::GetOAuthClients(read_oauth_clients));
        contracted.push(ApiPermission::UpdateOAuthClients(update_oauth_clients));
        contracted.push(ApiPermission::DeleteOAuthClients(delete_oauth_clients));

        contracted.into()
    }

    fn expand(&self, user: &impl UserPermissionHolder) -> Self {
        let mut expanded = Vec::new();

        for p in self.iter() {
            match p {
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

                ApiPermission::GetAssignedRfds => {
                    expanded.push(p.clone());
                    for p in user.permissions().iter() {
                        match p {
                            ApiPermission::GetRfd(number) => {
                                expanded.push(ApiPermission::GetRfd(*number))
                            }
                            _ => (),
                        }
                    }
                }
                ApiPermission::GetAssignedOAuthClients => {
                    expanded.push(p.clone());
                    for p in user.permissions().iter() {
                        match p {
                            ApiPermission::GetOAuthClient(id) => {
                                expanded.push(ApiPermission::GetOAuthClient(*id))
                            }
                            _ => (),
                        }
                    }
                }
                ApiPermission::UpdateAssignedOAuthClients => {
                    expanded.push(p.clone());
                    for p in user.permissions().iter() {
                        match p {
                            ApiPermission::UpdateOAuthClient(id) => {
                                expanded.push(ApiPermission::UpdateOAuthClient(*id))
                            }
                            _ => (),
                        }
                    }
                }
                ApiPermission::DeleteAssignedOAuthClients => {
                    expanded.push(p.clone());
                    for p in user.permissions().iter() {
                        match p {
                            ApiPermission::DeleteOAuthClient(id) => {
                                expanded.push(ApiPermission::DeleteOAuthClient(*id))
                            }
                            _ => (),
                        }
                    }
                }

                ApiPermission::GetApiUserSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::GetApiUser(*user.id()))
                }
                ApiPermission::CreateApiUserTokenSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::CreateApiUserToken(*user.id()))
                }
                ApiPermission::GetApiUserTokenSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::GetApiUserToken(*user.id()))
                }
                ApiPermission::DeleteApiUserTokenSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::DeleteApiUserToken(*user.id()))
                }
                ApiPermission::UpdateApiUserSelf => {
                    expanded.push(p.clone());
                    expanded.push(ApiPermission::UpdateApiUser(*user.id()))
                }

                other => expanded.push(other.clone()),
            }
        }

        expanded.into()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
pub enum ApiPermission {
    // User information permissions
    CreateApiUserToken(Uuid),
    CreateApiUserTokenSelf,
    CreateApiUserTokenAll,
    GetApiUser(Uuid),
    GetApiUserSelf,
    GetApiUserAll,
    GetApiUserToken(Uuid),
    GetApiUserTokenSelf,
    DeleteApiUserToken(Uuid),
    DeleteApiUserTokenSelf,
    DeleteApiUserTokenAll,
    CreateApiUser,
    UpdateApiUser(Uuid),
    UpdateApiUserSelf,
    UpdateApiUserAll,

    // RFD access permissions
    GetRfd(i32),
    GetRfds(BTreeSet<i32>),
    GetAllRfds,
    GetAssignedRfds,
    GetDiscussion(i32),
    GetDiscussions(BTreeSet<i32>),
    GetAllDiscussions,
    GetAssignedDiscussions,
    SearchRfds,

    // OAuth client manage permissions
    CreateOAuthClient,
    GetOAuthClient(Uuid),
    GetOAuthClients(BTreeSet<Uuid>),
    GetAssignedOAuthClients,
    UpdateOAuthClient(Uuid),
    UpdateOAuthClients(BTreeSet<Uuid>),
    UpdateAssignedOAuthClients,
    DeleteOAuthClient(Uuid),
    DeleteOAuthClients(BTreeSet<Uuid>),
    DeleteAssignedOAuthClients,
}
