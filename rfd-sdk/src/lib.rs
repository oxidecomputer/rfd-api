// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod generated;

use std::fmt::Display;

use generated::sdk::types::ApiPermissionResponse;
pub use generated::sdk::*;
pub use progenitor_client::Error as ProgenitorClientError;

impl Display for ApiPermissionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateApiUserToken(id) => write!(f, "create-token:{}", id),
            Self::CreateApiUserTokenSelf => write!(f, "create-token-self"),
            Self::CreateApiUserTokenAssigned => write!(f, "create-token-assigned"),
            Self::CreateApiUserTokenAll => write!(f, "create-token-all"),
            Self::GetApiUser(id) => write!(f, "get-user:{}", id),
            Self::GetApiUserSelf => write!(f, "get-user-self"),
            Self::GetApiUserAssigned => write!(f, "get-user-assigned"),
            Self::GetApiUserAll => write!(f, "get-user-all"),
            Self::GetApiUserToken(id) => write!(f, "get-token:{}", id),
            Self::GetApiUserTokenSelf => write!(f, "get-token-self"),
            Self::GetApiUserTokenAssigned => write!(f, "get-token-assigned"),
            Self::GetApiUserTokenAll => write!(f, "get-token-all"),
            Self::DeleteApiUserToken(id) => write!(f, "delete-token:{}", id),
            Self::DeleteApiUserTokenSelf => write!(f, "delete-token-self"),
            Self::DeleteApiUserTokenAssigned => write!(f, "delete-token-assigned"),
            Self::DeleteApiUserTokenAll => write!(f, "delete-token-all"),
            Self::CreateApiUser => write!(f, "create-user"),
            Self::UpdateApiUser(id) => write!(f, "update-user:{}", id),
            Self::UpdateApiUserSelf => write!(f, "update-user-self"),
            Self::UpdateApiUserAssigned => write!(f, "update-user-assigned"),
            Self::UpdateApiUserAll => write!(f, "update-user-all"),
            Self::CreateUserApiProviderLinkToken => write!(f, "create-link-request"),
            Self::GetGroupsJoined => write!(f, "get-groups-joined"),
            Self::GetGroupsAll => write!(f, "get-groups-all"),
            Self::CreateGroup => write!(f, "create-group"),
            Self::UpdateGroup(id) => write!(f, "update-group:{}", id),
            Self::AddToGroup(id) => write!(f, "add-group-membership:{}", id),
            Self::RemoveFromGroup(id) => write!(f, "remove-group-membership:{}", id),
            Self::ManageGroupMembership(id) => write!(f, "manage-group-membership:{}", id),
            Self::ManageGroupMemberships(ids) => write!(
                f,
                "manage-group-memberships:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageGroupMembershipAssigned => write!(f, "manage-group-membership-assigned"),
            Self::ManageGroupMembershipAll => write!(f, "manage-group-membership-all"),
            Self::DeleteGroup(id) => write!(f, "delete-group:{}", id),
            Self::ManageGroup(id) => write!(f, "manage-group:{}", id),
            Self::ManageGroups(ids) => write!(
                f,
                "manage-groups:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageGroupsAssigned => write!(f, "manage-groups-assigned"),
            Self::ManageGroupsAll => write!(f, "manage-groups-all"),
            Self::ListMappers => write!(f, "list-mappers"),
            Self::CreateMapper => write!(f, "create-mapper"),
            Self::UpdateMapper(id) => write!(f, "update-mapper:{}", id),
            Self::DeleteMapper(id) => write!(f, "delete-mapper:{}", id),
            Self::ManageMapper(id) => write!(f, "manage-mapper:{}", id),
            Self::ManageMappers(ids) => write!(
                f,
                "manage-mappers:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageMappersAssigned => write!(f, "manage-mappers-assigned"),
            Self::ManageMappersAll => write!(f, "manage-mappers-all"),
            Self::GetRfd(number) => write!(f, "get-rfd:{}", number),
            Self::GetRfds(numbers) => write!(
                f,
                "get-rfds:{}",
                numbers
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::GetRfdsAssigned => write!(f, "get-rfds-assigned"),
            Self::GetRfdsAll => write!(f, "get-rfds-all"),
            Self::CreateRfd => write!(f, "create-rfd"),
            Self::UpdateRfd(number) => write!(f, "update-rfd:{}", number),
            Self::UpdateRfds(numbers) => write!(
                f,
                "update-rfds:{}",
                numbers
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::UpdateRfdsAssigned => write!(f, "update-rfds-assigned"),
            Self::UpdateRfdsAll => write!(f, "update-rfds-all"),
            Self::ManageRfdVisibility(number) => write!(f, "manage-rfd-vis:{}", number),
            Self::ManageRfdsVisibility(numbers) => write!(
                f,
                "manage-rfds-vis:{}",
                numbers
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageRfdsVisibilityAssigned => write!(f, "manage-rfd-vis-assigned"),
            Self::ManageRfdsVisibilityAll => write!(f, "manage-rfd-vis-all"),
            Self::GetDiscussion(number) => write!(f, "get-discussion:{}", number),
            Self::GetDiscussions(numbers) => write!(
                f,
                "get-discussions:{}",
                numbers
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::GetDiscussionsAssigned => write!(f, "get-discussions-assigned"),
            Self::GetDiscussionsAll => write!(f, "get-discussions-all"),
            Self::SearchRfds => write!(f, "search-rfds"),
            Self::CreateOAuthClient => write!(f, "create-oauth-client"),
            Self::GetOAuthClient(id) => write!(f, "get-oauth-client:{}", id),
            Self::GetOAuthClients(ids) => write!(
                f,
                "get-oauth-clients:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::GetOAuthClientsAssigned => write!(f, "get-oauth-clients-assigned"),
            Self::GetOAuthClientsAll => write!(f, "get-oauth-clients-all"),
            Self::UpdateOAuthClient(id) => write!(f, "update-oauth-client:{}", id),
            Self::UpdateOAuthClients(ids) => write!(
                f,
                "update-oauth-clients:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::UpdateOAuthClientsAssigned => write!(f, "update-oauth-clients-assigned"),
            Self::UpdateOAuthClientsAll => write!(f, "update-oauth-clients-all"),
            Self::DeleteOAuthClient(id) => write!(f, "delete-oauth-client:{}", id),
            Self::DeleteOAuthClients(ids) => write!(
                f,
                "delete-oauth-clients:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::DeleteOAuthClientsAssigned => write!(f, "delete-oauth-clients-assigned"),
            Self::DeleteOAuthClientsAll => write!(f, "delete-oauth-clients-self"),
            Self::CreateAccessToken => Ok(()),
            Self::Removed => Ok(()),
        }
    }
}
