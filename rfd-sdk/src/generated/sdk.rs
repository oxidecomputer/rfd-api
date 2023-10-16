// The contents of this file are generated; do not modify them.

#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct AccessGroupForApiPermission {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub id: uuid::Uuid,
        pub name: String,
        pub permissions: PermissionsForApiPermission,
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&AccessGroupForApiPermission> for AccessGroupForApiPermission {
        fn from(value: &AccessGroupForApiPermission) -> Self {
            value.clone()
        }
    }

    impl AccessGroupForApiPermission {
        pub fn builder() -> builder::AccessGroupForApiPermission {
            builder::AccessGroupForApiPermission::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct AccessGroupUpdateParams {
        pub name: String,
        pub permissions: PermissionsForApiPermission,
    }

    impl From<&AccessGroupUpdateParams> for AccessGroupUpdateParams {
        fn from(value: &AccessGroupUpdateParams) -> Self {
            value.clone()
        }
    }

    impl AccessGroupUpdateParams {
        pub fn builder() -> builder::AccessGroupUpdateParams {
            builder::AccessGroupUpdateParams::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct AccessTokenExchangeRequest {
        pub device_code: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expires_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub grant_type: String,
    }

    impl From<&AccessTokenExchangeRequest> for AccessTokenExchangeRequest {
        fn from(value: &AccessTokenExchangeRequest) -> Self {
            value.clone()
        }
    }

    impl AccessTokenExchangeRequest {
        pub fn builder() -> builder::AccessTokenExchangeRequest {
            builder::AccessTokenExchangeRequest::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct AddGroupBody {
        pub group_id: uuid::Uuid,
    }

    impl From<&AddGroupBody> for AddGroupBody {
        fn from(value: &AddGroupBody) -> Self {
            value.clone()
        }
    }

    impl AddGroupBody {
        pub fn builder() -> builder::AddGroupBody {
            builder::AddGroupBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct AddOAuthClientRedirectBody {
        pub redirect_uri: String,
    }

    impl From<&AddOAuthClientRedirectBody> for AddOAuthClientRedirectBody {
        fn from(value: &AddOAuthClientRedirectBody) -> Self {
            value.clone()
        }
    }

    impl AddOAuthClientRedirectBody {
        pub fn builder() -> builder::AddOAuthClientRedirectBody {
            builder::AddOAuthClientRedirectBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiKeyCreateParams {
        pub expires_at: chrono::DateTime<chrono::offset::Utc>,
        pub permissions: PermissionsForApiPermission,
    }

    impl From<&ApiKeyCreateParams> for ApiKeyCreateParams {
        fn from(value: &ApiKeyCreateParams) -> Self {
            value.clone()
        }
    }

    impl ApiKeyCreateParams {
        pub fn builder() -> builder::ApiKeyCreateParams {
            builder::ApiKeyCreateParams::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiKeyResponse {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        pub id: uuid::Uuid,
        pub permissions: PermissionsForApiPermission,
    }

    impl From<&ApiKeyResponse> for ApiKeyResponse {
        fn from(value: &ApiKeyResponse) -> Self {
            value.clone()
        }
    }

    impl ApiKeyResponse {
        pub fn builder() -> builder::ApiKeyResponse {
            builder::ApiKeyResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub enum ApiPermission {
        CreateApiUserTokenSelf,
        CreateApiUserTokenAssigned,
        CreateApiUserTokenAll,
        GetApiUserSelf,
        GetApiUserAssigned,
        GetApiUserAll,
        GetApiUserTokenSelf,
        GetApiUserTokenAssigned,
        GetApiUserTokenAll,
        DeleteApiUserTokenSelf,
        DeleteApiUserTokenAssigned,
        DeleteApiUserTokenAll,
        CreateApiUser,
        UpdateApiUserSelf,
        UpdateApiUserAssigned,
        UpdateApiUserAll,
        CreateUserApiProviderLinkToken,
        ListGroups,
        CreateGroup,
        ManageGroupMembershipAssigned,
        ManageGroupMembershipAll,
        ManageGroupsAssigned,
        ManageGroupsAll,
        ListMappers,
        CreateMapper,
        ManageMappersAssigned,
        ManageMappersAll,
        GetRfdsAssigned,
        GetRfdsAll,
        GetDiscussionsAssigned,
        GetDiscussionsAll,
        SearchRfds,
        CreateOAuthClient,
        GetOAuthClientsAssigned,
        GetOAuthClientsAll,
        UpdateOAuthClientsAssigned,
        UpdateOAuthClientsAll,
        DeleteOAuthClientsAssigned,
        DeleteOAuthClientsAll,
        CreateApiUserToken(uuid::Uuid),
        GetApiUser(uuid::Uuid),
        GetApiUserToken(uuid::Uuid),
        DeleteApiUserToken(uuid::Uuid),
        UpdateApiUser(uuid::Uuid),
        UpdateGroup(uuid::Uuid),
        AddToGroup(uuid::Uuid),
        RemoveFromGroup(uuid::Uuid),
        ManageGroupMembership(uuid::Uuid),
        ManageGroupMemberships(Vec<uuid::Uuid>),
        DeleteGroup(uuid::Uuid),
        ManageGroup(uuid::Uuid),
        ManageGroups(Vec<uuid::Uuid>),
        UpdateMapper(uuid::Uuid),
        DeleteMapper(uuid::Uuid),
        ManageMapper(uuid::Uuid),
        ManageMappers(Vec<uuid::Uuid>),
        GetRfd(i32),
        GetRfds(Vec<i32>),
        GetDiscussion(i32),
        GetDiscussions(Vec<i32>),
        GetOAuthClient(uuid::Uuid),
        GetOAuthClients(Vec<uuid::Uuid>),
        UpdateOAuthClient(uuid::Uuid),
        UpdateOAuthClients(Vec<uuid::Uuid>),
        DeleteOAuthClient(uuid::Uuid),
        DeleteOAuthClients(Vec<uuid::Uuid>),
    }

    impl From<&ApiPermission> for ApiPermission {
        fn from(value: &ApiPermission) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserForApiPermission {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub groups: Vec<uuid::Uuid>,
        pub id: uuid::Uuid,
        pub permissions: PermissionsForApiPermission,
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&ApiUserForApiPermission> for ApiUserForApiPermission {
        fn from(value: &ApiUserForApiPermission) -> Self {
            value.clone()
        }
    }

    impl ApiUserForApiPermission {
        pub fn builder() -> builder::ApiUserForApiPermission {
            builder::ApiUserForApiPermission::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserLinkRequestPayload {
        pub user_identifier: uuid::Uuid,
    }

    impl From<&ApiUserLinkRequestPayload> for ApiUserLinkRequestPayload {
        fn from(value: &ApiUserLinkRequestPayload) -> Self {
            value.clone()
        }
    }

    impl ApiUserLinkRequestPayload {
        pub fn builder() -> builder::ApiUserLinkRequestPayload {
            builder::ApiUserLinkRequestPayload::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserLinkRequestResponse {
        pub token: String,
    }

    impl From<&ApiUserLinkRequestResponse> for ApiUserLinkRequestResponse {
        fn from(value: &ApiUserLinkRequestResponse) -> Self {
            value.clone()
        }
    }

    impl ApiUserLinkRequestResponse {
        pub fn builder() -> builder::ApiUserLinkRequestResponse {
            builder::ApiUserLinkRequestResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserProvider {
        pub api_user_id: uuid::Uuid,
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub emails: Vec<String>,
        pub id: uuid::Uuid,
        pub provider: String,
        pub provider_id: String,
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&ApiUserProvider> for ApiUserProvider {
        fn from(value: &ApiUserProvider) -> Self {
            value.clone()
        }
    }

    impl ApiUserProvider {
        pub fn builder() -> builder::ApiUserProvider {
            builder::ApiUserProvider::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserProviderLinkPayload {
        pub token: String,
    }

    impl From<&ApiUserProviderLinkPayload> for ApiUserProviderLinkPayload {
        fn from(value: &ApiUserProviderLinkPayload) -> Self {
            value.clone()
        }
    }

    impl ApiUserProviderLinkPayload {
        pub fn builder() -> builder::ApiUserProviderLinkPayload {
            builder::ApiUserProviderLinkPayload::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserUpdateParams {
        pub groups: Vec<uuid::Uuid>,
        pub permissions: PermissionsForApiPermission,
    }

    impl From<&ApiUserUpdateParams> for ApiUserUpdateParams {
        fn from(value: &ApiUserUpdateParams) -> Self {
            value.clone()
        }
    }

    impl ApiUserUpdateParams {
        pub fn builder() -> builder::ApiUserUpdateParams {
            builder::ApiUserUpdateParams::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct CreateMapper {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_activations: Option<i32>,
        pub name: String,
        pub rule: MappingRules,
    }

    impl From<&CreateMapper> for CreateMapper {
        fn from(value: &CreateMapper) -> Self {
            value.clone()
        }
    }

    impl CreateMapper {
        pub fn builder() -> builder::CreateMapper {
            builder::CreateMapper::default()
        }
    }

    /// Error information from a response.
    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct Error {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error_code: Option<String>,
        pub message: String,
        pub request_id: String,
    }

    impl From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }

    impl Error {
        pub fn builder() -> builder::Error {
            builder::Error::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct FullRfd {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authors: Option<String>,
        pub commit: String,
        pub committed_at: chrono::DateTime<chrono::offset::Utc>,
        pub content: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discussion: Option<String>,
        pub id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        pub pdfs: Vec<FullRfdPdfEntry>,
        pub rfd_number: i32,
        pub sha: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        pub title: String,
        pub visibility: Visibility,
    }

    impl From<&FullRfd> for FullRfd {
        fn from(value: &FullRfd) -> Self {
            value.clone()
        }
    }

    impl FullRfd {
        pub fn builder() -> builder::FullRfd {
            builder::FullRfd::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct FullRfdPdfEntry {
        pub link: String,
        pub source: String,
    }

    impl From<&FullRfdPdfEntry> for FullRfdPdfEntry {
        fn from(value: &FullRfdPdfEntry) -> Self {
            value.clone()
        }
    }

    impl FullRfdPdfEntry {
        pub fn builder() -> builder::FullRfdPdfEntry {
            builder::FullRfdPdfEntry::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct GetApiUserResponse {
        pub info: ApiUserForApiPermission,
        pub providers: Vec<ApiUserProvider>,
    }

    impl From<&GetApiUserResponse> for GetApiUserResponse {
        fn from(value: &GetApiUserResponse) -> Self {
            value.clone()
        }
    }

    impl GetApiUserResponse {
        pub fn builder() -> builder::GetApiUserResponse {
            builder::GetApiUserResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct GitHubCommit {
        pub added: Vec<String>,
        pub id: String,
        pub modified: Vec<String>,
        pub removed: Vec<String>,
        pub timestamp: chrono::DateTime<chrono::offset::Utc>,
    }

    impl From<&GitHubCommit> for GitHubCommit {
        fn from(value: &GitHubCommit) -> Self {
            value.clone()
        }
    }

    impl GitHubCommit {
        pub fn builder() -> builder::GitHubCommit {
            builder::GitHubCommit::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct GitHubCommitPayload {
        pub commits: Vec<GitHubCommit>,
        pub head_commit: GitHubCommit,
        pub installation: GitHubInstallation,
        pub ref_: String,
        pub repository: GitHubRepository,
        pub sender: GitHubSender,
    }

    impl From<&GitHubCommitPayload> for GitHubCommitPayload {
        fn from(value: &GitHubCommitPayload) -> Self {
            value.clone()
        }
    }

    impl GitHubCommitPayload {
        pub fn builder() -> builder::GitHubCommitPayload {
            builder::GitHubCommitPayload::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct GitHubInstallation {
        pub id: u64,
        pub node_id: String,
    }

    impl From<&GitHubInstallation> for GitHubInstallation {
        fn from(value: &GitHubInstallation) -> Self {
            value.clone()
        }
    }

    impl GitHubInstallation {
        pub fn builder() -> builder::GitHubInstallation {
            builder::GitHubInstallation::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct GitHubRepository {
        pub default_branch: String,
        pub id: u64,
        pub name: String,
        pub node_id: String,
        pub owner: GitHubRepositoryOwner,
    }

    impl From<&GitHubRepository> for GitHubRepository {
        fn from(value: &GitHubRepository) -> Self {
            value.clone()
        }
    }

    impl GitHubRepository {
        pub fn builder() -> builder::GitHubRepository {
            builder::GitHubRepository::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct GitHubRepositoryOwner {
        pub login: String,
    }

    impl From<&GitHubRepositoryOwner> for GitHubRepositoryOwner {
        fn from(value: &GitHubRepositoryOwner) -> Self {
            value.clone()
        }
    }

    impl GitHubRepositoryOwner {
        pub fn builder() -> builder::GitHubRepositoryOwner {
            builder::GitHubRepositoryOwner::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct GitHubSender {
        pub id: u64,
        pub login: String,
        pub node_id: String,
        #[serde(rename = "type")]
        pub type_: String,
    }

    impl From<&GitHubSender> for GitHubSender {
        fn from(value: &GitHubSender) -> Self {
            value.clone()
        }
    }

    impl GitHubSender {
        pub fn builder() -> builder::GitHubSender {
            builder::GitHubSender::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct InitialApiKeyResponse {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        pub id: uuid::Uuid,
        pub key: String,
        pub permissions: PermissionsForApiPermission,
    }

    impl From<&InitialApiKeyResponse> for InitialApiKeyResponse {
        fn from(value: &InitialApiKeyResponse) -> Self {
            value.clone()
        }
    }

    impl InitialApiKeyResponse {
        pub fn builder() -> builder::InitialApiKeyResponse {
            builder::InitialApiKeyResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct InitialOAuthClientSecretResponse {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        pub id: uuid::Uuid,
        pub key: String,
    }

    impl From<&InitialOAuthClientSecretResponse> for InitialOAuthClientSecretResponse {
        fn from(value: &InitialOAuthClientSecretResponse) -> Self {
            value.clone()
        }
    }

    impl InitialOAuthClientSecretResponse {
        pub fn builder() -> builder::InitialOAuthClientSecretResponse {
            builder::InitialOAuthClientSecretResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct Jwk {
        pub e: String,
        pub kid: String,
        pub kty: String,
        pub n: String,
        #[serde(rename = "use")]
        pub use_: String,
    }

    impl From<&Jwk> for Jwk {
        fn from(value: &Jwk) -> Self {
            value.clone()
        }
    }

    impl Jwk {
        pub fn builder() -> builder::Jwk {
            builder::Jwk::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct Jwks {
        pub keys: Vec<Jwk>,
    }

    impl From<&Jwks> for Jwks {
        fn from(value: &Jwks) -> Self {
            value.clone()
        }
    }

    impl Jwks {
        pub fn builder() -> builder::Jwks {
            builder::Jwks::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ListRfd {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authors: Option<String>,
        pub commit: String,
        pub committed_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub discussion: Option<String>,
        pub id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        pub rfd_number: i32,
        pub sha: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        pub title: String,
        pub visibility: Visibility,
    }

    impl From<&ListRfd> for ListRfd {
        fn from(value: &ListRfd) -> Self {
            value.clone()
        }
    }

    impl ListRfd {
        pub fn builder() -> builder::ListRfd {
            builder::ListRfd::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct Mapper {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub activations: Option<i32>,
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub depleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub id: uuid::Uuid,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub max_activations: Option<i32>,
        pub name: String,
        pub rule: serde_json::Value,
    }

    impl From<&Mapper> for Mapper {
        fn from(value: &Mapper) -> Self {
            value.clone()
        }
    }

    impl Mapper {
        pub fn builder() -> builder::Mapper {
            builder::Mapper::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    #[serde(tag = "rule")]
    pub enum MappingRules {
        #[serde(rename = "email_address")]
        EmailAddress {
            email: String,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            groups: Vec<String>,
            #[serde(default = "defaults::mapping_rules_email_address_permissions")]
            permissions: PermissionsForApiPermission,
        },
        #[serde(rename = "email_domain")]
        EmailDomain {
            domain: String,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            groups: Vec<String>,
            #[serde(default = "defaults::mapping_rules_email_domain_permissions")]
            permissions: PermissionsForApiPermission,
        },
        #[serde(rename = "git_hub_username")]
        GitHubUsername {
            github_username: String,
            #[serde(default, skip_serializing_if = "Vec::is_empty")]
            groups: Vec<String>,
            #[serde(default = "defaults::mapping_rules_git_hub_username_permissions")]
            permissions: PermissionsForApiPermission,
        },
    }

    impl From<&MappingRules> for MappingRules {
        fn from(value: &MappingRules) -> Self {
            value.clone()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct OAuthAuthzCodeExchangeBody {
        pub client_id: uuid::Uuid,
        pub client_secret: String,
        pub code: String,
        pub grant_type: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub pkce_verifier: Option<String>,
        pub redirect_uri: String,
    }

    impl From<&OAuthAuthzCodeExchangeBody> for OAuthAuthzCodeExchangeBody {
        fn from(value: &OAuthAuthzCodeExchangeBody) -> Self {
            value.clone()
        }
    }

    impl OAuthAuthzCodeExchangeBody {
        pub fn builder() -> builder::OAuthAuthzCodeExchangeBody {
            builder::OAuthAuthzCodeExchangeBody::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct OAuthAuthzCodeExchangeResponse {
        pub access_token: String,
        pub expires_in: i64,
        pub token_type: String,
    }

    impl From<&OAuthAuthzCodeExchangeResponse> for OAuthAuthzCodeExchangeResponse {
        fn from(value: &OAuthAuthzCodeExchangeResponse) -> Self {
            value.clone()
        }
    }

    impl OAuthAuthzCodeExchangeResponse {
        pub fn builder() -> builder::OAuthAuthzCodeExchangeResponse {
            builder::OAuthAuthzCodeExchangeResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct OAuthClient {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub id: uuid::Uuid,
        pub redirect_uris: Vec<OAuthClientRedirectUri>,
        pub secrets: Vec<OAuthClientSecret>,
    }

    impl From<&OAuthClient> for OAuthClient {
        fn from(value: &OAuthClient) -> Self {
            value.clone()
        }
    }

    impl OAuthClient {
        pub fn builder() -> builder::OAuthClient {
            builder::OAuthClient::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct OAuthClientRedirectUri {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub id: uuid::Uuid,
        pub oauth_client_id: uuid::Uuid,
        pub redirect_uri: String,
    }

    impl From<&OAuthClientRedirectUri> for OAuthClientRedirectUri {
        fn from(value: &OAuthClientRedirectUri) -> Self {
            value.clone()
        }
    }

    impl OAuthClientRedirectUri {
        pub fn builder() -> builder::OAuthClientRedirectUri {
            builder::OAuthClientRedirectUri::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct OAuthClientSecret {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub id: uuid::Uuid,
        pub oauth_client_id: uuid::Uuid,
        pub secret_signature: String,
    }

    impl From<&OAuthClientSecret> for OAuthClientSecret {
        fn from(value: &OAuthClientSecret) -> Self {
            value.clone()
        }
    }

    impl OAuthClientSecret {
        pub fn builder() -> builder::OAuthClientSecret {
            builder::OAuthClientSecret::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct OAuthProviderInfo {
        pub auth_url_endpoint: String,
        pub client_id: String,
        pub device_code_endpoint: String,
        pub provider: OAuthProviderName,
        pub scopes: Vec<String>,
        pub token_endpoint: String,
    }

    impl From<&OAuthProviderInfo> for OAuthProviderInfo {
        fn from(value: &OAuthProviderInfo) -> Self {
            value.clone()
        }
    }

    impl OAuthProviderInfo {
        pub fn builder() -> builder::OAuthProviderInfo {
            builder::OAuthProviderInfo::default()
        }
    }

    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum OAuthProviderName {
        #[serde(rename = "github")]
        Github,
        #[serde(rename = "google")]
        Google,
    }

    impl From<&OAuthProviderName> for OAuthProviderName {
        fn from(value: &OAuthProviderName) -> Self {
            value.clone()
        }
    }

    impl ToString for OAuthProviderName {
        fn to_string(&self) -> String {
            match *self {
                Self::Github => "github".to_string(),
                Self::Google => "google".to_string(),
            }
        }
    }

    impl std::str::FromStr for OAuthProviderName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "github" => Ok(Self::Github),
                "google" => Ok(Self::Google),
                _ => Err("invalid value"),
            }
        }
    }

    impl std::convert::TryFrom<&str> for OAuthProviderName {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for OAuthProviderName {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for OAuthProviderName {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct OpenIdConfiguration {
        pub jwks_uri: String,
    }

    impl From<&OpenIdConfiguration> for OpenIdConfiguration {
        fn from(value: &OpenIdConfiguration) -> Self {
            value.clone()
        }
    }

    impl OpenIdConfiguration {
        pub fn builder() -> builder::OpenIdConfiguration {
            builder::OpenIdConfiguration::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct PermissionsForApiPermission(pub Vec<ApiPermission>);
    impl std::ops::Deref for PermissionsForApiPermission {
        type Target = Vec<ApiPermission>;
        fn deref(&self) -> &Vec<ApiPermission> {
            &self.0
        }
    }

    impl From<PermissionsForApiPermission> for Vec<ApiPermission> {
        fn from(value: PermissionsForApiPermission) -> Self {
            value.0
        }
    }

    impl From<&PermissionsForApiPermission> for PermissionsForApiPermission {
        fn from(value: &PermissionsForApiPermission) -> Self {
            value.clone()
        }
    }

    impl From<Vec<ApiPermission>> for PermissionsForApiPermission {
        fn from(value: Vec<ApiPermission>) -> Self {
            Self(value)
        }
    }

    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize,
        schemars :: JsonSchema,
    )]
    pub enum Visibility {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
    }

    impl From<&Visibility> for Visibility {
        fn from(value: &Visibility) -> Self {
            value.clone()
        }
    }

    impl ToString for Visibility {
        fn to_string(&self) -> String {
            match *self {
                Self::Public => "public".to_string(),
                Self::Private => "private".to_string(),
            }
        }
    }

    impl std::str::FromStr for Visibility {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "public" => Ok(Self::Public),
                "private" => Ok(Self::Private),
                _ => Err("invalid value"),
            }
        }
    }

    impl std::convert::TryFrom<&str> for Visibility {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Visibility {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Visibility {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct AccessGroupForApiPermission {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            deleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            id: Result<uuid::Uuid, String>,
            name: Result<String, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
            updated_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for AccessGroupForApiPermission {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl AccessGroupForApiPermission {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PermissionsForApiPermission>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AccessGroupForApiPermission> for super::AccessGroupForApiPermission {
            type Error = String;
            fn try_from(value: AccessGroupForApiPermission) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    name: value.name?,
                    permissions: value.permissions?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl From<super::AccessGroupForApiPermission> for AccessGroupForApiPermission {
            fn from(value: super::AccessGroupForApiPermission) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    name: Ok(value.name),
                    permissions: Ok(value.permissions),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccessGroupUpdateParams {
            name: Result<String, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
        }

        impl Default for AccessGroupUpdateParams {
            fn default() -> Self {
                Self {
                    name: Err("no value supplied for name".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl AccessGroupUpdateParams {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PermissionsForApiPermission>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AccessGroupUpdateParams> for super::AccessGroupUpdateParams {
            type Error = String;
            fn try_from(value: AccessGroupUpdateParams) -> Result<Self, String> {
                Ok(Self {
                    name: value.name?,
                    permissions: value.permissions?,
                })
            }
        }

        impl From<super::AccessGroupUpdateParams> for AccessGroupUpdateParams {
            fn from(value: super::AccessGroupUpdateParams) -> Self {
                Self {
                    name: Ok(value.name),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccessTokenExchangeRequest {
            device_code: Result<String, String>,
            expires_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            grant_type: Result<String, String>,
        }

        impl Default for AccessTokenExchangeRequest {
            fn default() -> Self {
                Self {
                    device_code: Err("no value supplied for device_code".to_string()),
                    expires_at: Ok(Default::default()),
                    grant_type: Err("no value supplied for grant_type".to_string()),
                }
            }
        }

        impl AccessTokenExchangeRequest {
            pub fn device_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.device_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for device_code: {}", e));
                self
            }
            pub fn expires_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.expires_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_at: {}", e));
                self
            }
            pub fn grant_type<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.grant_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grant_type: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AccessTokenExchangeRequest> for super::AccessTokenExchangeRequest {
            type Error = String;
            fn try_from(value: AccessTokenExchangeRequest) -> Result<Self, String> {
                Ok(Self {
                    device_code: value.device_code?,
                    expires_at: value.expires_at?,
                    grant_type: value.grant_type?,
                })
            }
        }

        impl From<super::AccessTokenExchangeRequest> for AccessTokenExchangeRequest {
            fn from(value: super::AccessTokenExchangeRequest) -> Self {
                Self {
                    device_code: Ok(value.device_code),
                    expires_at: Ok(value.expires_at),
                    grant_type: Ok(value.grant_type),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddGroupBody {
            group_id: Result<uuid::Uuid, String>,
        }

        impl Default for AddGroupBody {
            fn default() -> Self {
                Self {
                    group_id: Err("no value supplied for group_id".to_string()),
                }
            }
        }

        impl AddGroupBody {
            pub fn group_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.group_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for group_id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<AddGroupBody> for super::AddGroupBody {
            type Error = String;
            fn try_from(value: AddGroupBody) -> Result<Self, String> {
                Ok(Self {
                    group_id: value.group_id?,
                })
            }
        }

        impl From<super::AddGroupBody> for AddGroupBody {
            fn from(value: super::AddGroupBody) -> Self {
                Self {
                    group_id: Ok(value.group_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddOAuthClientRedirectBody {
            redirect_uri: Result<String, String>,
        }

        impl Default for AddOAuthClientRedirectBody {
            fn default() -> Self {
                Self {
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl AddOAuthClientRedirectBody {
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<AddOAuthClientRedirectBody> for super::AddOAuthClientRedirectBody {
            type Error = String;
            fn try_from(value: AddOAuthClientRedirectBody) -> Result<Self, String> {
                Ok(Self {
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl From<super::AddOAuthClientRedirectBody> for AddOAuthClientRedirectBody {
            fn from(value: super::AddOAuthClientRedirectBody) -> Self {
                Self {
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiKeyCreateParams {
            expires_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
        }

        impl Default for ApiKeyCreateParams {
            fn default() -> Self {
                Self {
                    expires_at: Err("no value supplied for expires_at".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl ApiKeyCreateParams {
            pub fn expires_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.expires_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_at: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PermissionsForApiPermission>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ApiKeyCreateParams> for super::ApiKeyCreateParams {
            type Error = String;
            fn try_from(value: ApiKeyCreateParams) -> Result<Self, String> {
                Ok(Self {
                    expires_at: value.expires_at?,
                    permissions: value.permissions?,
                })
            }
        }

        impl From<super::ApiKeyCreateParams> for ApiKeyCreateParams {
            fn from(value: super::ApiKeyCreateParams) -> Self {
                Self {
                    expires_at: Ok(value.expires_at),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiKeyResponse {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            id: Result<uuid::Uuid, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
        }

        impl Default for ApiKeyResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl ApiKeyResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PermissionsForApiPermission>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ApiKeyResponse> for super::ApiKeyResponse {
            type Error = String;
            fn try_from(value: ApiKeyResponse) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    permissions: value.permissions?,
                })
            }
        }

        impl From<super::ApiKeyResponse> for ApiKeyResponse {
            fn from(value: super::ApiKeyResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserForApiPermission {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            deleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            groups: Result<Vec<uuid::Uuid>, String>,
            id: Result<uuid::Uuid, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
            updated_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for ApiUserForApiPermission {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    groups: Err("no value supplied for groups".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl ApiUserForApiPermission {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn groups<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<uuid::Uuid>>,
                T::Error: std::fmt::Display,
            {
                self.groups = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for groups: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PermissionsForApiPermission>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ApiUserForApiPermission> for super::ApiUserForApiPermission {
            type Error = String;
            fn try_from(value: ApiUserForApiPermission) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    groups: value.groups?,
                    id: value.id?,
                    permissions: value.permissions?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl From<super::ApiUserForApiPermission> for ApiUserForApiPermission {
            fn from(value: super::ApiUserForApiPermission) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    groups: Ok(value.groups),
                    id: Ok(value.id),
                    permissions: Ok(value.permissions),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserLinkRequestPayload {
            user_identifier: Result<uuid::Uuid, String>,
        }

        impl Default for ApiUserLinkRequestPayload {
            fn default() -> Self {
                Self {
                    user_identifier: Err("no value supplied for user_identifier".to_string()),
                }
            }
        }

        impl ApiUserLinkRequestPayload {
            pub fn user_identifier<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.user_identifier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for user_identifier: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<ApiUserLinkRequestPayload> for super::ApiUserLinkRequestPayload {
            type Error = String;
            fn try_from(value: ApiUserLinkRequestPayload) -> Result<Self, String> {
                Ok(Self {
                    user_identifier: value.user_identifier?,
                })
            }
        }

        impl From<super::ApiUserLinkRequestPayload> for ApiUserLinkRequestPayload {
            fn from(value: super::ApiUserLinkRequestPayload) -> Self {
                Self {
                    user_identifier: Ok(value.user_identifier),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserLinkRequestResponse {
            token: Result<String, String>,
        }

        impl Default for ApiUserLinkRequestResponse {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl ApiUserLinkRequestResponse {
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ApiUserLinkRequestResponse> for super::ApiUserLinkRequestResponse {
            type Error = String;
            fn try_from(value: ApiUserLinkRequestResponse) -> Result<Self, String> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }

        impl From<super::ApiUserLinkRequestResponse> for ApiUserLinkRequestResponse {
            fn from(value: super::ApiUserLinkRequestResponse) -> Self {
                Self {
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserProvider {
            api_user_id: Result<uuid::Uuid, String>,
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            deleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            emails: Result<Vec<String>, String>,
            id: Result<uuid::Uuid, String>,
            provider: Result<String, String>,
            provider_id: Result<String, String>,
            updated_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for ApiUserProvider {
            fn default() -> Self {
                Self {
                    api_user_id: Err("no value supplied for api_user_id".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    emails: Err("no value supplied for emails".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    provider: Err("no value supplied for provider".to_string()),
                    provider_id: Err("no value supplied for provider_id".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl ApiUserProvider {
            pub fn api_user_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.api_user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for api_user_id: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn emails<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.emails = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for emails: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn provider<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.provider = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider: {}", e));
                self
            }
            pub fn provider_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.provider_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider_id: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ApiUserProvider> for super::ApiUserProvider {
            type Error = String;
            fn try_from(value: ApiUserProvider) -> Result<Self, String> {
                Ok(Self {
                    api_user_id: value.api_user_id?,
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    emails: value.emails?,
                    id: value.id?,
                    provider: value.provider?,
                    provider_id: value.provider_id?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl From<super::ApiUserProvider> for ApiUserProvider {
            fn from(value: super::ApiUserProvider) -> Self {
                Self {
                    api_user_id: Ok(value.api_user_id),
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    emails: Ok(value.emails),
                    id: Ok(value.id),
                    provider: Ok(value.provider),
                    provider_id: Ok(value.provider_id),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserProviderLinkPayload {
            token: Result<String, String>,
        }

        impl Default for ApiUserProviderLinkPayload {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl ApiUserProviderLinkPayload {
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ApiUserProviderLinkPayload> for super::ApiUserProviderLinkPayload {
            type Error = String;
            fn try_from(value: ApiUserProviderLinkPayload) -> Result<Self, String> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }

        impl From<super::ApiUserProviderLinkPayload> for ApiUserProviderLinkPayload {
            fn from(value: super::ApiUserProviderLinkPayload) -> Self {
                Self {
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserUpdateParams {
            groups: Result<Vec<uuid::Uuid>, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
        }

        impl Default for ApiUserUpdateParams {
            fn default() -> Self {
                Self {
                    groups: Err("no value supplied for groups".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl ApiUserUpdateParams {
            pub fn groups<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<uuid::Uuid>>,
                T::Error: std::fmt::Display,
            {
                self.groups = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for groups: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PermissionsForApiPermission>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ApiUserUpdateParams> for super::ApiUserUpdateParams {
            type Error = String;
            fn try_from(value: ApiUserUpdateParams) -> Result<Self, String> {
                Ok(Self {
                    groups: value.groups?,
                    permissions: value.permissions?,
                })
            }
        }

        impl From<super::ApiUserUpdateParams> for ApiUserUpdateParams {
            fn from(value: super::ApiUserUpdateParams) -> Self {
                Self {
                    groups: Ok(value.groups),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateMapper {
            max_activations: Result<Option<i32>, String>,
            name: Result<String, String>,
            rule: Result<super::MappingRules, String>,
        }

        impl Default for CreateMapper {
            fn default() -> Self {
                Self {
                    max_activations: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    rule: Err("no value supplied for rule".to_string()),
                }
            }
        }

        impl CreateMapper {
            pub fn max_activations<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i32>>,
                T::Error: std::fmt::Display,
            {
                self.max_activations = value.try_into().map_err(|e| {
                    format!("error converting supplied value for max_activations: {}", e)
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn rule<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::MappingRules>,
                T::Error: std::fmt::Display,
            {
                self.rule = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rule: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<CreateMapper> for super::CreateMapper {
            type Error = String;
            fn try_from(value: CreateMapper) -> Result<Self, String> {
                Ok(Self {
                    max_activations: value.max_activations?,
                    name: value.name?,
                    rule: value.rule?,
                })
            }
        }

        impl From<super::CreateMapper> for CreateMapper {
            fn from(value: super::CreateMapper) -> Self {
                Self {
                    max_activations: Ok(value.max_activations),
                    name: Ok(value.name),
                    rule: Ok(value.rule),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Error {
            error_code: Result<Option<String>, String>,
            message: Result<String, String>,
            request_id: Result<String, String>,
        }

        impl Default for Error {
            fn default() -> Self {
                Self {
                    error_code: Ok(Default::default()),
                    message: Err("no value supplied for message".to_string()),
                    request_id: Err("no value supplied for request_id".to_string()),
                }
            }
        }

        impl Error {
            pub fn error_code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.error_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error_code: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn request_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.request_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for request_id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Error> for super::Error {
            type Error = String;
            fn try_from(value: Error) -> Result<Self, String> {
                Ok(Self {
                    error_code: value.error_code?,
                    message: value.message?,
                    request_id: value.request_id?,
                })
            }
        }

        impl From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    error_code: Ok(value.error_code),
                    message: Ok(value.message),
                    request_id: Ok(value.request_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FullRfd {
            authors: Result<Option<String>, String>,
            commit: Result<String, String>,
            committed_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            content: Result<String, String>,
            discussion: Result<Option<String>, String>,
            id: Result<uuid::Uuid, String>,
            link: Result<Option<String>, String>,
            pdfs: Result<Vec<super::FullRfdPdfEntry>, String>,
            rfd_number: Result<i32, String>,
            sha: Result<String, String>,
            state: Result<Option<String>, String>,
            title: Result<String, String>,
            visibility: Result<super::Visibility, String>,
        }

        impl Default for FullRfd {
            fn default() -> Self {
                Self {
                    authors: Ok(Default::default()),
                    commit: Err("no value supplied for commit".to_string()),
                    committed_at: Err("no value supplied for committed_at".to_string()),
                    content: Err("no value supplied for content".to_string()),
                    discussion: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    link: Ok(Default::default()),
                    pdfs: Err("no value supplied for pdfs".to_string()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                    state: Ok(Default::default()),
                    title: Err("no value supplied for title".to_string()),
                    visibility: Err("no value supplied for visibility".to_string()),
                }
            }
        }

        impl FullRfd {
            pub fn authors<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.authors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for authors: {}", e));
                self
            }
            pub fn commit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.commit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit: {}", e));
                self
            }
            pub fn committed_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.committed_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_at: {}", e)
                });
                self
            }
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn discussion<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.discussion = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for discussion: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn link<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.link = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link: {}", e));
                self
            }
            pub fn pdfs<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::FullRfdPdfEntry>>,
                T::Error: std::fmt::Display,
            {
                self.pdfs = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for pdfs: {}", e));
                self
            }
            pub fn rfd_number<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
                T::Error: std::fmt::Display,
            {
                self.rfd_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_number: {}", e));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Visibility>,
                T::Error: std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FullRfd> for super::FullRfd {
            type Error = String;
            fn try_from(value: FullRfd) -> Result<Self, String> {
                Ok(Self {
                    authors: value.authors?,
                    commit: value.commit?,
                    committed_at: value.committed_at?,
                    content: value.content?,
                    discussion: value.discussion?,
                    id: value.id?,
                    link: value.link?,
                    pdfs: value.pdfs?,
                    rfd_number: value.rfd_number?,
                    sha: value.sha?,
                    state: value.state?,
                    title: value.title?,
                    visibility: value.visibility?,
                })
            }
        }

        impl From<super::FullRfd> for FullRfd {
            fn from(value: super::FullRfd) -> Self {
                Self {
                    authors: Ok(value.authors),
                    commit: Ok(value.commit),
                    committed_at: Ok(value.committed_at),
                    content: Ok(value.content),
                    discussion: Ok(value.discussion),
                    id: Ok(value.id),
                    link: Ok(value.link),
                    pdfs: Ok(value.pdfs),
                    rfd_number: Ok(value.rfd_number),
                    sha: Ok(value.sha),
                    state: Ok(value.state),
                    title: Ok(value.title),
                    visibility: Ok(value.visibility),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FullRfdPdfEntry {
            link: Result<String, String>,
            source: Result<String, String>,
        }

        impl Default for FullRfdPdfEntry {
            fn default() -> Self {
                Self {
                    link: Err("no value supplied for link".to_string()),
                    source: Err("no value supplied for source".to_string()),
                }
            }
        }

        impl FullRfdPdfEntry {
            pub fn link<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.link = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link: {}", e));
                self
            }
            pub fn source<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.source = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<FullRfdPdfEntry> for super::FullRfdPdfEntry {
            type Error = String;
            fn try_from(value: FullRfdPdfEntry) -> Result<Self, String> {
                Ok(Self {
                    link: value.link?,
                    source: value.source?,
                })
            }
        }

        impl From<super::FullRfdPdfEntry> for FullRfdPdfEntry {
            fn from(value: super::FullRfdPdfEntry) -> Self {
                Self {
                    link: Ok(value.link),
                    source: Ok(value.source),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetApiUserResponse {
            info: Result<super::ApiUserForApiPermission, String>,
            providers: Result<Vec<super::ApiUserProvider>, String>,
        }

        impl Default for GetApiUserResponse {
            fn default() -> Self {
                Self {
                    info: Err("no value supplied for info".to_string()),
                    providers: Err("no value supplied for providers".to_string()),
                }
            }
        }

        impl GetApiUserResponse {
            pub fn info<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::ApiUserForApiPermission>,
                T::Error: std::fmt::Display,
            {
                self.info = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for info: {}", e));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::ApiUserProvider>>,
                T::Error: std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GetApiUserResponse> for super::GetApiUserResponse {
            type Error = String;
            fn try_from(value: GetApiUserResponse) -> Result<Self, String> {
                Ok(Self {
                    info: value.info?,
                    providers: value.providers?,
                })
            }
        }

        impl From<super::GetApiUserResponse> for GetApiUserResponse {
            fn from(value: super::GetApiUserResponse) -> Self {
                Self {
                    info: Ok(value.info),
                    providers: Ok(value.providers),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubCommit {
            added: Result<Vec<String>, String>,
            id: Result<String, String>,
            modified: Result<Vec<String>, String>,
            removed: Result<Vec<String>, String>,
            timestamp: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for GitHubCommit {
            fn default() -> Self {
                Self {
                    added: Err("no value supplied for added".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    modified: Err("no value supplied for modified".to_string()),
                    removed: Err("no value supplied for removed".to_string()),
                    timestamp: Err("no value supplied for timestamp".to_string()),
                }
            }
        }

        impl GitHubCommit {
            pub fn added<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.added = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for added: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn modified<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.modified = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for modified: {}", e));
                self
            }
            pub fn removed<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.removed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for removed: {}", e));
                self
            }
            pub fn timestamp<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.timestamp = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GitHubCommit> for super::GitHubCommit {
            type Error = String;
            fn try_from(value: GitHubCommit) -> Result<Self, String> {
                Ok(Self {
                    added: value.added?,
                    id: value.id?,
                    modified: value.modified?,
                    removed: value.removed?,
                    timestamp: value.timestamp?,
                })
            }
        }

        impl From<super::GitHubCommit> for GitHubCommit {
            fn from(value: super::GitHubCommit) -> Self {
                Self {
                    added: Ok(value.added),
                    id: Ok(value.id),
                    modified: Ok(value.modified),
                    removed: Ok(value.removed),
                    timestamp: Ok(value.timestamp),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubCommitPayload {
            commits: Result<Vec<super::GitHubCommit>, String>,
            head_commit: Result<super::GitHubCommit, String>,
            installation: Result<super::GitHubInstallation, String>,
            ref_: Result<String, String>,
            repository: Result<super::GitHubRepository, String>,
            sender: Result<super::GitHubSender, String>,
        }

        impl Default for GitHubCommitPayload {
            fn default() -> Self {
                Self {
                    commits: Err("no value supplied for commits".to_string()),
                    head_commit: Err("no value supplied for head_commit".to_string()),
                    installation: Err("no value supplied for installation".to_string()),
                    ref_: Err("no value supplied for ref_".to_string()),
                    repository: Err("no value supplied for repository".to_string()),
                    sender: Err("no value supplied for sender".to_string()),
                }
            }
        }

        impl GitHubCommitPayload {
            pub fn commits<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::GitHubCommit>>,
                T::Error: std::fmt::Display,
            {
                self.commits = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commits: {}", e));
                self
            }
            pub fn head_commit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::GitHubCommit>,
                T::Error: std::fmt::Display,
            {
                self.head_commit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for head_commit: {}", e));
                self
            }
            pub fn installation<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::GitHubInstallation>,
                T::Error: std::fmt::Display,
            {
                self.installation = value.try_into().map_err(|e| {
                    format!("error converting supplied value for installation: {}", e)
                });
                self
            }
            pub fn ref_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.ref_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ref_: {}", e));
                self
            }
            pub fn repository<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::GitHubRepository>,
                T::Error: std::fmt::Display,
            {
                self.repository = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for repository: {}", e));
                self
            }
            pub fn sender<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::GitHubSender>,
                T::Error: std::fmt::Display,
            {
                self.sender = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sender: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GitHubCommitPayload> for super::GitHubCommitPayload {
            type Error = String;
            fn try_from(value: GitHubCommitPayload) -> Result<Self, String> {
                Ok(Self {
                    commits: value.commits?,
                    head_commit: value.head_commit?,
                    installation: value.installation?,
                    ref_: value.ref_?,
                    repository: value.repository?,
                    sender: value.sender?,
                })
            }
        }

        impl From<super::GitHubCommitPayload> for GitHubCommitPayload {
            fn from(value: super::GitHubCommitPayload) -> Self {
                Self {
                    commits: Ok(value.commits),
                    head_commit: Ok(value.head_commit),
                    installation: Ok(value.installation),
                    ref_: Ok(value.ref_),
                    repository: Ok(value.repository),
                    sender: Ok(value.sender),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubInstallation {
            id: Result<u64, String>,
            node_id: Result<String, String>,
        }

        impl Default for GitHubInstallation {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    node_id: Err("no value supplied for node_id".to_string()),
                }
            }
        }

        impl GitHubInstallation {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn node_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.node_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for node_id: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GitHubInstallation> for super::GitHubInstallation {
            type Error = String;
            fn try_from(value: GitHubInstallation) -> Result<Self, String> {
                Ok(Self {
                    id: value.id?,
                    node_id: value.node_id?,
                })
            }
        }

        impl From<super::GitHubInstallation> for GitHubInstallation {
            fn from(value: super::GitHubInstallation) -> Self {
                Self {
                    id: Ok(value.id),
                    node_id: Ok(value.node_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubRepository {
            default_branch: Result<String, String>,
            id: Result<u64, String>,
            name: Result<String, String>,
            node_id: Result<String, String>,
            owner: Result<super::GitHubRepositoryOwner, String>,
        }

        impl Default for GitHubRepository {
            fn default() -> Self {
                Self {
                    default_branch: Err("no value supplied for default_branch".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    name: Err("no value supplied for name".to_string()),
                    node_id: Err("no value supplied for node_id".to_string()),
                    owner: Err("no value supplied for owner".to_string()),
                }
            }
        }

        impl GitHubRepository {
            pub fn default_branch<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {}", e)
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn node_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.node_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for node_id: {}", e));
                self
            }
            pub fn owner<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::GitHubRepositoryOwner>,
                T::Error: std::fmt::Display,
            {
                self.owner = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for owner: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GitHubRepository> for super::GitHubRepository {
            type Error = String;
            fn try_from(value: GitHubRepository) -> Result<Self, String> {
                Ok(Self {
                    default_branch: value.default_branch?,
                    id: value.id?,
                    name: value.name?,
                    node_id: value.node_id?,
                    owner: value.owner?,
                })
            }
        }

        impl From<super::GitHubRepository> for GitHubRepository {
            fn from(value: super::GitHubRepository) -> Self {
                Self {
                    default_branch: Ok(value.default_branch),
                    id: Ok(value.id),
                    name: Ok(value.name),
                    node_id: Ok(value.node_id),
                    owner: Ok(value.owner),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubRepositoryOwner {
            login: Result<String, String>,
        }

        impl Default for GitHubRepositoryOwner {
            fn default() -> Self {
                Self {
                    login: Err("no value supplied for login".to_string()),
                }
            }
        }

        impl GitHubRepositoryOwner {
            pub fn login<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.login = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for login: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GitHubRepositoryOwner> for super::GitHubRepositoryOwner {
            type Error = String;
            fn try_from(value: GitHubRepositoryOwner) -> Result<Self, String> {
                Ok(Self {
                    login: value.login?,
                })
            }
        }

        impl From<super::GitHubRepositoryOwner> for GitHubRepositoryOwner {
            fn from(value: super::GitHubRepositoryOwner) -> Self {
                Self {
                    login: Ok(value.login),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubSender {
            id: Result<u64, String>,
            login: Result<String, String>,
            node_id: Result<String, String>,
            type_: Result<String, String>,
        }

        impl Default for GitHubSender {
            fn default() -> Self {
                Self {
                    id: Err("no value supplied for id".to_string()),
                    login: Err("no value supplied for login".to_string()),
                    node_id: Err("no value supplied for node_id".to_string()),
                    type_: Err("no value supplied for type_".to_string()),
                }
            }
        }

        impl GitHubSender {
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<u64>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn login<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.login = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for login: {}", e));
                self
            }
            pub fn node_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.node_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for node_id: {}", e));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<GitHubSender> for super::GitHubSender {
            type Error = String;
            fn try_from(value: GitHubSender) -> Result<Self, String> {
                Ok(Self {
                    id: value.id?,
                    login: value.login?,
                    node_id: value.node_id?,
                    type_: value.type_?,
                })
            }
        }

        impl From<super::GitHubSender> for GitHubSender {
            fn from(value: super::GitHubSender) -> Self {
                Self {
                    id: Ok(value.id),
                    login: Ok(value.login),
                    node_id: Ok(value.node_id),
                    type_: Ok(value.type_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InitialApiKeyResponse {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            id: Result<uuid::Uuid, String>,
            key: Result<String, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
        }

        impl Default for InitialApiKeyResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    key: Err("no value supplied for key".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl InitialApiKeyResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::PermissionsForApiPermission>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<InitialApiKeyResponse> for super::InitialApiKeyResponse {
            type Error = String;
            fn try_from(value: InitialApiKeyResponse) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                    permissions: value.permissions?,
                })
            }
        }

        impl From<super::InitialApiKeyResponse> for InitialApiKeyResponse {
            fn from(value: super::InitialApiKeyResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InitialOAuthClientSecretResponse {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            id: Result<uuid::Uuid, String>,
            key: Result<String, String>,
        }

        impl Default for InitialOAuthClientSecretResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    key: Err("no value supplied for key".to_string()),
                }
            }
        }

        impl InitialOAuthClientSecretResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<InitialOAuthClientSecretResponse>
            for super::InitialOAuthClientSecretResponse
        {
            type Error = String;
            fn try_from(value: InitialOAuthClientSecretResponse) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                })
            }
        }

        impl From<super::InitialOAuthClientSecretResponse> for InitialOAuthClientSecretResponse {
            fn from(value: super::InitialOAuthClientSecretResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Jwk {
            e: Result<String, String>,
            kid: Result<String, String>,
            kty: Result<String, String>,
            n: Result<String, String>,
            use_: Result<String, String>,
        }

        impl Default for Jwk {
            fn default() -> Self {
                Self {
                    e: Err("no value supplied for e".to_string()),
                    kid: Err("no value supplied for kid".to_string()),
                    kty: Err("no value supplied for kty".to_string()),
                    n: Err("no value supplied for n".to_string()),
                    use_: Err("no value supplied for use_".to_string()),
                }
            }
        }

        impl Jwk {
            pub fn e<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.e = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for e: {}", e));
                self
            }
            pub fn kid<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.kid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kid: {}", e));
                self
            }
            pub fn kty<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.kty = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kty: {}", e));
                self
            }
            pub fn n<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.n = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for n: {}", e));
                self
            }
            pub fn use_<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.use_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for use_: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Jwk> for super::Jwk {
            type Error = String;
            fn try_from(value: Jwk) -> Result<Self, String> {
                Ok(Self {
                    e: value.e?,
                    kid: value.kid?,
                    kty: value.kty?,
                    n: value.n?,
                    use_: value.use_?,
                })
            }
        }

        impl From<super::Jwk> for Jwk {
            fn from(value: super::Jwk) -> Self {
                Self {
                    e: Ok(value.e),
                    kid: Ok(value.kid),
                    kty: Ok(value.kty),
                    n: Ok(value.n),
                    use_: Ok(value.use_),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Jwks {
            keys: Result<Vec<super::Jwk>, String>,
        }

        impl Default for Jwks {
            fn default() -> Self {
                Self {
                    keys: Err("no value supplied for keys".to_string()),
                }
            }
        }

        impl Jwks {
            pub fn keys<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::Jwk>>,
                T::Error: std::fmt::Display,
            {
                self.keys = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for keys: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Jwks> for super::Jwks {
            type Error = String;
            fn try_from(value: Jwks) -> Result<Self, String> {
                Ok(Self { keys: value.keys? })
            }
        }

        impl From<super::Jwks> for Jwks {
            fn from(value: super::Jwks) -> Self {
                Self {
                    keys: Ok(value.keys),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ListRfd {
            authors: Result<Option<String>, String>,
            commit: Result<String, String>,
            committed_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            discussion: Result<Option<String>, String>,
            id: Result<uuid::Uuid, String>,
            link: Result<Option<String>, String>,
            rfd_number: Result<i32, String>,
            sha: Result<String, String>,
            state: Result<Option<String>, String>,
            title: Result<String, String>,
            visibility: Result<super::Visibility, String>,
        }

        impl Default for ListRfd {
            fn default() -> Self {
                Self {
                    authors: Ok(Default::default()),
                    commit: Err("no value supplied for commit".to_string()),
                    committed_at: Err("no value supplied for committed_at".to_string()),
                    discussion: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    link: Ok(Default::default()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                    state: Ok(Default::default()),
                    title: Err("no value supplied for title".to_string()),
                    visibility: Err("no value supplied for visibility".to_string()),
                }
            }
        }

        impl ListRfd {
            pub fn authors<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.authors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for authors: {}", e));
                self
            }
            pub fn commit<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.commit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit: {}", e));
                self
            }
            pub fn committed_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.committed_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_at: {}", e)
                });
                self
            }
            pub fn discussion<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.discussion = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for discussion: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn link<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.link = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link: {}", e));
                self
            }
            pub fn rfd_number<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i32>,
                T::Error: std::fmt::Display,
            {
                self.rfd_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_number: {}", e));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::Visibility>,
                T::Error: std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<ListRfd> for super::ListRfd {
            type Error = String;
            fn try_from(value: ListRfd) -> Result<Self, String> {
                Ok(Self {
                    authors: value.authors?,
                    commit: value.commit?,
                    committed_at: value.committed_at?,
                    discussion: value.discussion?,
                    id: value.id?,
                    link: value.link?,
                    rfd_number: value.rfd_number?,
                    sha: value.sha?,
                    state: value.state?,
                    title: value.title?,
                    visibility: value.visibility?,
                })
            }
        }

        impl From<super::ListRfd> for ListRfd {
            fn from(value: super::ListRfd) -> Self {
                Self {
                    authors: Ok(value.authors),
                    commit: Ok(value.commit),
                    committed_at: Ok(value.committed_at),
                    discussion: Ok(value.discussion),
                    id: Ok(value.id),
                    link: Ok(value.link),
                    rfd_number: Ok(value.rfd_number),
                    sha: Ok(value.sha),
                    state: Ok(value.state),
                    title: Ok(value.title),
                    visibility: Ok(value.visibility),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Mapper {
            activations: Result<Option<i32>, String>,
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            deleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            depleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            id: Result<uuid::Uuid, String>,
            max_activations: Result<Option<i32>, String>,
            name: Result<String, String>,
            rule: Result<serde_json::Value, String>,
        }

        impl Default for Mapper {
            fn default() -> Self {
                Self {
                    activations: Ok(Default::default()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    depleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    max_activations: Ok(Default::default()),
                    name: Err("no value supplied for name".to_string()),
                    rule: Err("no value supplied for rule".to_string()),
                }
            }
        }

        impl Mapper {
            pub fn activations<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i32>>,
                T::Error: std::fmt::Display,
            {
                self.activations = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for activations: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn depleted_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.depleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for depleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn max_activations<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<i32>>,
                T::Error: std::fmt::Display,
            {
                self.max_activations = value.try_into().map_err(|e| {
                    format!("error converting supplied value for max_activations: {}", e)
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn rule<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<serde_json::Value>,
                T::Error: std::fmt::Display,
            {
                self.rule = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rule: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<Mapper> for super::Mapper {
            type Error = String;
            fn try_from(value: Mapper) -> Result<Self, String> {
                Ok(Self {
                    activations: value.activations?,
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    depleted_at: value.depleted_at?,
                    id: value.id?,
                    max_activations: value.max_activations?,
                    name: value.name?,
                    rule: value.rule?,
                })
            }
        }

        impl From<super::Mapper> for Mapper {
            fn from(value: super::Mapper) -> Self {
                Self {
                    activations: Ok(value.activations),
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    depleted_at: Ok(value.depleted_at),
                    id: Ok(value.id),
                    max_activations: Ok(value.max_activations),
                    name: Ok(value.name),
                    rule: Ok(value.rule),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthAuthzCodeExchangeBody {
            client_id: Result<uuid::Uuid, String>,
            client_secret: Result<String, String>,
            code: Result<String, String>,
            grant_type: Result<String, String>,
            pkce_verifier: Result<Option<String>, String>,
            redirect_uri: Result<String, String>,
        }

        impl Default for OAuthAuthzCodeExchangeBody {
            fn default() -> Self {
                Self {
                    client_id: Err("no value supplied for client_id".to_string()),
                    client_secret: Err("no value supplied for client_secret".to_string()),
                    code: Err("no value supplied for code".to_string()),
                    grant_type: Err("no value supplied for grant_type".to_string()),
                    pkce_verifier: Ok(Default::default()),
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl OAuthAuthzCodeExchangeBody {
            pub fn client_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {}", e));
                self
            }
            pub fn client_secret<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.client_secret = value.try_into().map_err(|e| {
                    format!("error converting supplied value for client_secret: {}", e)
                });
                self
            }
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {}", e));
                self
            }
            pub fn grant_type<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.grant_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grant_type: {}", e));
                self
            }
            pub fn pkce_verifier<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.pkce_verifier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for pkce_verifier: {}", e)
                });
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<OAuthAuthzCodeExchangeBody> for super::OAuthAuthzCodeExchangeBody {
            type Error = String;
            fn try_from(value: OAuthAuthzCodeExchangeBody) -> Result<Self, String> {
                Ok(Self {
                    client_id: value.client_id?,
                    client_secret: value.client_secret?,
                    code: value.code?,
                    grant_type: value.grant_type?,
                    pkce_verifier: value.pkce_verifier?,
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl From<super::OAuthAuthzCodeExchangeBody> for OAuthAuthzCodeExchangeBody {
            fn from(value: super::OAuthAuthzCodeExchangeBody) -> Self {
                Self {
                    client_id: Ok(value.client_id),
                    client_secret: Ok(value.client_secret),
                    code: Ok(value.code),
                    grant_type: Ok(value.grant_type),
                    pkce_verifier: Ok(value.pkce_verifier),
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthAuthzCodeExchangeResponse {
            access_token: Result<String, String>,
            expires_in: Result<i64, String>,
            token_type: Result<String, String>,
        }

        impl Default for OAuthAuthzCodeExchangeResponse {
            fn default() -> Self {
                Self {
                    access_token: Err("no value supplied for access_token".to_string()),
                    expires_in: Err("no value supplied for expires_in".to_string()),
                    token_type: Err("no value supplied for token_type".to_string()),
                }
            }
        }

        impl OAuthAuthzCodeExchangeResponse {
            pub fn access_token<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.access_token = value.try_into().map_err(|e| {
                    format!("error converting supplied value for access_token: {}", e)
                });
                self
            }
            pub fn expires_in<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<i64>,
                T::Error: std::fmt::Display,
            {
                self.expires_in = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_in: {}", e));
                self
            }
            pub fn token_type<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.token_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token_type: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<OAuthAuthzCodeExchangeResponse>
            for super::OAuthAuthzCodeExchangeResponse
        {
            type Error = String;
            fn try_from(value: OAuthAuthzCodeExchangeResponse) -> Result<Self, String> {
                Ok(Self {
                    access_token: value.access_token?,
                    expires_in: value.expires_in?,
                    token_type: value.token_type?,
                })
            }
        }

        impl From<super::OAuthAuthzCodeExchangeResponse> for OAuthAuthzCodeExchangeResponse {
            fn from(value: super::OAuthAuthzCodeExchangeResponse) -> Self {
                Self {
                    access_token: Ok(value.access_token),
                    expires_in: Ok(value.expires_in),
                    token_type: Ok(value.token_type),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthClient {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            deleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            id: Result<uuid::Uuid, String>,
            redirect_uris: Result<Vec<super::OAuthClientRedirectUri>, String>,
            secrets: Result<Vec<super::OAuthClientSecret>, String>,
        }

        impl Default for OAuthClient {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    redirect_uris: Err("no value supplied for redirect_uris".to_string()),
                    secrets: Err("no value supplied for secrets".to_string()),
                }
            }
        }

        impl OAuthClient {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn redirect_uris<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::OAuthClientRedirectUri>>,
                T::Error: std::fmt::Display,
            {
                self.redirect_uris = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uris: {}", e)
                });
                self
            }
            pub fn secrets<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<super::OAuthClientSecret>>,
                T::Error: std::fmt::Display,
            {
                self.secrets = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secrets: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<OAuthClient> for super::OAuthClient {
            type Error = String;
            fn try_from(value: OAuthClient) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    redirect_uris: value.redirect_uris?,
                    secrets: value.secrets?,
                })
            }
        }

        impl From<super::OAuthClient> for OAuthClient {
            fn from(value: super::OAuthClient) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    redirect_uris: Ok(value.redirect_uris),
                    secrets: Ok(value.secrets),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthClientRedirectUri {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            deleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            id: Result<uuid::Uuid, String>,
            oauth_client_id: Result<uuid::Uuid, String>,
            redirect_uri: Result<String, String>,
        }

        impl Default for OAuthClientRedirectUri {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    oauth_client_id: Err("no value supplied for oauth_client_id".to_string()),
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl OAuthClientRedirectUri {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn oauth_client_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.oauth_client_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for oauth_client_id: {}", e)
                });
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<OAuthClientRedirectUri> for super::OAuthClientRedirectUri {
            type Error = String;
            fn try_from(value: OAuthClientRedirectUri) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    oauth_client_id: value.oauth_client_id?,
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl From<super::OAuthClientRedirectUri> for OAuthClientRedirectUri {
            fn from(value: super::OAuthClientRedirectUri) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    oauth_client_id: Ok(value.oauth_client_id),
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthClientSecret {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            deleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            id: Result<uuid::Uuid, String>,
            oauth_client_id: Result<uuid::Uuid, String>,
            secret_signature: Result<String, String>,
        }

        impl Default for OAuthClientSecret {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    oauth_client_id: Err("no value supplied for oauth_client_id".to_string()),
                    secret_signature: Err("no value supplied for secret_signature".to_string()),
                }
            }
        }

        impl OAuthClientSecret {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
                T::Error: std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn oauth_client_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<uuid::Uuid>,
                T::Error: std::fmt::Display,
            {
                self.oauth_client_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for oauth_client_id: {}", e)
                });
                self
            }
            pub fn secret_signature<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.secret_signature = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for secret_signature: {}",
                        e
                    )
                });
                self
            }
        }

        impl std::convert::TryFrom<OAuthClientSecret> for super::OAuthClientSecret {
            type Error = String;
            fn try_from(value: OAuthClientSecret) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    oauth_client_id: value.oauth_client_id?,
                    secret_signature: value.secret_signature?,
                })
            }
        }

        impl From<super::OAuthClientSecret> for OAuthClientSecret {
            fn from(value: super::OAuthClientSecret) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    oauth_client_id: Ok(value.oauth_client_id),
                    secret_signature: Ok(value.secret_signature),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthProviderInfo {
            auth_url_endpoint: Result<String, String>,
            client_id: Result<String, String>,
            device_code_endpoint: Result<String, String>,
            provider: Result<super::OAuthProviderName, String>,
            scopes: Result<Vec<String>, String>,
            token_endpoint: Result<String, String>,
        }

        impl Default for OAuthProviderInfo {
            fn default() -> Self {
                Self {
                    auth_url_endpoint: Err("no value supplied for auth_url_endpoint".to_string()),
                    client_id: Err("no value supplied for client_id".to_string()),
                    device_code_endpoint: Err(
                        "no value supplied for device_code_endpoint".to_string()
                    ),
                    provider: Err("no value supplied for provider".to_string()),
                    scopes: Err("no value supplied for scopes".to_string()),
                    token_endpoint: Err("no value supplied for token_endpoint".to_string()),
                }
            }
        }

        impl OAuthProviderInfo {
            pub fn auth_url_endpoint<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.auth_url_endpoint = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for auth_url_endpoint: {}",
                        e
                    )
                });
                self
            }
            pub fn client_id<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {}", e));
                self
            }
            pub fn device_code_endpoint<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.device_code_endpoint = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for device_code_endpoint: {}",
                        e
                    )
                });
                self
            }
            pub fn provider<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::OAuthProviderName>,
                T::Error: std::fmt::Display,
            {
                self.provider = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider: {}", e));
                self
            }
            pub fn scopes<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Vec<String>>,
                T::Error: std::fmt::Display,
            {
                self.scopes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scopes: {}", e));
                self
            }
            pub fn token_endpoint<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.token_endpoint = value.try_into().map_err(|e| {
                    format!("error converting supplied value for token_endpoint: {}", e)
                });
                self
            }
        }

        impl std::convert::TryFrom<OAuthProviderInfo> for super::OAuthProviderInfo {
            type Error = String;
            fn try_from(value: OAuthProviderInfo) -> Result<Self, String> {
                Ok(Self {
                    auth_url_endpoint: value.auth_url_endpoint?,
                    client_id: value.client_id?,
                    device_code_endpoint: value.device_code_endpoint?,
                    provider: value.provider?,
                    scopes: value.scopes?,
                    token_endpoint: value.token_endpoint?,
                })
            }
        }

        impl From<super::OAuthProviderInfo> for OAuthProviderInfo {
            fn from(value: super::OAuthProviderInfo) -> Self {
                Self {
                    auth_url_endpoint: Ok(value.auth_url_endpoint),
                    client_id: Ok(value.client_id),
                    device_code_endpoint: Ok(value.device_code_endpoint),
                    provider: Ok(value.provider),
                    scopes: Ok(value.scopes),
                    token_endpoint: Ok(value.token_endpoint),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OpenIdConfiguration {
            jwks_uri: Result<String, String>,
        }

        impl Default for OpenIdConfiguration {
            fn default() -> Self {
                Self {
                    jwks_uri: Err("no value supplied for jwks_uri".to_string()),
                }
            }
        }

        impl OpenIdConfiguration {
            pub fn jwks_uri<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<String>,
                T::Error: std::fmt::Display,
            {
                self.jwks_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for jwks_uri: {}", e));
                self
            }
        }

        impl std::convert::TryFrom<OpenIdConfiguration> for super::OpenIdConfiguration {
            type Error = String;
            fn try_from(value: OpenIdConfiguration) -> Result<Self, String> {
                Ok(Self {
                    jwks_uri: value.jwks_uri?,
                })
            }
        }

        impl From<super::OpenIdConfiguration> for OpenIdConfiguration {
            fn from(value: super::OpenIdConfiguration) -> Self {
                Self {
                    jwks_uri: Ok(value.jwks_uri),
                }
            }
        }
    }

    pub mod defaults {
        pub(super) fn mapping_rules_email_address_permissions() -> super::PermissionsForApiPermission
        {
            super::PermissionsForApiPermission(vec![])
        }

        pub(super) fn mapping_rules_email_domain_permissions() -> super::PermissionsForApiPermission
        {
            super::PermissionsForApiPermission(vec![])
        }

        pub(super) fn mapping_rules_git_hub_username_permissions(
        ) -> super::PermissionsForApiPermission {
            super::PermissionsForApiPermission(vec![])
        }
    }
}

#[derive(Clone, Debug)]
/// Client for RFD API
///
/// Programmatic access to RFDs
///
/// Version:
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        ""
    }
}

impl Client {
    /// Sends a `GET` request to `/.well-known/jwks.json`
    ///
    /// ```ignore
    /// let response = client.jwks_json()
    ///    .send()
    ///    .await;
    /// ```
    pub fn jwks_json(&self) -> builder::JwksJson {
        builder::JwksJson::new(self)
    }

    /// Sends a `GET` request to `/.well-known/openid-configuration`
    ///
    /// ```ignore
    /// let response = client.openid_configuration()
    ///    .send()
    ///    .await;
    /// ```
    pub fn openid_configuration(&self) -> builder::OpenidConfiguration {
        builder::OpenidConfiguration::new(self)
    }

    /// Create a new user with a given set of permissions
    ///
    /// Sends a `POST` request to `/api-user`
    ///
    /// ```ignore
    /// let response = client.create_api_user()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_api_user(&self) -> builder::CreateApiUser {
        builder::CreateApiUser::new(self)
    }

    /// Get user information for a given user id
    ///
    /// Sends a `GET` request to `/api-user/{identifier}`
    ///
    /// ```ignore
    /// let response = client.get_api_user()
    ///    .identifier(identifier)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_api_user(&self) -> builder::GetApiUser {
        builder::GetApiUser::new(self)
    }

    /// Update the permissions assigned to a given user
    ///
    /// Sends a `POST` request to `/api-user/{identifier}`
    ///
    /// ```ignore
    /// let response = client.update_api_user()
    ///    .identifier(identifier)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_api_user(&self) -> builder::UpdateApiUser {
        builder::UpdateApiUser::new(self)
    }

    /// Sends a `POST` request to `/api-user/{identifier}/group`
    ///
    /// ```ignore
    /// let response = client.add_api_user_to_group()
    ///    .identifier(identifier)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn add_api_user_to_group(&self) -> builder::AddApiUserToGroup {
        builder::AddApiUserToGroup::new(self)
    }

    /// Sends a `DELETE` request to `/api-user/{identifier}/group/{group_id}`
    ///
    /// ```ignore
    /// let response = client.remove_api_user_from_group()
    ///    .identifier(identifier)
    ///    .group_id(group_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn remove_api_user_from_group(&self) -> builder::RemoveApiUserFromGroup {
        builder::RemoveApiUserFromGroup::new(self)
    }

    /// Link an existing login provider to this user
    ///
    /// Sends a `POST` request to `/api-user/{identifier}/link`
    ///
    /// ```ignore
    /// let response = client.link_provider()
    ///    .identifier(identifier)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn link_provider(&self) -> builder::LinkProvider {
        builder::LinkProvider::new(self)
    }

    /// List the active and expired API tokens for a given user
    ///
    /// Sends a `GET` request to `/api-user/{identifier}/token`
    ///
    /// ```ignore
    /// let response = client.list_api_user_tokens()
    ///    .identifier(identifier)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_api_user_tokens(&self) -> builder::ListApiUserTokens {
        builder::ListApiUserTokens::new(self)
    }

    /// Sends a `POST` request to `/api-user/{identifier}/token`
    ///
    /// ```ignore
    /// let response = client.create_api_user_token()
    ///    .identifier(identifier)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_api_user_token(&self) -> builder::CreateApiUserToken {
        builder::CreateApiUserToken::new(self)
    }

    /// Sends a `GET` request to
    /// `/api-user/{identifier}/token/{token_identifier}`
    ///
    /// ```ignore
    /// let response = client.get_api_user_token()
    ///    .identifier(identifier)
    ///    .token_identifier(token_identifier)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_api_user_token(&self) -> builder::GetApiUserToken {
        builder::GetApiUserToken::new(self)
    }

    /// Sends a `DELETE` request to
    /// `/api-user/{identifier}/token/{token_identifier}`
    ///
    /// ```ignore
    /// let response = client.delete_api_user_token()
    ///    .identifier(identifier)
    ///    .token_identifier(token_identifier)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_api_user_token(&self) -> builder::DeleteApiUserToken {
        builder::DeleteApiUserToken::new(self)
    }

    /// Create a new link token for linking this provider to a different api
    /// user
    ///
    /// Sends a `POST` request to `/api-user-provider/{identifier}/link-token`
    ///
    /// ```ignore
    /// let response = client.create_link_token()
    ///    .identifier(identifier)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_link_token(&self) -> builder::CreateLinkToken {
        builder::CreateLinkToken::new(self)
    }

    /// Sends a `GET` request to `/group`
    ///
    /// ```ignore
    /// let response = client.get_groups()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_groups(&self) -> builder::GetGroups {
        builder::GetGroups::new(self)
    }

    /// Sends a `POST` request to `/group`
    ///
    /// ```ignore
    /// let response = client.create_group()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_group(&self) -> builder::CreateGroup {
        builder::CreateGroup::new(self)
    }

    /// Sends a `PUT` request to `/group/{group_id}`
    ///
    /// ```ignore
    /// let response = client.update_group()
    ///    .group_id(group_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_group(&self) -> builder::UpdateGroup {
        builder::UpdateGroup::new(self)
    }

    /// Sends a `DELETE` request to `/group/{group_id}`
    ///
    /// ```ignore
    /// let response = client.delete_group()
    ///    .group_id(group_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_group(&self) -> builder::DeleteGroup {
        builder::DeleteGroup::new(self)
    }

    /// Generate the remote provider login url and redirect the user
    ///
    /// Sends a `GET` request to `/login/oauth/{provider}/code/authorize`
    ///
    /// ```ignore
    /// let response = client.authz_code_redirect()
    ///    .provider(provider)
    ///    .client_id(client_id)
    ///    .redirect_uri(redirect_uri)
    ///    .response_type(response_type)
    ///    .scope(scope)
    ///    .state(state)
    ///    .send()
    ///    .await;
    /// ```
    pub fn authz_code_redirect(&self) -> builder::AuthzCodeRedirect {
        builder::AuthzCodeRedirect::new(self)
    }

    /// Handle return calls from a remote OAuth provider
    ///
    /// Sends a `GET` request to `/login/oauth/{provider}/code/callback`
    ///
    /// ```ignore
    /// let response = client.authz_code_callback()
    ///    .provider(provider)
    ///    .code(code)
    ///    .error(error)
    ///    .state(state)
    ///    .send()
    ///    .await;
    /// ```
    pub fn authz_code_callback(&self) -> builder::AuthzCodeCallback {
        builder::AuthzCodeCallback::new(self)
    }

    /// Exchange an authorization code for an access token
    ///
    /// Sends a `POST` request to `/login/oauth/{provider}/code/token`
    ///
    /// ```ignore
    /// let response = client.authz_code_exchange()
    ///    .provider(provider)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn authz_code_exchange(&self) -> builder::AuthzCodeExchange {
        builder::AuthzCodeExchange::new(self)
    }

    /// Sends a `GET` request to `/login/oauth/{provider}/device`
    ///
    /// ```ignore
    /// let response = client.get_device_provider()
    ///    .provider(provider)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_device_provider(&self) -> builder::GetDeviceProvider {
        builder::GetDeviceProvider::new(self)
    }

    /// Sends a `POST` request to `/login/oauth/{provider}/device/exchange`
    ///
    /// ```ignore
    /// let response = client.exchange_device_token()
    ///    .provider(provider)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn exchange_device_token(&self) -> builder::ExchangeDeviceToken {
        builder::ExchangeDeviceToken::new(self)
    }

    /// Sends a `GET` request to `/mapper`
    ///
    /// ```ignore
    /// let response = client.get_mappers()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_mappers(&self) -> builder::GetMappers {
        builder::GetMappers::new(self)
    }

    /// Sends a `POST` request to `/mapper`
    ///
    /// ```ignore
    /// let response = client.create_mapper()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_mapper(&self) -> builder::CreateMapper {
        builder::CreateMapper::new(self)
    }

    /// Sends a `DELETE` request to `/mapper/{identifier}`
    ///
    /// ```ignore
    /// let response = client.delete_mapper()
    ///    .identifier(identifier)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_mapper(&self) -> builder::DeleteMapper {
        builder::DeleteMapper::new(self)
    }

    /// List OAuth clients
    ///
    /// Sends a `GET` request to `/oauth/client`
    ///
    /// ```ignore
    /// let response = client.list_oauth_clients()
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_oauth_clients(&self) -> builder::ListOauthClients {
        builder::ListOauthClients::new(self)
    }

    /// Create a new OAuth Client
    ///
    /// Sends a `POST` request to `/oauth/client`
    ///
    /// ```ignore
    /// let response = client.create_oauth_client()
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_oauth_client(&self) -> builder::CreateOauthClient {
        builder::CreateOauthClient::new(self)
    }

    /// Get an new OAuth Client
    ///
    /// Sends a `GET` request to `/oauth/client/{client_id}`
    ///
    /// ```ignore
    /// let response = client.get_oauth_client()
    ///    .client_id(client_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_oauth_client(&self) -> builder::GetOauthClient {
        builder::GetOauthClient::new(self)
    }

    /// Add an OAuth client redirect uri
    ///
    /// Sends a `POST` request to `/oauth/client/{client_id}/redirect_uri`
    ///
    /// ```ignore
    /// let response = client.create_oauth_client_redirect_uri()
    ///    .client_id(client_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_oauth_client_redirect_uri(&self) -> builder::CreateOauthClientRedirectUri {
        builder::CreateOauthClientRedirectUri::new(self)
    }

    /// Delete an OAuth client redirect uri
    ///
    /// Sends a `DELETE` request to
    /// `/oauth/client/{client_id}/redirect_uri/{redirect_uri_id}`
    ///
    /// ```ignore
    /// let response = client.delete_oauth_client_redirect_uri()
    ///    .client_id(client_id)
    ///    .redirect_uri_id(redirect_uri_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_oauth_client_redirect_uri(&self) -> builder::DeleteOauthClientRedirectUri {
        builder::DeleteOauthClientRedirectUri::new(self)
    }

    /// Add an OAuth client secret
    ///
    /// Sends a `POST` request to `/oauth/client/{client_id}/secret`
    ///
    /// ```ignore
    /// let response = client.create_oauth_client_secret()
    ///    .client_id(client_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_oauth_client_secret(&self) -> builder::CreateOauthClientSecret {
        builder::CreateOauthClientSecret::new(self)
    }

    /// Delete an OAuth client secret
    ///
    /// Sends a `DELETE` request to
    /// `/oauth/client/{client_id}/secret/{secret_id}`
    ///
    /// ```ignore
    /// let response = client.delete_oauth_client_secret()
    ///    .client_id(client_id)
    ///    .secret_id(secret_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_oauth_client_secret(&self) -> builder::DeleteOauthClientSecret {
        builder::DeleteOauthClientSecret::new(self)
    }

    /// List all available RFDs
    ///
    /// Sends a `GET` request to `/rfd`
    ///
    /// ```ignore
    /// let response = client.get_rfds()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_rfds(&self) -> builder::GetRfds {
        builder::GetRfds::new(self)
    }

    /// Get the latest representation of an RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}`
    ///
    /// ```ignore
    /// let response = client.get_rfd()
    ///    .number(number)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_rfd(&self) -> builder::GetRfd {
        builder::GetRfd::new(self)
    }

    /// Search the RFD index and get a list of results
    ///
    /// Sends a `GET` request to `/rfd-search`
    ///
    /// ```ignore
    /// let response = client.search_rfds()
    ///    .q(q)
    ///    .send()
    ///    .await;
    /// ```
    pub fn search_rfds(&self) -> builder::SearchRfds {
        builder::SearchRfds::new(self)
    }

    /// Retrieve the user information of the calling user
    ///
    /// Sends a `GET` request to `/self`
    ///
    /// ```ignore
    /// let response = client.get_self()
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_self(&self) -> builder::GetSelf {
        builder::GetSelf::new(self)
    }
}

pub trait ClientHiddenExt {
    /// Sends a `POST` request to `/github`
    ///
    /// ```ignore
    /// let response = client.github_webhook()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    fn github_webhook(&self) -> builder::GithubWebhook;
}

impl ClientHiddenExt for Client {
    fn github_webhook(&self) -> builder::GithubWebhook {
        builder::GithubWebhook::new(self)
    }
}

pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
    /// Builder for [`Client::jwks_json`]
    ///
    /// [`Client::jwks_json`]: super::Client::jwks_json
    #[derive(Debug, Clone)]
    pub struct JwksJson<'a> {
        client: &'a super::Client,
    }

    impl<'a> JwksJson<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        /// Sends a `GET` request to `/.well-known/jwks.json`
        pub async fn send(self) -> Result<ResponseValue<types::Jwks>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/.well-known/jwks.json", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::openid_configuration`]
    ///
    /// [`Client::openid_configuration`]: super::Client::openid_configuration
    #[derive(Debug, Clone)]
    pub struct OpenidConfiguration<'a> {
        client: &'a super::Client,
    }

    impl<'a> OpenidConfiguration<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        /// Sends a `GET` request to `/.well-known/openid-configuration`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OpenIdConfiguration>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/.well-known/openid-configuration", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_api_user`]
    ///
    /// [`Client::create_api_user`]: super::Client::create_api_user
    #[derive(Debug, Clone)]
    pub struct CreateApiUser<'a> {
        client: &'a super::Client,
        body: Result<types::builder::ApiUserUpdateParams, String>,
    }

    impl<'a> CreateApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                body: Ok(types::builder::ApiUserUpdateParams::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserUpdateParams>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `ApiUserUpdateParams` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiUserUpdateParams,
            ) -> types::builder::ApiUserUpdateParams,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserForApiPermission>, Error<types::Error>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::ApiUserUpdateParams>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api-user", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_api_user`]
    ///
    /// [`Client::get_api_user`]: super::Client::get_api_user
    #[derive(Debug, Clone)]
    pub struct GetApiUser<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
    }

    impl<'a> GetApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        /// Sends a `GET` request to `/api-user/{identifier}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetApiUserResponse>, Error<types::Error>> {
            let Self { client, identifier } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}",
                client.baseurl,
                encode_path(&identifier.to_string()),
            );
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::update_api_user`]
    ///
    /// [`Client::update_api_user`]: super::Client::update_api_user
    #[derive(Debug, Clone)]
    pub struct UpdateApiUser<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
        body: Result<types::builder::ApiUserUpdateParams, String>,
    }

    impl<'a> UpdateApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                body: Ok(types::builder::ApiUserUpdateParams::default()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserUpdateParams>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `ApiUserUpdateParams` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiUserUpdateParams,
            ) -> types::builder::ApiUserUpdateParams,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{identifier}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserForApiPermission>, Error<types::Error>> {
            let Self {
                client,
                identifier,
                body,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::ApiUserUpdateParams>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}",
                client.baseurl,
                encode_path(&identifier.to_string()),
            );
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::add_api_user_to_group`]
    ///
    /// [`Client::add_api_user_to_group`]: super::Client::add_api_user_to_group
    #[derive(Debug, Clone)]
    pub struct AddApiUserToGroup<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
        body: Result<types::builder::AddGroupBody, String>,
    }

    impl<'a> AddApiUserToGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                body: Ok(types::builder::AddGroupBody::default()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddGroupBody>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `AddGroupBody` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::AddGroupBody) -> types::builder::AddGroupBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{identifier}/group`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserForApiPermission>, Error<types::Error>> {
            let Self {
                client,
                identifier,
                body,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AddGroupBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/group",
                client.baseurl,
                encode_path(&identifier.to_string()),
            );
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::remove_api_user_from_group`]
    ///
    /// [`Client::remove_api_user_from_group`]: super::Client::remove_api_user_from_group
    #[derive(Debug, Clone)]
    pub struct RemoveApiUserFromGroup<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
        group_id: Result<uuid::Uuid, String>,
    }

    impl<'a> RemoveApiUserFromGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                group_id: Err("group_id was not initialized".to_string()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.group_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for group_id failed".to_string());
            self
        }

        /// Sends a `DELETE` request to
        /// `/api-user/{identifier}/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserForApiPermission>, Error<types::Error>> {
            let Self {
                client,
                identifier,
                group_id,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/group/{}",
                client.baseurl,
                encode_path(&identifier.to_string()),
                encode_path(&group_id.to_string()),
            );
            let request = client
                .client
                .delete(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::link_provider`]
    ///
    /// [`Client::link_provider`]: super::Client::link_provider
    #[derive(Debug, Clone)]
    pub struct LinkProvider<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
        body: Result<types::builder::ApiUserProviderLinkPayload, String>,
    }

    impl<'a> LinkProvider<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                body: Ok(types::builder::ApiUserProviderLinkPayload::default()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserProviderLinkPayload>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `ApiUserProviderLinkPayload` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiUserProviderLinkPayload,
            ) -> types::builder::ApiUserProviderLinkPayload,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{identifier}/link`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self {
                client,
                identifier,
                body,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::ApiUserProviderLinkPayload>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/link",
                client.baseurl,
                encode_path(&identifier.to_string()),
            );
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                204u16 => Ok(ResponseValue::empty(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::list_api_user_tokens`]
    ///
    /// [`Client::list_api_user_tokens`]: super::Client::list_api_user_tokens
    #[derive(Debug, Clone)]
    pub struct ListApiUserTokens<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
    }

    impl<'a> ListApiUserTokens<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        /// Sends a `GET` request to `/api-user/{identifier}/token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<Vec<types::ApiKeyResponse>>, Error<types::Error>> {
            let Self { client, identifier } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token",
                client.baseurl,
                encode_path(&identifier.to_string()),
            );
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_api_user_token`]
    ///
    /// [`Client::create_api_user_token`]: super::Client::create_api_user_token
    #[derive(Debug, Clone)]
    pub struct CreateApiUserToken<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
        body: Result<types::builder::ApiKeyCreateParams, String>,
    }

    impl<'a> CreateApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                body: Ok(types::builder::ApiKeyCreateParams::default()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiKeyCreateParams>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `ApiKeyCreateParams` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiKeyCreateParams,
            ) -> types::builder::ApiKeyCreateParams,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{identifier}/token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InitialApiKeyResponse>, Error<types::Error>> {
            let Self {
                client,
                identifier,
                body,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::ApiKeyCreateParams>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token",
                client.baseurl,
                encode_path(&identifier.to_string()),
            );
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                201u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_api_user_token`]
    ///
    /// [`Client::get_api_user_token`]: super::Client::get_api_user_token
    #[derive(Debug, Clone)]
    pub struct GetApiUserToken<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
        token_identifier: Result<uuid::Uuid, String>,
    }

    impl<'a> GetApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                token_identifier: Err("token_identifier was not initialized".to_string()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        pub fn token_identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.token_identifier = value.try_into().map_err(|_| {
                "conversion to `uuid :: Uuid` for token_identifier failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to
        /// `/api-user/{identifier}/token/{token_identifier}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiKeyResponse>, Error<types::Error>> {
            let Self {
                client,
                identifier,
                token_identifier,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let token_identifier = token_identifier.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token/{}",
                client.baseurl,
                encode_path(&identifier.to_string()),
                encode_path(&token_identifier.to_string()),
            );
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_api_user_token`]
    ///
    /// [`Client::delete_api_user_token`]: super::Client::delete_api_user_token
    #[derive(Debug, Clone)]
    pub struct DeleteApiUserToken<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
        token_identifier: Result<uuid::Uuid, String>,
    }

    impl<'a> DeleteApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                token_identifier: Err("token_identifier was not initialized".to_string()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        pub fn token_identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.token_identifier = value.try_into().map_err(|_| {
                "conversion to `uuid :: Uuid` for token_identifier failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to
        /// `/api-user/{identifier}/token/{token_identifier}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiKeyResponse>, Error<types::Error>> {
            let Self {
                client,
                identifier,
                token_identifier,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let token_identifier = token_identifier.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token/{}",
                client.baseurl,
                encode_path(&identifier.to_string()),
                encode_path(&token_identifier.to_string()),
            );
            let request = client
                .client
                .delete(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_link_token`]
    ///
    /// [`Client::create_link_token`]: super::Client::create_link_token
    #[derive(Debug, Clone)]
    pub struct CreateLinkToken<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
        body: Result<types::builder::ApiUserLinkRequestPayload, String>,
    }

    impl<'a> CreateLinkToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                body: Ok(types::builder::ApiUserLinkRequestPayload::default()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserLinkRequestPayload>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `ApiUserLinkRequestPayload` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiUserLinkRequestPayload,
            ) -> types::builder::ApiUserLinkRequestPayload,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to
        /// `/api-user-provider/{identifier}/link-token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserLinkRequestResponse>, Error<types::Error>> {
            let Self {
                client,
                identifier,
                body,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::ApiUserLinkRequestPayload>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user-provider/{}/link-token",
                client.baseurl,
                encode_path(&identifier.to_string()),
            );
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`ClientHiddenExt::github_webhook`]
    ///
    /// [`ClientHiddenExt::github_webhook`]: super::ClientHiddenExt::github_webhook
    #[derive(Debug, Clone)]
    pub struct GithubWebhook<'a> {
        client: &'a super::Client,
        body: Result<types::builder::GitHubCommitPayload, String>,
    }

    impl<'a> GithubWebhook<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                body: Ok(types::builder::GitHubCommitPayload::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GitHubCommitPayload>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `GitHubCommitPayload` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::GitHubCommitPayload,
            ) -> types::builder::GitHubCommitPayload,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/github`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::GitHubCommitPayload>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/github", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                202u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_groups`]
    ///
    /// [`Client::get_groups`]: super::Client::get_groups
    #[derive(Debug, Clone)]
    pub struct GetGroups<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetGroups<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        /// Sends a `GET` request to `/group`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<Vec<types::AccessGroupForApiPermission>>, Error<types::Error>>
        {
            let Self { client } = self;
            let url = format!("{}/group", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_group`]
    ///
    /// [`Client::create_group`]: super::Client::create_group
    #[derive(Debug, Clone)]
    pub struct CreateGroup<'a> {
        client: &'a super::Client,
        body: Result<types::builder::AccessGroupUpdateParams, String>,
    }

    impl<'a> CreateGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                body: Ok(types::builder::AccessGroupUpdateParams::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessGroupUpdateParams>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `AccessGroupUpdateParams` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AccessGroupUpdateParams,
            ) -> types::builder::AccessGroupUpdateParams,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/group`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForApiPermission>, Error<types::Error>>
        {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::AccessGroupUpdateParams>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/group", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::update_group`]
    ///
    /// [`Client::update_group`]: super::Client::update_group
    #[derive(Debug, Clone)]
    pub struct UpdateGroup<'a> {
        client: &'a super::Client,
        group_id: Result<uuid::Uuid, String>,
        body: Result<types::builder::AccessGroupUpdateParams, String>,
    }

    impl<'a> UpdateGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                group_id: Err("group_id was not initialized".to_string()),
                body: Ok(types::builder::AccessGroupUpdateParams::default()),
            }
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.group_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for group_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessGroupUpdateParams>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `AccessGroupUpdateParams` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AccessGroupUpdateParams,
            ) -> types::builder::AccessGroupUpdateParams,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `PUT` request to `/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForApiPermission>, Error<types::Error>>
        {
            let Self {
                client,
                group_id,
                body,
            } = self;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AccessGroupUpdateParams>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/group/{}",
                client.baseurl,
                encode_path(&group_id.to_string()),
            );
            let request = client
                .client
                .put(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_group`]
    ///
    /// [`Client::delete_group`]: super::Client::delete_group
    #[derive(Debug, Clone)]
    pub struct DeleteGroup<'a> {
        client: &'a super::Client,
        group_id: Result<uuid::Uuid, String>,
    }

    impl<'a> DeleteGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                group_id: Err("group_id was not initialized".to_string()),
            }
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.group_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for group_id failed".to_string());
            self
        }

        /// Sends a `DELETE` request to `/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForApiPermission>, Error<types::Error>>
        {
            let Self { client, group_id } = self;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/group/{}",
                client.baseurl,
                encode_path(&group_id.to_string()),
            );
            let request = client
                .client
                .delete(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::authz_code_redirect`]
    ///
    /// [`Client::authz_code_redirect`]: super::Client::authz_code_redirect
    #[derive(Debug, Clone)]
    pub struct AuthzCodeRedirect<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        client_id: Result<uuid::Uuid, String>,
        redirect_uri: Result<String, String>,
        response_type: Result<String, String>,
        scope: Result<Option<String>, String>,
        state: Result<String, String>,
    }

    impl<'a> AuthzCodeRedirect<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                provider: Err("provider was not initialized".to_string()),
                client_id: Err("client_id was not initialized".to_string()),
                redirect_uri: Err("redirect_uri was not initialized".to_string()),
                response_type: Err("response_type was not initialized".to_string()),
                scope: Ok(None),
                state: Err("state was not initialized".to_string()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.client_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for client_id failed".to_string());
            self
        }

        pub fn redirect_uri<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.redirect_uri = value
                .try_into()
                .map_err(|_| "conversion to `String` for redirect_uri failed".to_string());
            self
        }

        pub fn response_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.response_type = value
                .try_into()
                .map_err(|_| "conversion to `String` for response_type failed".to_string());
            self
        }

        pub fn scope<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.scope = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for scope failed".to_string());
            self
        }

        pub fn state<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.state = value
                .try_into()
                .map_err(|_| "conversion to `String` for state failed".to_string());
            self
        }

        /// Sends a `GET` request to `/login/oauth/{provider}/code/authorize`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
            let Self {
                client,
                provider,
                client_id,
                redirect_uri,
                response_type,
                scope,
                state,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let redirect_uri = redirect_uri.map_err(Error::InvalidRequest)?;
            let response_type = response_type.map_err(Error::InvalidRequest)?;
            let scope = scope.map_err(Error::InvalidRequest)?;
            let state = state.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/code/authorize",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut query = Vec::with_capacity(5usize);
            query.push(("client_id", client_id.to_string()));
            query.push(("redirect_uri", redirect_uri.to_string()));
            query.push(("response_type", response_type.to_string()));
            if let Some(v) = &scope {
                query.push(("scope", v.to_string()));
            }
            query.push(("state", state.to_string()));
            let request = client.client.get(url).query(&query).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
            }
        }
    }

    /// Builder for [`Client::authz_code_callback`]
    ///
    /// [`Client::authz_code_callback`]: super::Client::authz_code_callback
    #[derive(Debug, Clone)]
    pub struct AuthzCodeCallback<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        code: Result<Option<String>, String>,
        error: Result<Option<String>, String>,
        state: Result<Option<String>, String>,
    }

    impl<'a> AuthzCodeCallback<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                provider: Err("provider was not initialized".to_string()),
                code: Ok(None),
                error: Ok(None),
                state: Ok(None),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn code<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.code = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for code failed".to_string());
            self
        }

        pub fn error<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.error = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for error failed".to_string());
            self
        }

        pub fn state<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.state = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `String` for state failed".to_string());
            self
        }

        /// Sends a `GET` request to `/login/oauth/{provider}/code/callback`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<types::Error>> {
            let Self {
                client,
                provider,
                code,
                error,
                state,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let code = code.map_err(Error::InvalidRequest)?;
            let error = error.map_err(Error::InvalidRequest)?;
            let state = state.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/code/callback",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut query = Vec::with_capacity(3usize);
            if let Some(v) = &code {
                query.push(("code", v.to_string()));
            }
            if let Some(v) = &error {
                query.push(("error", v.to_string()));
            }
            if let Some(v) = &state {
                query.push(("state", v.to_string()));
            }
            let request = client.client.get(url).query(&query).build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::authz_code_exchange`]
    ///
    /// [`Client::authz_code_exchange`]: super::Client::authz_code_exchange
    #[derive(Debug, Clone)]
    pub struct AuthzCodeExchange<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        body: Result<types::builder::OAuthAuthzCodeExchangeBody, String>,
    }

    impl<'a> AuthzCodeExchange<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                provider: Err("provider was not initialized".to_string()),
                body: Ok(types::builder::OAuthAuthzCodeExchangeBody::default()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthAuthzCodeExchangeBody>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `OAuthAuthzCodeExchangeBody` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::OAuthAuthzCodeExchangeBody,
            ) -> types::builder::OAuthAuthzCodeExchangeBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/oauth/{provider}/code/token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthAuthzCodeExchangeResponse>, Error<types::Error>>
        {
            let Self {
                client,
                provider,
                body,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::OAuthAuthzCodeExchangeBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/code/token",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_device_provider`]
    ///
    /// [`Client::get_device_provider`]: super::Client::get_device_provider
    #[derive(Debug, Clone)]
    pub struct GetDeviceProvider<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
    }

    impl<'a> GetDeviceProvider<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                provider: Err("provider was not initialized".to_string()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        /// Sends a `GET` request to `/login/oauth/{provider}/device`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthProviderInfo>, Error<types::Error>> {
            let Self { client, provider } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/device",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::exchange_device_token`]
    ///
    /// [`Client::exchange_device_token`]: super::Client::exchange_device_token
    #[derive(Debug, Clone)]
    pub struct ExchangeDeviceToken<'a> {
        client: &'a super::Client,
        provider: Result<types::OAuthProviderName, String>,
        body: Result<types::builder::AccessTokenExchangeRequest, String>,
    }

    impl<'a> ExchangeDeviceToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                provider: Err("provider was not initialized".to_string()),
                body: Ok(types::builder::AccessTokenExchangeRequest::default()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::OAuthProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `OAuthProviderName` for provider failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessTokenExchangeRequest>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `AccessTokenExchangeRequest` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AccessTokenExchangeRequest,
            ) -> types::builder::AccessTokenExchangeRequest,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/oauth/{provider}/device/exchange`
        pub async fn send(self) -> Result<ResponseValue<ByteStream>, Error<ByteStream>> {
            let Self {
                client,
                provider,
                body,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AccessTokenExchangeRequest>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/device/exchange",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let request = client.client.post(url).form_urlencoded(&body)?.build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
            }
        }
    }

    /// Builder for [`Client::get_mappers`]
    ///
    /// [`Client::get_mappers`]: super::Client::get_mappers
    #[derive(Debug, Clone)]
    pub struct GetMappers<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetMappers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        /// Sends a `GET` request to `/mapper`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::Mapper>>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/mapper", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_mapper`]
    ///
    /// [`Client::create_mapper`]: super::Client::create_mapper
    #[derive(Debug, Clone)]
    pub struct CreateMapper<'a> {
        client: &'a super::Client,
        body: Result<types::builder::CreateMapper, String>,
    }

    impl<'a> CreateMapper<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                body: Ok(types::builder::CreateMapper::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateMapper>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `CreateMapper` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::CreateMapper) -> types::builder::CreateMapper,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/mapper`
        pub async fn send(self) -> Result<ResponseValue<types::Mapper>, Error<types::Error>> {
            let Self { client, body } = self;
            let body = body
                .and_then(std::convert::TryInto::<types::CreateMapper>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/mapper", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_mapper`]
    ///
    /// [`Client::delete_mapper`]: super::Client::delete_mapper
    #[derive(Debug, Clone)]
    pub struct DeleteMapper<'a> {
        client: &'a super::Client,
        identifier: Result<uuid::Uuid, String>,
    }

    impl<'a> DeleteMapper<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
            }
        }

        pub fn identifier<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.identifier = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for identifier failed".to_string());
            self
        }

        /// Sends a `DELETE` request to `/mapper/{identifier}`
        pub async fn send(self) -> Result<ResponseValue<types::Mapper>, Error<types::Error>> {
            let Self { client, identifier } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/mapper/{}",
                client.baseurl,
                encode_path(&identifier.to_string()),
            );
            let request = client
                .client
                .delete(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::list_oauth_clients`]
    ///
    /// [`Client::list_oauth_clients`]: super::Client::list_oauth_clients
    #[derive(Debug, Clone)]
    pub struct ListOauthClients<'a> {
        client: &'a super::Client,
    }

    impl<'a> ListOauthClients<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        /// Sends a `GET` request to `/oauth/client`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<Vec<types::OAuthClient>>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/oauth/client", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_oauth_client`]
    ///
    /// [`Client::create_oauth_client`]: super::Client::create_oauth_client
    #[derive(Debug, Clone)]
    pub struct CreateOauthClient<'a> {
        client: &'a super::Client,
    }

    impl<'a> CreateOauthClient<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        /// Sends a `POST` request to `/oauth/client`
        pub async fn send(self) -> Result<ResponseValue<types::OAuthClient>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/oauth/client", client.baseurl,);
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_oauth_client`]
    ///
    /// [`Client::get_oauth_client`]: super::Client::get_oauth_client
    #[derive(Debug, Clone)]
    pub struct GetOauthClient<'a> {
        client: &'a super::Client,
        client_id: Result<uuid::Uuid, String>,
    }

    impl<'a> GetOauthClient<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.client_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for client_id failed".to_string());
            self
        }

        /// Sends a `GET` request to `/oauth/client/{client_id}`
        pub async fn send(self) -> Result<ResponseValue<types::OAuthClient>, Error<types::Error>> {
            let Self { client, client_id } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_oauth_client_redirect_uri`]
    ///
    /// [`Client::create_oauth_client_redirect_uri`]: super::Client::create_oauth_client_redirect_uri
    #[derive(Debug, Clone)]
    pub struct CreateOauthClientRedirectUri<'a> {
        client: &'a super::Client,
        client_id: Result<uuid::Uuid, String>,
        body: Result<types::builder::AddOAuthClientRedirectBody, String>,
    }

    impl<'a> CreateOauthClientRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                client_id: Err("client_id was not initialized".to_string()),
                body: Ok(types::builder::AddOAuthClientRedirectBody::default()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.client_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for client_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddOAuthClientRedirectBody>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `AddOAuthClientRedirectBody` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AddOAuthClientRedirectBody,
            ) -> types::builder::AddOAuthClientRedirectBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/oauth/client/{client_id}/redirect_uri`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthClientRedirectUri>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                body,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AddOAuthClientRedirectBody>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/redirect_uri",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_oauth_client_redirect_uri`]
    ///
    /// [`Client::delete_oauth_client_redirect_uri`]: super::Client::delete_oauth_client_redirect_uri
    #[derive(Debug, Clone)]
    pub struct DeleteOauthClientRedirectUri<'a> {
        client: &'a super::Client,
        client_id: Result<uuid::Uuid, String>,
        redirect_uri_id: Result<uuid::Uuid, String>,
    }

    impl<'a> DeleteOauthClientRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                client_id: Err("client_id was not initialized".to_string()),
                redirect_uri_id: Err("redirect_uri_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.client_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for client_id failed".to_string());
            self
        }

        pub fn redirect_uri_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.redirect_uri_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for redirect_uri_id failed".to_string());
            self
        }

        /// Sends a `DELETE` request to
        /// `/oauth/client/{client_id}/redirect_uri/{redirect_uri_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthClientRedirectUri>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                redirect_uri_id,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let redirect_uri_id = redirect_uri_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/redirect_uri/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
                encode_path(&redirect_uri_id.to_string()),
            );
            let request = client
                .client
                .delete(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::create_oauth_client_secret`]
    ///
    /// [`Client::create_oauth_client_secret`]: super::Client::create_oauth_client_secret
    #[derive(Debug, Clone)]
    pub struct CreateOauthClientSecret<'a> {
        client: &'a super::Client,
        client_id: Result<uuid::Uuid, String>,
    }

    impl<'a> CreateOauthClientSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.client_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for client_id failed".to_string());
            self
        }

        /// Sends a `POST` request to `/oauth/client/{client_id}/secret`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InitialOAuthClientSecretResponse>, Error<types::Error>>
        {
            let Self { client, client_id } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/secret",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let request = client
                .client
                .post(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::delete_oauth_client_secret`]
    ///
    /// [`Client::delete_oauth_client_secret`]: super::Client::delete_oauth_client_secret
    #[derive(Debug, Clone)]
    pub struct DeleteOauthClientSecret<'a> {
        client: &'a super::Client,
        client_id: Result<uuid::Uuid, String>,
        secret_id: Result<uuid::Uuid, String>,
    }

    impl<'a> DeleteOauthClientSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                client_id: Err("client_id was not initialized".to_string()),
                secret_id: Err("secret_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.client_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for client_id failed".to_string());
            self
        }

        pub fn secret_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<uuid::Uuid>,
        {
            self.secret_id = value
                .try_into()
                .map_err(|_| "conversion to `uuid :: Uuid` for secret_id failed".to_string());
            self
        }

        /// Sends a `DELETE` request to
        /// `/oauth/client/{client_id}/secret/{secret_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OAuthClientSecret>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                secret_id,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let secret_id = secret_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/secret/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
                encode_path(&secret_id.to_string()),
            );
            let request = client
                .client
                .delete(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_rfds`]
    ///
    /// [`Client::get_rfds`]: super::Client::get_rfds
    #[derive(Debug, Clone)]
    pub struct GetRfds<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetRfds<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        /// Sends a `GET` request to `/rfd`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::ListRfd>>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/rfd", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_rfd`]
    ///
    /// [`Client::get_rfd`]: super::Client::get_rfd
    #[derive(Debug, Clone)]
    pub struct GetRfd<'a> {
        client: &'a super::Client,
        number: Result<String, String>,
    }

    impl<'a> GetRfd<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                number: Err("number was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.number = value
                .try_into()
                .map_err(|_| "conversion to `String` for number failed".to_string());
            self
        }

        /// Sends a `GET` request to `/rfd/{number}`
        pub async fn send(self) -> Result<ResponseValue<types::FullRfd>, Error<types::Error>> {
            let Self { client, number } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::search_rfds`]
    ///
    /// [`Client::search_rfds`]: super::Client::search_rfds
    #[derive(Debug, Clone)]
    pub struct SearchRfds<'a> {
        client: &'a super::Client,
        q: Result<String, String>,
    }

    impl<'a> SearchRfds<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                q: Err("q was not initialized".to_string()),
            }
        }

        pub fn q<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<String>,
        {
            self.q = value
                .try_into()
                .map_err(|_| "conversion to `String` for q failed".to_string());
            self
        }

        /// Sends a `GET` request to `/rfd-search`
        pub async fn send(self) -> Result<ResponseValue<Vec<types::ListRfd>>, Error<types::Error>> {
            let Self { client, q } = self;
            let q = q.map_err(Error::InvalidRequest)?;
            let url = format!("{}/rfd-search", client.baseurl,);
            let mut query = Vec::with_capacity(1usize);
            query.push(("q", q.to_string()));
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&query)
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }

    /// Builder for [`Client::get_self`]
    ///
    /// [`Client::get_self`]: super::Client::get_self
    #[derive(Debug, Clone)]
    pub struct GetSelf<'a> {
        client: &'a super::Client,
    }

    impl<'a> GetSelf<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client }
        }

        /// Sends a `GET` request to `/self`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetApiUserResponse>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/self", client.baseurl,);
            let request = client
                .client
                .get(url)
                .header(
                    reqwest::header::ACCEPT,
                    reqwest::header::HeaderValue::from_static("application/json"),
                )
                .build()?;
            let result = client.client.execute(request).await;
            let response = result?;
            match response.status().as_u16() {
                200u16 => ResponseValue::from_response(response).await,
                400u16..=499u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                500u16..=599u16 => Err(Error::ErrorResponse(
                    ResponseValue::from_response(response).await?,
                )),
                _ => Err(Error::UnexpectedResponse(response)),
            }
        }
    }
}

pub mod prelude {
    pub use super::Client;
    pub use super::ClientHiddenExt;
}
