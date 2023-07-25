use rfd_model::permissions::Permissions;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use uuid::Uuid;

use crate::User;

pub trait ExpandPermission {
    fn expand(self, user: &User) -> Self;
}

impl ExpandPermission for Permissions<ApiPermission> {
    fn expand(self, user: &User) -> Self {
        self.into_iter()
            .map(|p| p.expand(user))
            .collect::<Vec<_>>()
            .into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum ApiPermission {
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

    GetRfd(BTreeSet<i32>),
    GetAllRfds,
    GetAssignedRfds,
}

impl ApiPermission {
    pub fn get_rfd(rfd_number: i32) -> ApiPermission {
        let mut set = BTreeSet::new();
        set.insert(rfd_number);

        ApiPermission::GetRfd(set)
    }
}

// Expand a meta permission into the full individual permissions assigned to the given user
impl ExpandPermission for ApiPermission {
    fn expand(self, user: &User) -> Self {
        match self {
            Self::CreateApiUserTokenSelf => Self::CreateApiUserToken(user.id),
            Self::GetApiUserSelf => Self::GetApiUser(user.id),
            Self::GetApiUserTokenSelf => Self::GetApiUserToken(user.id),
            Self::DeleteApiUserTokenSelf => Self::DeleteApiUserToken(user.id),
            Self::UpdateApiUserSelf => Self::UpdateApiUser(user.id),

            Self::GetRfd(_) => self,
            Self::GetAllRfds => self,
            Self::GetAssignedRfds => {
                let id_permissions = user
                    .permissions
                    .inner()
                    .iter()
                    .filter_map(|p| match p {
                        ApiPermission::GetRfd(ids) => Some(ids),
                        _ => None,
                    })
                    .flatten()
                    .copied()
                    .collect::<BTreeSet<i32>>();

                Self::GetRfd(id_permissions)
            }

            other => other,
        }
    }
}
