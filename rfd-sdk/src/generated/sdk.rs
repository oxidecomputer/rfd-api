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
    pub struct AccessTokenProviderLogin {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub permissions: LoginPermissions,
        pub token: String,
    }

    impl From<&AccessTokenProviderLogin> for AccessTokenProviderLogin {
        fn from(value: &AccessTokenProviderLogin) -> Self {
            value.clone()
        }
    }

    impl AccessTokenProviderLogin {
        pub fn builder() -> builder::AccessTokenProviderLogin {
            builder::AccessTokenProviderLogin::default()
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
    pub enum AccessTokenProviderName {
        #[serde(rename = "github")]
        Github,
    }

    impl From<&AccessTokenProviderName> for AccessTokenProviderName {
        fn from(value: &AccessTokenProviderName) -> Self {
            value.clone()
        }
    }

    impl ToString for AccessTokenProviderName {
        fn to_string(&self) -> String {
            match *self {
                Self::Github => "github".to_string(),
            }
        }
    }

    impl std::str::FromStr for AccessTokenProviderName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "github" => Ok(Self::Github),
                _ => Err("invalid value"),
            }
        }
    }

    impl std::convert::TryFrom<&str> for AccessTokenProviderName {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for AccessTokenProviderName {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for AccessTokenProviderName {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub enum ApiPermission {
        CreateApiUserTokenSelf,
        CreateApiUserTokenAll,
        GetApiUserSelf,
        GetApiUserAll,
        GetApiUserTokenSelf,
        DeleteApiUserTokenSelf,
        DeleteApiUserTokenAll,
        CreateApiUser,
        UpdateApiUserSelf,
        UpdateApiUserAll,
        GetAllRfds,
        GetAssignedRfds,
        CreateApiUserToken(uuid::Uuid),
        GetApiUser(uuid::Uuid),
        GetApiUserToken(uuid::Uuid),
        DeleteApiUserToken(uuid::Uuid),
        UpdateApiUser(uuid::Uuid),
        GetRfd(Vec<i32>),
    }

    impl From<&ApiPermission> for ApiPermission {
        fn from(value: &ApiPermission) -> Self {
            value.clone()
        }
    }

    impl From<Vec<i32>> for ApiPermission {
        fn from(value: Vec<i32>) -> Self {
            Self::GetRfd(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserForApiPermission {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
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
    pub struct ApiUserTokenCreateParams {
        pub expires_at: chrono::DateTime<chrono::offset::Utc>,
        pub permissions: PermissionsForApiPermission,
    }

    impl From<&ApiUserTokenCreateParams> for ApiUserTokenCreateParams {
        fn from(value: &ApiUserTokenCreateParams) -> Self {
            value.clone()
        }
    }

    impl ApiUserTokenCreateParams {
        pub fn builder() -> builder::ApiUserTokenCreateParams {
            builder::ApiUserTokenCreateParams::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserTokenResponse {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        pub id: uuid::Uuid,
        pub permissions: PermissionsForApiPermission,
    }

    impl From<&ApiUserTokenResponse> for ApiUserTokenResponse {
        fn from(value: &ApiUserTokenResponse) -> Self {
            value.clone()
        }
    }

    impl ApiUserTokenResponse {
        pub fn builder() -> builder::ApiUserTokenResponse {
            builder::ApiUserTokenResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct ApiUserUpdateParams {
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
        pub labels: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub link: Option<String>,
        pub pdfs: Vec<FullRfdPdfEntry>,
        pub rfd_number: i32,
        pub sha: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: Option<String>,
        pub title: String,
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
    pub struct InitialApiUserTokenResponse {
        pub created_at: chrono::DateTime<chrono::offset::Utc>,
        pub id: uuid::Uuid,
        pub permissions: PermissionsForApiPermission,
        pub token: String,
    }

    impl From<&InitialApiUserTokenResponse> for InitialApiUserTokenResponse {
        fn from(value: &InitialApiUserTokenResponse) -> Self {
            value.clone()
        }
    }

    impl InitialApiUserTokenResponse {
        pub fn builder() -> builder::InitialApiUserTokenResponse {
            builder::InitialApiUserTokenResponse::default()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct JwtProviderLogin {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration: Option<chrono::DateTime<chrono::offset::Utc>>,
        pub permissions: LoginPermissions,
        pub token: String,
    }

    impl From<&JwtProviderLogin> for JwtProviderLogin {
        fn from(value: &JwtProviderLogin) -> Self {
            value.clone()
        }
    }

    impl JwtProviderLogin {
        pub fn builder() -> builder::JwtProviderLogin {
            builder::JwtProviderLogin::default()
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
    pub enum JwtProviderName {
        #[serde(rename = "google")]
        Google,
    }

    impl From<&JwtProviderName> for JwtProviderName {
        fn from(value: &JwtProviderName) -> Self {
            value.clone()
        }
    }

    impl ToString for JwtProviderName {
        fn to_string(&self) -> String {
            match *self {
                Self::Google => "google".to_string(),
            }
        }
    }

    impl std::str::FromStr for JwtProviderName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
                "google" => Ok(Self::Google),
                _ => Err("invalid value"),
            }
        }
    }

    impl std::convert::TryFrom<&str> for JwtProviderName {
        type Error = &'static str;
        fn try_from(value: &str) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for JwtProviderName {
        type Error = &'static str;
        fn try_from(value: &String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for JwtProviderName {
        type Error = &'static str;
        fn try_from(value: String) -> Result<Self, &'static str> {
            value.parse()
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub enum LoginPermissions {
        All,
        Specific(Vec<ApiPermission>),
    }

    impl From<&LoginPermissions> for LoginPermissions {
        fn from(value: &LoginPermissions) -> Self {
            value.clone()
        }
    }

    impl From<Vec<ApiPermission>> for LoginPermissions {
        fn from(value: Vec<ApiPermission>) -> Self {
            Self::Specific(value)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize, schemars :: JsonSchema)]
    pub struct LoginTokenResponse {
        pub token: String,
    }

    impl From<&LoginTokenResponse> for LoginTokenResponse {
        fn from(value: &LoginTokenResponse) -> Self {
            value.clone()
        }
    }

    impl LoginTokenResponse {
        pub fn builder() -> builder::LoginTokenResponse {
            builder::LoginTokenResponse::default()
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
                Self::Google => "google".to_string(),
            }
        }
    }

    impl std::str::FromStr for OAuthProviderName {
        type Err = &'static str;
        fn from_str(value: &str) -> Result<Self, &'static str> {
            match value {
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

    pub mod builder {
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
        pub struct AccessTokenProviderLogin {
            expiration: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            permissions: Result<super::LoginPermissions, String>,
            token: Result<String, String>,
        }

        impl Default for AccessTokenProviderLogin {
            fn default() -> Self {
                Self {
                    expiration: Ok(Default::default()),
                    permissions: Err("no value supplied for permissions".to_string()),
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl AccessTokenProviderLogin {
            pub fn expiration<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.expiration = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expiration: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::LoginPermissions>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
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

        impl std::convert::TryFrom<AccessTokenProviderLogin> for super::AccessTokenProviderLogin {
            type Error = String;
            fn try_from(value: AccessTokenProviderLogin) -> Result<Self, String> {
                Ok(Self {
                    expiration: value.expiration?,
                    permissions: value.permissions?,
                    token: value.token?,
                })
            }
        }

        impl From<super::AccessTokenProviderLogin> for AccessTokenProviderLogin {
            fn from(value: super::AccessTokenProviderLogin) -> Self {
                Self {
                    expiration: Ok(value.expiration),
                    permissions: Ok(value.permissions),
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserForApiPermission {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            deleted_at: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            id: Result<uuid::Uuid, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
            updated_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        }

        impl Default for ApiUserForApiPermission {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
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
                    id: Ok(value.id),
                    permissions: Ok(value.permissions),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserTokenCreateParams {
            expires_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
        }

        impl Default for ApiUserTokenCreateParams {
            fn default() -> Self {
                Self {
                    expires_at: Err("no value supplied for expires_at".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl ApiUserTokenCreateParams {
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

        impl std::convert::TryFrom<ApiUserTokenCreateParams> for super::ApiUserTokenCreateParams {
            type Error = String;
            fn try_from(value: ApiUserTokenCreateParams) -> Result<Self, String> {
                Ok(Self {
                    expires_at: value.expires_at?,
                    permissions: value.permissions?,
                })
            }
        }

        impl From<super::ApiUserTokenCreateParams> for ApiUserTokenCreateParams {
            fn from(value: super::ApiUserTokenCreateParams) -> Self {
                Self {
                    expires_at: Ok(value.expires_at),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserTokenResponse {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            id: Result<uuid::Uuid, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
        }

        impl Default for ApiUserTokenResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl ApiUserTokenResponse {
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

        impl std::convert::TryFrom<ApiUserTokenResponse> for super::ApiUserTokenResponse {
            type Error = String;
            fn try_from(value: ApiUserTokenResponse) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    permissions: value.permissions?,
                })
            }
        }

        impl From<super::ApiUserTokenResponse> for ApiUserTokenResponse {
            fn from(value: super::ApiUserTokenResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserUpdateParams {
            permissions: Result<super::PermissionsForApiPermission, String>,
        }

        impl Default for ApiUserUpdateParams {
            fn default() -> Self {
                Self {
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl ApiUserUpdateParams {
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
                    permissions: value.permissions?,
                })
            }
        }

        impl From<super::ApiUserUpdateParams> for ApiUserUpdateParams {
            fn from(value: super::ApiUserUpdateParams) -> Self {
                Self {
                    permissions: Ok(value.permissions),
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
            labels: Result<Option<String>, String>,
            link: Result<Option<String>, String>,
            pdfs: Result<Vec<super::FullRfdPdfEntry>, String>,
            rfd_number: Result<i32, String>,
            sha: Result<String, String>,
            state: Result<Option<String>, String>,
            title: Result<String, String>,
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
                    labels: Ok(Default::default()),
                    link: Ok(Default::default()),
                    pdfs: Err("no value supplied for pdfs".to_string()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                    state: Ok(Default::default()),
                    title: Err("no value supplied for title".to_string()),
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
            pub fn labels<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<String>>,
                T::Error: std::fmt::Display,
            {
                self.labels = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for labels: {}", e));
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
                    labels: value.labels?,
                    link: value.link?,
                    pdfs: value.pdfs?,
                    rfd_number: value.rfd_number?,
                    sha: value.sha?,
                    state: value.state?,
                    title: value.title?,
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
                    labels: Ok(value.labels),
                    link: Ok(value.link),
                    pdfs: Ok(value.pdfs),
                    rfd_number: Ok(value.rfd_number),
                    sha: Ok(value.sha),
                    state: Ok(value.state),
                    title: Ok(value.title),
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
        pub struct InitialApiUserTokenResponse {
            created_at: Result<chrono::DateTime<chrono::offset::Utc>, String>,
            id: Result<uuid::Uuid, String>,
            permissions: Result<super::PermissionsForApiPermission, String>,
            token: Result<String, String>,
        }

        impl Default for InitialApiUserTokenResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl InitialApiUserTokenResponse {
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

        impl std::convert::TryFrom<InitialApiUserTokenResponse> for super::InitialApiUserTokenResponse {
            type Error = String;
            fn try_from(value: InitialApiUserTokenResponse) -> Result<Self, String> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    permissions: value.permissions?,
                    token: value.token?,
                })
            }
        }

        impl From<super::InitialApiUserTokenResponse> for InitialApiUserTokenResponse {
            fn from(value: super::InitialApiUserTokenResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    permissions: Ok(value.permissions),
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct JwtProviderLogin {
            expiration: Result<Option<chrono::DateTime<chrono::offset::Utc>>, String>,
            permissions: Result<super::LoginPermissions, String>,
            token: Result<String, String>,
        }

        impl Default for JwtProviderLogin {
            fn default() -> Self {
                Self {
                    expiration: Ok(Default::default()),
                    permissions: Err("no value supplied for permissions".to_string()),
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl JwtProviderLogin {
            pub fn expiration<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<Option<chrono::DateTime<chrono::offset::Utc>>>,
                T::Error: std::fmt::Display,
            {
                self.expiration = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expiration: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: std::convert::TryInto<super::LoginPermissions>,
                T::Error: std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
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

        impl std::convert::TryFrom<JwtProviderLogin> for super::JwtProviderLogin {
            type Error = String;
            fn try_from(value: JwtProviderLogin) -> Result<Self, String> {
                Ok(Self {
                    expiration: value.expiration?,
                    permissions: value.permissions?,
                    token: value.token?,
                })
            }
        }

        impl From<super::JwtProviderLogin> for JwtProviderLogin {
            fn from(value: super::JwtProviderLogin) -> Self {
                Self {
                    expiration: Ok(value.expiration),
                    permissions: Ok(value.permissions),
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct LoginTokenResponse {
            token: Result<String, String>,
        }

        impl Default for LoginTokenResponse {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl LoginTokenResponse {
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

        impl std::convert::TryFrom<LoginTokenResponse> for super::LoginTokenResponse {
            type Error = String;
            fn try_from(value: LoginTokenResponse) -> Result<Self, String> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }

        impl From<super::LoginTokenResponse> for LoginTokenResponse {
            fn from(value: super::LoginTokenResponse) -> Self {
                Self {
                    token: Ok(value.token),
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

    /// Sends a `POST` request to `/github`
    ///
    /// ```ignore
    /// let response = client.github_webhook()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn github_webhook(&self) -> builder::GithubWebhook {
        builder::GithubWebhook::new(self)
    }

    /// Sends a `POST` request to `/login/access-token/{provider}`
    ///
    /// ```ignore
    /// let response = client.access_token_login()
    ///    .provider(provider)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn access_token_login(&self) -> builder::AccessTokenLogin {
        builder::AccessTokenLogin::new(self)
    }

    /// Sends a `POST` request to `/login/jwt/{provider}`
    ///
    /// ```ignore
    /// let response = client.jwt_login()
    ///    .provider(provider)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn jwt_login(&self) -> builder::JwtLogin {
        builder::JwtLogin::new(self)
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

pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, Error, HeaderMap, HeaderValue, RequestBuilderExt, ResponseValue,
    };
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
        ) -> Result<ResponseValue<types::ApiUserForApiPermission>, Error<types::Error>> {
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
        ) -> Result<ResponseValue<Vec<types::ApiUserTokenResponse>>, Error<types::Error>> {
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
        body: Result<types::builder::ApiUserTokenCreateParams, String>,
    }

    impl<'a> CreateApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                identifier: Err("identifier was not initialized".to_string()),
                body: Ok(types::builder::ApiUserTokenCreateParams::default()),
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
            V: std::convert::TryInto<types::ApiUserTokenCreateParams>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `ApiUserTokenCreateParams` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiUserTokenCreateParams,
            ) -> types::builder::ApiUserTokenCreateParams,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{identifier}/token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InitialApiUserTokenResponse>, Error<types::Error>>
        {
            let Self {
                client,
                identifier,
                body,
            } = self;
            let identifier = identifier.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::ApiUserTokenCreateParams>::try_into)
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
        ) -> Result<ResponseValue<types::ApiUserTokenResponse>, Error<types::Error>> {
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
        ) -> Result<ResponseValue<types::ApiUserTokenResponse>, Error<types::Error>> {
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

    /// Builder for [`Client::github_webhook`]
    ///
    /// [`Client::github_webhook`]: super::Client::github_webhook
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

    /// Builder for [`Client::access_token_login`]
    ///
    /// [`Client::access_token_login`]: super::Client::access_token_login
    #[derive(Debug, Clone)]
    pub struct AccessTokenLogin<'a> {
        client: &'a super::Client,
        provider: Result<types::AccessTokenProviderName, String>,
        body: Result<types::builder::AccessTokenProviderLogin, String>,
    }

    impl<'a> AccessTokenLogin<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                provider: Err("provider was not initialized".to_string()),
                body: Ok(types::builder::AccessTokenProviderLogin::default()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessTokenProviderName>,
        {
            self.provider = value.try_into().map_err(|_| {
                "conversion to `AccessTokenProviderName` for provider failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessTokenProviderLogin>,
        {
            self.body = value.try_into().map(From::from).map_err(|_| {
                "conversion to `AccessTokenProviderLogin` for body failed".to_string()
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AccessTokenProviderLogin,
            ) -> types::builder::AccessTokenProviderLogin,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/access-token/{provider}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::LoginTokenResponse>, Error<types::Error>> {
            let Self {
                client,
                provider,
                body,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::AccessTokenProviderLogin>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/access-token/{}",
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

    /// Builder for [`Client::jwt_login`]
    ///
    /// [`Client::jwt_login`]: super::Client::jwt_login
    #[derive(Debug, Clone)]
    pub struct JwtLogin<'a> {
        client: &'a super::Client,
        provider: Result<types::JwtProviderName, String>,
        body: Result<types::builder::JwtProviderLogin, String>,
    }

    impl<'a> JwtLogin<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client,
                provider: Err("provider was not initialized".to_string()),
                body: Ok(types::builder::JwtProviderLogin::default()),
            }
        }

        pub fn provider<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::JwtProviderName>,
        {
            self.provider = value
                .try_into()
                .map_err(|_| "conversion to `JwtProviderName` for provider failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::JwtProviderLogin>,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|_| "conversion to `JwtProviderLogin` for body failed".to_string());
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::JwtProviderLogin,
            ) -> types::builder::JwtProviderLogin,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/jwt/{provider}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::LoginTokenResponse>, Error<types::Error>> {
            let Self {
                client,
                provider,
                body,
            } = self;
            let provider = provider.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(std::convert::TryInto::<types::JwtProviderLogin>::try_into)
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/jwt/{}",
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
        ) -> Result<ResponseValue<types::ApiUserForApiPermission>, Error<types::Error>> {
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
}
