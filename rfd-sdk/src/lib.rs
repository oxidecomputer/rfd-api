// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[allow(clippy::all)]
mod generated;

use std::fmt::Display;

pub use generated::sdk::*;
pub use progenitor_client::Error as ProgenitorClientError;
use types::RfdPermission;

impl Display for RfdPermission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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

            Self::CreateApiUser => write!(f, "create-user"),
            Self::GetApiUser(id) => write!(f, "get-user:{}", **id),
            Self::GetApiUserSelf => write!(f, "get-user-self"),
            Self::GetApiUsers(ids) => write!(
                f,
                "get-user:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::GetApiUsersAssigned => write!(f, "get-user-assigned"),
            Self::GetApiUsersAll => write!(f, "get-user-all"),
            Self::ManageApiUser(id) => write!(f, "manage-user:{}", **id),
            Self::ManageApiUsers(ids) => write!(
                f,
                "manage-user:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageApiUsersAssigned => write!(f, "manage-user-assigned"),
            Self::ManageApiUsersAll => write!(f, "manage-user-all"),
            Self::CreateApiKey(id) => write!(f, "create-api-key:{}", **id),
            Self::CreateApiKeySelf => write!(f, "create-api-key-self"),
            Self::CreateApiKeyAssigned => write!(f, "create-api-key-assigned"),
            Self::CreateApiKeyAll => write!(f, "create-api-key-all"),
            Self::GetApiKey(id) => write!(f, "get-api-key:{}", **id),
            Self::GetApiKeys(ids) => write!(
                f,
                "get-api-key:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::GetApiKeysAssigned => write!(f, "get-api-key-assigned"),
            Self::GetApiKeysAll => write!(f, "get-api=key-all"),
            Self::ManageApiKey(id) => write!(f, "manage-api-key:{}", **id),
            Self::ManageApiKeys(ids) => write!(
                f,
                "manage-api-key:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageApiKeysAssigned => write!(f, "manage-api-key-assigned"),
            Self::ManageApiKeysAll => write!(f, "manage-api-key-all"),
            Self::CreateUserApiProviderLinkToken => write!(f, "create-link-request"),
            Self::GetGroup(id) => write!(f, "get-group:{}", **id),
            Self::CreateGroup => write!(f, "create-group"),
            Self::GetGroupsJoined => write!(f, "get-groups-joined"),
            Self::GetGroupsAll => write!(f, "get-group-all"),
            Self::ManageGroup(id) => write!(f, "manage-group:{}", **id),
            Self::ManageGroups(ids) => write!(
                f,
                "manage-group:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageGroupsAssigned => write!(f, "manage-group-assigned"),
            Self::ManageGroupsAll => write!(f, "manage-group-all"),
            Self::ManageGroupMembership(id) => write!(f, "manage-group-membership:{}", **id),
            Self::ManageGroupMemberships(ids) => write!(
                f,
                "manage-group-membership:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageGroupMembershipsAssigned => write!(f, "manage-group-membership-assigned"),
            Self::ManageGroupMembershipsAll => write!(f, "manage-group-membership-all"),
            Self::CreateMapper => write!(f, "create-mapper"),
            Self::GetMappersAll => write!(f, "get-mapper-all"),
            Self::ManageMapper(id) => write!(f, "manage-mapper:{}", **id),
            Self::ManageMappers(ids) => write!(
                f,
                "manage-mapper:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageMappersAssigned => write!(f, "manage-mapper-assigned"),
            Self::ManageMappersAll => write!(f, "manage-mapper-all"),
            Self::CreateOAuthClient => write!(f, "create-oauth-client"),
            Self::GetOAuthClient(id) => write!(f, "get-oauth-client:{}", **id),
            Self::GetOAuthClients(ids) => write!(
                f,
                "get-oauth-client:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::GetOAuthClientsAssigned => write!(f, "get-oauth-client-assigned"),
            Self::GetOAuthClientsAll => write!(f, "manage-oauth-client-all"),
            Self::ManageOAuthClient(id) => write!(f, "manage-oauth-client:{}", **id),
            Self::ManageOAuthClients(ids) => write!(
                f,
                "manage-oauth-client:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageOAuthClientsAssigned => write!(f, "manage-oauth-client-assigned"),
            Self::ManageOAuthClientsAll => write!(f, "manage-oauth-client-all"),

            Self::CreateMagicLinkClient => write!(f, "create-magic-link"),
            Self::GetMagicLinkClient(id) => write!(f, "get-magic-link:{}", **id),
            Self::GetMagicLinkClients(ids) => write!(
                f,
                "get-magic-link:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::GetMagicLinkClientsAssigned => write!(f, "get-magic-link-assigned"),
            Self::GetMagicLinkClientsAll => write!(f, "manage-magic-link-all"),
            Self::ManageMagicLinkClient(id) => write!(f, "manage-magic-link:{}", **id),
            Self::ManageMagicLinkClients(ids) => write!(
                f,
                "manage-magic-link:{}",
                ids.iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::ManageMagicLinkClientsAssigned => write!(f, "manage-magic-link-assigned"),
            Self::ManageMagicLinkClientsAll => write!(f, "manage-magic-link-all"),

            Self::CreateAccessToken => Ok(()),
            Self::Unsupported(deprecated) => write!(f, "deprecated-permission:{:?}", deprecated),
        }
    }
}
