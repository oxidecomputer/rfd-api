// The contents of this file are generated; do not modify them.

#[allow(unused_imports)]
use progenitor_client::{encode_path, ClientHooks, OperationInfo, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, ClientInfo, Error, ResponseValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    /// Error types.
    pub mod error {
        /// Error from a `TryFrom` or `FromStr` implementation.
        pub struct ConversionError(::std::borrow::Cow<'static, str>);
        impl ::std::error::Error for ConversionError {}

        impl ::std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl ::std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
                ::std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    /// `AccessGroupForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "name",
    ///    "permissions",
    ///    "updated_at"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/Permissions_for_RfdPermission"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AccessGroupForRfdPermission {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForAccessGroupId,
        pub name: ::std::string::String,
        pub permissions: PermissionsForRfdPermission,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&AccessGroupForRfdPermission> for AccessGroupForRfdPermission {
        fn from(value: &AccessGroupForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl AccessGroupForRfdPermission {
        pub fn builder() -> builder::AccessGroupForRfdPermission {
            Default::default()
        }
    }

    /// `AccessGroupUpdateParamsForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "permissions"
    ///  ],
    ///  "properties": {
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/Permissions_for_RfdPermission"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AccessGroupUpdateParamsForRfdPermission {
        pub name: ::std::string::String,
        pub permissions: PermissionsForRfdPermission,
    }

    impl ::std::convert::From<&AccessGroupUpdateParamsForRfdPermission>
        for AccessGroupUpdateParamsForRfdPermission
    {
        fn from(value: &AccessGroupUpdateParamsForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl AccessGroupUpdateParamsForRfdPermission {
        pub fn builder() -> builder::AccessGroupUpdateParamsForRfdPermission {
            Default::default()
        }
    }

    /// `AccessTokenExchangeRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "device_code",
    ///    "grant_type"
    ///  ],
    ///  "properties": {
    ///    "device_code": {
    ///      "type": "string"
    ///    },
    ///    "expires_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "grant_type": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AccessTokenExchangeRequest {
        pub device_code: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub grant_type: ::std::string::String,
    }

    impl ::std::convert::From<&AccessTokenExchangeRequest> for AccessTokenExchangeRequest {
        fn from(value: &AccessTokenExchangeRequest) -> Self {
            value.clone()
        }
    }

    impl AccessTokenExchangeRequest {
        pub fn builder() -> builder::AccessTokenExchangeRequest {
            Default::default()
        }
    }

    /// `AddGroupBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "group_id"
    ///  ],
    ///  "properties": {
    ///    "group_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AddGroupBody {
        pub group_id: TypedUuidForAccessGroupId,
    }

    impl ::std::convert::From<&AddGroupBody> for AddGroupBody {
        fn from(value: &AddGroupBody) -> Self {
            value.clone()
        }
    }

    impl AddGroupBody {
        pub fn builder() -> builder::AddGroupBody {
            Default::default()
        }
    }

    /// `AddMagicLinkRedirectBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AddMagicLinkRedirectBody {
        pub redirect_uri: ::std::string::String,
    }

    impl ::std::convert::From<&AddMagicLinkRedirectBody> for AddMagicLinkRedirectBody {
        fn from(value: &AddMagicLinkRedirectBody) -> Self {
            value.clone()
        }
    }

    impl AddMagicLinkRedirectBody {
        pub fn builder() -> builder::AddMagicLinkRedirectBody {
            Default::default()
        }
    }

    /// `AddOAuthClientRedirectBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct AddOAuthClientRedirectBody {
        pub redirect_uri: ::std::string::String,
    }

    impl ::std::convert::From<&AddOAuthClientRedirectBody> for AddOAuthClientRedirectBody {
        fn from(value: &AddOAuthClientRedirectBody) -> Self {
            value.clone()
        }
    }

    impl AddOAuthClientRedirectBody {
        pub fn builder() -> builder::AddOAuthClientRedirectBody {
            Default::default()
        }
    }

    /// `ApiKeyCreateParamsForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "expires_at"
    ///  ],
    ///  "properties": {
    ///    "expires_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "permissions": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/Permissions_for_RfdPermission"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiKeyCreateParamsForRfdPermission {
        pub expires_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub permissions: ::std::option::Option<PermissionsForRfdPermission>,
    }

    impl ::std::convert::From<&ApiKeyCreateParamsForRfdPermission>
        for ApiKeyCreateParamsForRfdPermission
    {
        fn from(value: &ApiKeyCreateParamsForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl ApiKeyCreateParamsForRfdPermission {
        pub fn builder() -> builder::ApiKeyCreateParamsForRfdPermission {
            Default::default()
        }
    }

    /// `ApiKeyResponseForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///    },
    ///    "permissions": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/Permissions_for_RfdPermission"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiKeyResponseForRfdPermission {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: TypedUuidForApiKeyId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub permissions: ::std::option::Option<PermissionsForRfdPermission>,
    }

    impl ::std::convert::From<&ApiKeyResponseForRfdPermission> for ApiKeyResponseForRfdPermission {
        fn from(value: &ApiKeyResponseForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl ApiKeyResponseForRfdPermission {
        pub fn builder() -> builder::ApiKeyResponseForRfdPermission {
            Default::default()
        }
    }

    /// `ApiUserContactEmail`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "email",
    ///    "id",
    ///    "updated_at",
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserProviderId"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "user_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserContactEmail {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub email: ::std::string::String,
        pub id: TypedUuidForUserProviderId,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub user_id: TypedUuidForUserId,
    }

    impl ::std::convert::From<&ApiUserContactEmail> for ApiUserContactEmail {
        fn from(value: &ApiUserContactEmail) -> Self {
            value.clone()
        }
    }

    impl ApiUserContactEmail {
        pub fn builder() -> builder::ApiUserContactEmail {
            Default::default()
        }
    }

    /// `ApiUserEmailUpdateParams`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "email"
    ///  ],
    ///  "properties": {
    ///    "email": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserEmailUpdateParams {
        pub email: ::std::string::String,
    }

    impl ::std::convert::From<&ApiUserEmailUpdateParams> for ApiUserEmailUpdateParams {
        fn from(value: &ApiUserEmailUpdateParams) -> Self {
            value.clone()
        }
    }

    impl ApiUserEmailUpdateParams {
        pub fn builder() -> builder::ApiUserEmailUpdateParams {
            Default::default()
        }
    }

    /// `ApiUserForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "groups",
    ///    "id",
    ///    "permissions",
    ///    "updated_at"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserId"
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/Permissions_for_RfdPermission"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserForRfdPermission {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub groups: Vec<TypedUuidForAccessGroupId>,
        pub id: TypedUuidForUserId,
        pub permissions: PermissionsForRfdPermission,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&ApiUserForRfdPermission> for ApiUserForRfdPermission {
        fn from(value: &ApiUserForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl ApiUserForRfdPermission {
        pub fn builder() -> builder::ApiUserForRfdPermission {
            Default::default()
        }
    }

    /// `ApiUserLinkRequestPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "user_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserLinkRequestPayload {
        pub user_id: TypedUuidForUserId,
    }

    impl ::std::convert::From<&ApiUserLinkRequestPayload> for ApiUserLinkRequestPayload {
        fn from(value: &ApiUserLinkRequestPayload) -> Self {
            value.clone()
        }
    }

    impl ApiUserLinkRequestPayload {
        pub fn builder() -> builder::ApiUserLinkRequestPayload {
            Default::default()
        }
    }

    /// `ApiUserLinkRequestResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "token": {
    ///      "$ref": "#/components/schemas/SecretString"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserLinkRequestResponse {
        pub token: SecretString,
    }

    impl ::std::convert::From<&ApiUserLinkRequestResponse> for ApiUserLinkRequestResponse {
        fn from(value: &ApiUserLinkRequestResponse) -> Self {
            value.clone()
        }
    }

    impl ApiUserLinkRequestResponse {
        pub fn builder() -> builder::ApiUserLinkRequestResponse {
            Default::default()
        }
    }

    /// `ApiUserProvider`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "display_names",
    ///    "emails",
    ///    "id",
    ///    "provider",
    ///    "provider_id",
    ///    "updated_at",
    ///    "user_id"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "display_names": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }

    ///    },
    ///    "emails": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }

    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserProviderId"
    ///    },
    ///    "provider": {
    ///      "type": "string"
    ///    },
    ///    "provider_id": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "user_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForUserId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserProvider {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub display_names: ::std::vec::Vec<::std::string::String>,
        pub emails: ::std::vec::Vec<::std::string::String>,
        pub id: TypedUuidForUserProviderId,
        pub provider: ::std::string::String,
        pub provider_id: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub user_id: TypedUuidForUserId,
    }

    impl ::std::convert::From<&ApiUserProvider> for ApiUserProvider {
        fn from(value: &ApiUserProvider) -> Self {
            value.clone()
        }
    }

    impl ApiUserProvider {
        pub fn builder() -> builder::ApiUserProvider {
            Default::default()
        }
    }

    /// `ApiUserProviderLinkPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "token"
    ///  ],
    ///  "properties": {
    ///    "token": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserProviderLinkPayload {
        pub token: ::std::string::String,
    }

    impl ::std::convert::From<&ApiUserProviderLinkPayload> for ApiUserProviderLinkPayload {
        fn from(value: &ApiUserProviderLinkPayload) -> Self {
            value.clone()
        }
    }

    impl ApiUserProviderLinkPayload {
        pub fn builder() -> builder::ApiUserProviderLinkPayload {
            Default::default()
        }
    }

    /// `ApiUserUpdateParamsForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "group_ids",
    ///    "permissions"
    ///  ],
    ///  "properties": {
    ///    "group_ids": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/Permissions_for_RfdPermission"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ApiUserUpdateParamsForRfdPermission {
        pub group_ids: Vec<TypedUuidForAccessGroupId>,
        pub permissions: PermissionsForRfdPermission,
    }

    impl ::std::convert::From<&ApiUserUpdateParamsForRfdPermission>
        for ApiUserUpdateParamsForRfdPermission
    {
        fn from(value: &ApiUserUpdateParamsForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl ApiUserUpdateParamsForRfdPermission {
        pub fn builder() -> builder::ApiUserUpdateParamsForRfdPermission {
            Default::default()
        }
    }

    /// `CommitSha`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct CommitSha(pub ::std::string::String);
    impl ::std::ops::Deref for CommitSha {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<CommitSha> for ::std::string::String {
        fn from(value: CommitSha) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&CommitSha> for CommitSha {
        fn from(value: &CommitSha) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for CommitSha {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for CommitSha {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for CommitSha {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `ContentFormat`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "asciidoc",
    ///    "markdown"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum ContentFormat {
        #[serde(rename = "asciidoc")]
        Asciidoc,
        #[serde(rename = "markdown")]
        Markdown,
    }

    impl ::std::convert::From<&Self> for ContentFormat {
        fn from(value: &ContentFormat) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for ContentFormat {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Asciidoc => write!(f, "asciidoc"),
                Self::Markdown => write!(f, "markdown"),
            }
        }
    }

    impl ::std::str::FromStr for ContentFormat {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "asciidoc" => Ok(Self::Asciidoc),
                "markdown" => Ok(Self::Markdown),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for ContentFormat {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for ContentFormat {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for ContentFormat {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// `CreateMapper`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "name",
    ///    "rule"
    ///  ],
    ///  "properties": {
    ///    "max_activations": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "rule": {}

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct CreateMapper {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_activations: ::std::option::Option<i32>,
        pub name: ::std::string::String,
        pub rule: ::serde_json::Value,
    }

    impl ::std::convert::From<&CreateMapper> for CreateMapper {
        fn from(value: &CreateMapper) -> Self {
            value.clone()
        }
    }

    impl CreateMapper {
        pub fn builder() -> builder::CreateMapper {
            Default::default()
        }
    }

    /// Error information from a response.
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "description": "Error information from a response.",
    ///  "type": "object",
    ///  "required": [
    ///    "message",
    ///    "request_id"
    ///  ],
    ///  "properties": {
    ///    "error_code": {
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    },
    ///    "request_id": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Error {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub error_code: ::std::option::Option<::std::string::String>,
        pub message: ::std::string::String,
        pub request_id: ::std::string::String,
    }

    impl ::std::convert::From<&Error> for Error {
        fn from(value: &Error) -> Self {
            value.clone()
        }
    }

    impl Error {
        pub fn builder() -> builder::Error {
            Default::default()
        }
    }

    /// `FileSha`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct FileSha(pub ::std::string::String);
    impl ::std::ops::Deref for FileSha {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<FileSha> for ::std::string::String {
        fn from(value: FileSha) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&FileSha> for FileSha {
        fn from(value: &FileSha) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for FileSha {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for FileSha {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for FileSha {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `FormattedSearchResultHit`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "hierarchy",
    ///    "hierarchy_radio",
    ///    "object_id",
    ///    "rfd_number"
    ///  ],
    ///  "properties": {
    ///    "anchor": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "content": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "hierarchy": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": [
    ///          "string",
    ///          "null"
    ///        ]
    ///      },
    ///      "maxItems": 6,
    ///      "minItems": 6
    ///    },
    ///    "hierarchy_radio": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": [
    ///          "string",
    ///          "null"
    ///        ]
    ///      },
    ///      "maxItems": 6,
    ///      "minItems": 6
    ///    },
    ///    "object_id": {
    ///      "type": "string"
    ///    },
    ///    "rfd_number": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "url": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct FormattedSearchResultHit {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub anchor: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        pub hierarchy: [::std::option::Option<::std::string::String>; 6usize],
        pub hierarchy_radio: [::std::option::Option<::std::string::String>; 6usize],
        pub object_id: ::std::string::String,
        pub rfd_number: u64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&FormattedSearchResultHit> for FormattedSearchResultHit {
        fn from(value: &FormattedSearchResultHit) -> Self {
            value.clone()
        }
    }

    impl FormattedSearchResultHit {
        pub fn builder() -> builder::FormattedSearchResultHit {
            Default::default()
        }
    }

    /// `GetUserResponseForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "info",
    ///    "providers"
    ///  ],
    ///  "properties": {
    ///    "info": {
    ///      "$ref": "#/components/schemas/ApiUser_for_RfdPermission"
    ///    },
    ///    "providers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ApiUserProvider"
    ///      }

    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct GetUserResponseForRfdPermission {
        pub info: ApiUserForRfdPermission,
        pub providers: ::std::vec::Vec<ApiUserProvider>,
    }

    impl ::std::convert::From<&GetUserResponseForRfdPermission> for GetUserResponseForRfdPermission {
        fn from(value: &GetUserResponseForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl GetUserResponseForRfdPermission {
        pub fn builder() -> builder::GetUserResponseForRfdPermission {
            Default::default()
        }
    }

    /// `GitHubCommit`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "added",
    ///    "id",
    ///    "modified",
    ///    "removed",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "added": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }

    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "modified": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }

    ///    },
    ///    "removed": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }

    ///    },
    ///    "timestamp": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct GitHubCommit {
        pub added: ::std::vec::Vec<::std::string::String>,
        pub id: ::std::string::String,
        pub modified: ::std::vec::Vec<::std::string::String>,
        pub removed: ::std::vec::Vec<::std::string::String>,
        pub timestamp: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&GitHubCommit> for GitHubCommit {
        fn from(value: &GitHubCommit) -> Self {
            value.clone()
        }
    }

    impl GitHubCommit {
        pub fn builder() -> builder::GitHubCommit {
            Default::default()
        }
    }

    /// `GitHubCommitPayload`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "commits",
    ///    "installation",
    ///    "ref",
    ///    "repository",
    ///    "sender"
    ///  ],
    ///  "properties": {
    ///    "commits": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/GitHubCommit"
    ///      }

    ///    },
    ///    "head_commit": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/GitHubCommit"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "installation": {
    ///      "$ref": "#/components/schemas/GitHubInstallation"
    ///    },
    ///    "ref": {
    ///      "type": "string"
    ///    },
    ///    "repository": {
    ///      "$ref": "#/components/schemas/GitHubRepository"
    ///    },
    ///    "sender": {
    ///      "$ref": "#/components/schemas/GitHubSender"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct GitHubCommitPayload {
        pub commits: ::std::vec::Vec<GitHubCommit>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub head_commit: ::std::option::Option<GitHubCommit>,
        pub installation: GitHubInstallation,
        #[serde(rename = "ref")]
        pub ref_: ::std::string::String,
        pub repository: GitHubRepository,
        pub sender: GitHubSender,
    }

    impl ::std::convert::From<&GitHubCommitPayload> for GitHubCommitPayload {
        fn from(value: &GitHubCommitPayload) -> Self {
            value.clone()
        }
    }

    impl GitHubCommitPayload {
        pub fn builder() -> builder::GitHubCommitPayload {
            Default::default()
        }
    }

    /// `GitHubInstallation`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "node_id"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "node_id": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct GitHubInstallation {
        pub id: u64,
        pub node_id: ::std::string::String,
    }

    impl ::std::convert::From<&GitHubInstallation> for GitHubInstallation {
        fn from(value: &GitHubInstallation) -> Self {
            value.clone()
        }
    }

    impl GitHubInstallation {
        pub fn builder() -> builder::GitHubInstallation {
            Default::default()
        }
    }

    /// `GitHubRepository`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "default_branch",
    ///    "id",
    ///    "name",
    ///    "node_id",
    ///    "owner"
    ///  ],
    ///  "properties": {
    ///    "default_branch": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "node_id": {
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "$ref": "#/components/schemas/GitHubRepositoryOwner"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct GitHubRepository {
        pub default_branch: ::std::string::String,
        pub id: u64,
        pub name: ::std::string::String,
        pub node_id: ::std::string::String,
        pub owner: GitHubRepositoryOwner,
    }

    impl ::std::convert::From<&GitHubRepository> for GitHubRepository {
        fn from(value: &GitHubRepository) -> Self {
            value.clone()
        }
    }

    impl GitHubRepository {
        pub fn builder() -> builder::GitHubRepository {
            Default::default()
        }
    }

    /// `GitHubRepositoryOwner`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "login"
    ///  ],
    ///  "properties": {
    ///    "login": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct GitHubRepositoryOwner {
        pub login: ::std::string::String,
    }

    impl ::std::convert::From<&GitHubRepositoryOwner> for GitHubRepositoryOwner {
        fn from(value: &GitHubRepositoryOwner) -> Self {
            value.clone()
        }
    }

    impl GitHubRepositoryOwner {
        pub fn builder() -> builder::GitHubRepositoryOwner {
            Default::default()
        }
    }

    /// `GitHubSender`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "login",
    ///    "node_id",
    ///    "type"
    ///  ],
    ///  "properties": {
    ///    "id": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "login": {
    ///      "type": "string"
    ///    },
    ///    "node_id": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct GitHubSender {
        pub id: u64,
        pub login: ::std::string::String,
        pub node_id: ::std::string::String,
        #[serde(rename = "type")]
        pub type_: ::std::string::String,
    }

    impl ::std::convert::From<&GitHubSender> for GitHubSender {
        fn from(value: &GitHubSender) -> Self {
            value.clone()
        }
    }

    impl GitHubSender {
        pub fn builder() -> builder::GitHubSender {
            Default::default()
        }
    }

    /// `InitialApiKeyResponseForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "key"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///    },
    ///    "key": {
    ///      "$ref": "#/components/schemas/SecretString"
    ///    },
    ///    "permissions": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/Permissions_for_RfdPermission"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InitialApiKeyResponseForRfdPermission {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: TypedUuidForApiKeyId,
        pub key: SecretString,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub permissions: ::std::option::Option<PermissionsForRfdPermission>,
    }

    impl ::std::convert::From<&InitialApiKeyResponseForRfdPermission>
        for InitialApiKeyResponseForRfdPermission
    {
        fn from(value: &InitialApiKeyResponseForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl InitialApiKeyResponseForRfdPermission {
        pub fn builder() -> builder::InitialApiKeyResponseForRfdPermission {
            Default::default()
        }
    }

    /// `InitialMagicLinkSecretResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "key"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkSecretId"
    ///    },
    ///    "key": {
    ///      "$ref": "#/components/schemas/SecretString"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InitialMagicLinkSecretResponse {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: TypedUuidForMagicLinkSecretId,
        pub key: SecretString,
    }

    impl ::std::convert::From<&InitialMagicLinkSecretResponse> for InitialMagicLinkSecretResponse {
        fn from(value: &InitialMagicLinkSecretResponse) -> Self {
            value.clone()
        }
    }

    impl InitialMagicLinkSecretResponse {
        pub fn builder() -> builder::InitialMagicLinkSecretResponse {
            Default::default()
        }
    }

    /// `InitialOAuthClientSecretResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "key"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthSecretId"
    ///    },
    ///    "key": {
    ///      "$ref": "#/components/schemas/SecretString"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct InitialOAuthClientSecretResponse {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: TypedUuidForOAuthSecretId,
        pub key: SecretString,
    }

    impl ::std::convert::From<&InitialOAuthClientSecretResponse> for InitialOAuthClientSecretResponse {
        fn from(value: &InitialOAuthClientSecretResponse) -> Self {
            value.clone()
        }
    }

    impl InitialOAuthClientSecretResponse {
        pub fn builder() -> builder::InitialOAuthClientSecretResponse {
            Default::default()
        }
    }

    /// `Job`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "branch",
    ///    "committed_at",
    ///    "created_at",
    ///    "id",
    ///    "owner",
    ///    "processed",
    ///    "repository",
    ///    "rfd",
    ///    "sha"
    ///  ],
    ///  "properties": {
    ///    "branch": {
    ///      "type": "string"
    ///    },
    ///    "committed_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "owner": {
    ///      "type": "string"
    ///    },
    ///    "processed": {
    ///      "type": "boolean"
    ///    },
    ///    "repository": {
    ///      "type": "string"
    ///    },
    ///    "rfd": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "sha": {
    ///      "$ref": "#/components/schemas/CommitSha"
    ///    },
    ///    "started_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "webhook_delivery_id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref":
    /// "#/components/schemas/TypedUuidForWebhookDeliveryId"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Job {
        pub branch: ::std::string::String,
        pub committed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub id: i32,
        pub owner: ::std::string::String,
        pub processed: bool,
        pub repository: ::std::string::String,
        pub rfd: i32,
        pub sha: CommitSha,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub started_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub webhook_delivery_id: ::std::option::Option<TypedUuidForWebhookDeliveryId>,
    }

    impl ::std::convert::From<&Job> for Job {
        fn from(value: &Job) -> Self {
            value.clone()
        }
    }

    impl Job {
        pub fn builder() -> builder::Job {
            Default::default()
        }
    }

    /// `Jwk`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "e",
    ///    "kid",
    ///    "kty",
    ///    "n",
    ///    "use"
    ///  ],
    ///  "properties": {
    ///    "e": {
    ///      "type": "string"
    ///    },
    ///    "kid": {
    ///      "type": "string"
    ///    },
    ///    "kty": {
    ///      "type": "string"
    ///    },
    ///    "n": {
    ///      "type": "string"
    ///    },
    ///    "use": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Jwk {
        pub e: ::std::string::String,
        pub kid: ::std::string::String,
        pub kty: ::std::string::String,
        pub n: ::std::string::String,
        #[serde(rename = "use")]
        pub use_: ::std::string::String,
    }

    impl ::std::convert::From<&Jwk> for Jwk {
        fn from(value: &Jwk) -> Self {
            value.clone()
        }
    }

    impl Jwk {
        pub fn builder() -> builder::Jwk {
            Default::default()
        }
    }

    /// `Jwks`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "keys"
    ///  ],
    ///  "properties": {
    ///    "keys": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Jwk"
    ///      }

    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Jwks {
        pub keys: ::std::vec::Vec<Jwk>,
    }

    impl ::std::convert::From<&Jwks> for Jwks {
        fn from(value: &Jwks) -> Self {
            value.clone()
        }
    }

    impl Jwks {
        pub fn builder() -> builder::Jwks {
            Default::default()
        }
    }

    /// `MagicLink`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "redirect_uris",
    ///    "secrets"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///    },
    ///    "redirect_uris": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MagicLinkRedirectUri"
    ///      }

    ///    },
    ///    "secrets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MagicLinkSecret"
    ///      }

    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLink {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForMagicLinkId,
        pub redirect_uris: ::std::vec::Vec<MagicLinkRedirectUri>,
        pub secrets: ::std::vec::Vec<MagicLinkSecret>,
    }

    impl ::std::convert::From<&MagicLink> for MagicLink {
        fn from(value: &MagicLink) -> Self {
            value.clone()
        }
    }

    impl MagicLink {
        pub fn builder() -> builder::MagicLink {
            Default::default()
        }
    }

    /// `MagicLinkExchangeRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "attempt_id",
    ///    "recipient",
    ///    "secret"
    ///  ],
    ///  "properties": {
    ///    "attempt_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkAttemptId"
    ///    },
    ///    "recipient": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkExchangeRequest {
        pub attempt_id: TypedUuidForMagicLinkAttemptId,
        pub recipient: ::std::string::String,
        pub secret: ::std::string::String,
    }

    impl ::std::convert::From<&MagicLinkExchangeRequest> for MagicLinkExchangeRequest {
        fn from(value: &MagicLinkExchangeRequest) -> Self {
            value.clone()
        }
    }

    impl MagicLinkExchangeRequest {
        pub fn builder() -> builder::MagicLinkExchangeRequest {
            Default::default()
        }
    }

    /// `MagicLinkExchangeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "access_token",
    ///    "expires_in",
    ///    "token_type"
    ///  ],
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "expires_in": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "token_type": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkExchangeResponse {
        pub access_token: ::std::string::String,
        pub expires_in: i64,
        pub token_type: ::std::string::String,
    }

    impl ::std::convert::From<&MagicLinkExchangeResponse> for MagicLinkExchangeResponse {
        fn from(value: &MagicLinkExchangeResponse) -> Self {
            value.clone()
        }
    }

    impl MagicLinkExchangeResponse {
        pub fn builder() -> builder::MagicLinkExchangeResponse {
            Default::default()
        }
    }

    /// `MagicLinkMedium`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "email"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum MagicLinkMedium {
        #[serde(rename = "email")]
        Email,
    }

    impl ::std::convert::From<&Self> for MagicLinkMedium {
        fn from(value: &MagicLinkMedium) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for MagicLinkMedium {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Email => write!(f, "email"),
            }
        }
    }

    impl ::std::str::FromStr for MagicLinkMedium {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "email" => Ok(Self::Email),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for MagicLinkMedium {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for MagicLinkMedium {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for MagicLinkMedium {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// `MagicLinkRedirectUri`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "magic_link_client_id",
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkRedirectUriId"
    ///    },
    ///    "magic_link_client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///    },
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkRedirectUri {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForMagicLinkRedirectUriId,
        pub magic_link_client_id: TypedUuidForMagicLinkId,
        pub redirect_uri: ::std::string::String,
    }

    impl ::std::convert::From<&MagicLinkRedirectUri> for MagicLinkRedirectUri {
        fn from(value: &MagicLinkRedirectUri) -> Self {
            value.clone()
        }
    }

    impl MagicLinkRedirectUri {
        pub fn builder() -> builder::MagicLinkRedirectUri {
            Default::default()
        }
    }

    /// `MagicLinkSecret`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "magic_link_client_id",
    ///    "secret_signature"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkSecretId"
    ///    },
    ///    "magic_link_client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///    },
    ///    "secret_signature": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkSecret {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForMagicLinkSecretId,
        pub magic_link_client_id: TypedUuidForMagicLinkId,
        pub secret_signature: ::std::string::String,
    }

    impl ::std::convert::From<&MagicLinkSecret> for MagicLinkSecret {
        fn from(value: &MagicLinkSecret) -> Self {
            value.clone()
        }
    }

    impl MagicLinkSecret {
        pub fn builder() -> builder::MagicLinkSecret {
            Default::default()
        }
    }

    /// `MagicLinkSendRequest`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "expires_in",
    ///    "medium",
    ///    "recipient",
    ///    "redirect_uri",
    ///    "secret"
    ///  ],
    ///  "properties": {
    ///    "expires_in": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "medium": {
    ///      "$ref": "#/components/schemas/MagicLinkMedium"
    ///    },
    ///    "recipient": {
    ///      "type": "string"
    ///    },
    ///    "redirect_uri": {
    ///      "type": "string",
    ///      "format": "uri"
    ///    },
    ///    "scope": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkSendRequest {
        pub expires_in: i64,
        pub medium: MagicLinkMedium,
        pub recipient: ::std::string::String,
        pub redirect_uri: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub scope: ::std::option::Option<::std::string::String>,
        pub secret: ::std::string::String,
    }

    impl ::std::convert::From<&MagicLinkSendRequest> for MagicLinkSendRequest {
        fn from(value: &MagicLinkSendRequest) -> Self {
            value.clone()
        }
    }

    impl MagicLinkSendRequest {
        pub fn builder() -> builder::MagicLinkSendRequest {
            Default::default()
        }
    }

    /// `MagicLinkSendResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "attempt_id"
    ///  ],
    ///  "properties": {
    ///    "attempt_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMagicLinkAttemptId"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct MagicLinkSendResponse {
        pub attempt_id: TypedUuidForMagicLinkAttemptId,
    }

    impl ::std::convert::From<&MagicLinkSendResponse> for MagicLinkSendResponse {
        fn from(value: &MagicLinkSendResponse) -> Self {
            value.clone()
        }
    }

    impl MagicLinkSendResponse {
        pub fn builder() -> builder::MagicLinkSendResponse {
            Default::default()
        }
    }

    /// `Mapper`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "name",
    ///    "rule",
    ///    "updated_at"
    ///  ],
    ///  "properties": {
    ///    "activations": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "depleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForMapperId"
    ///    },
    ///    "max_activations": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "int32"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "rule": {},
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Mapper {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub activations: ::std::option::Option<i32>,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub depleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForMapperId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub max_activations: ::std::option::Option<i32>,
        pub name: ::std::string::String,
        pub rule: ::serde_json::Value,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&Mapper> for Mapper {
        fn from(value: &Mapper) -> Self {
            value.clone()
        }
    }

    impl Mapper {
        pub fn builder() -> builder::Mapper {
            Default::default()
        }
    }

    /// `OAuthAuthzCodeExchangeBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "grant_type",
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "client_id": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "client_secret": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/SecretString"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "grant_type": {
    ///      "type": "string"
    ///    },
    ///    "pkce_verifier": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthAuthzCodeExchangeBody {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub client_id: ::std::option::Option<TypedUuidForOAuthClientId>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub client_secret: ::std::option::Option<SecretString>,
        pub code: ::std::string::String,
        pub grant_type: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub pkce_verifier: ::std::option::Option<::std::string::String>,
        pub redirect_uri: ::std::string::String,
    }

    impl ::std::convert::From<&OAuthAuthzCodeExchangeBody> for OAuthAuthzCodeExchangeBody {
        fn from(value: &OAuthAuthzCodeExchangeBody) -> Self {
            value.clone()
        }
    }

    impl OAuthAuthzCodeExchangeBody {
        pub fn builder() -> builder::OAuthAuthzCodeExchangeBody {
            Default::default()
        }
    }

    /// `OAuthAuthzCodeExchangeResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "access_token",
    ///    "expires_in",
    ///    "token_type"
    ///  ],
    ///  "properties": {
    ///    "access_token": {
    ///      "type": "string"
    ///    },
    ///    "expires_in": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "token_type": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthAuthzCodeExchangeResponse {
        pub access_token: ::std::string::String,
        pub expires_in: i64,
        pub token_type: ::std::string::String,
    }

    impl ::std::convert::From<&OAuthAuthzCodeExchangeResponse> for OAuthAuthzCodeExchangeResponse {
        fn from(value: &OAuthAuthzCodeExchangeResponse) -> Self {
            value.clone()
        }
    }

    impl OAuthAuthzCodeExchangeResponse {
        pub fn builder() -> builder::OAuthAuthzCodeExchangeResponse {
            Default::default()
        }
    }

    /// `OAuthClient`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "redirect_uris",
    ///    "secrets"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "redirect_uris": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OAuthClientRedirectUri"
    ///      }

    ///    },
    ///    "secrets": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OAuthClientSecret"
    ///      }

    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthClient {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForOAuthClientId,
        pub redirect_uris: ::std::vec::Vec<OAuthClientRedirectUri>,
        pub secrets: ::std::vec::Vec<OAuthClientSecret>,
    }

    impl ::std::convert::From<&OAuthClient> for OAuthClient {
        fn from(value: &OAuthClient) -> Self {
            value.clone()
        }
    }

    impl OAuthClient {
        pub fn builder() -> builder::OAuthClient {
            Default::default()
        }
    }

    /// `OAuthClientRedirectUri`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "oauth_client_id",
    ///    "redirect_uri"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthRedirectUriId"
    ///    },
    ///    "oauth_client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "redirect_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthClientRedirectUri {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForOAuthRedirectUriId,
        pub oauth_client_id: TypedUuidForOAuthClientId,
        pub redirect_uri: ::std::string::String,
    }

    impl ::std::convert::From<&OAuthClientRedirectUri> for OAuthClientRedirectUri {
        fn from(value: &OAuthClientRedirectUri) -> Self {
            value.clone()
        }
    }

    impl OAuthClientRedirectUri {
        pub fn builder() -> builder::OAuthClientRedirectUri {
            Default::default()
        }
    }

    /// `OAuthClientSecret`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "oauth_client_id",
    ///    "secret_signature"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthSecretId"
    ///    },
    ///    "oauth_client_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///    },
    ///    "secret_signature": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthClientSecret {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForOAuthSecretId,
        pub oauth_client_id: TypedUuidForOAuthClientId,
        pub secret_signature: ::std::string::String,
    }

    impl ::std::convert::From<&OAuthClientSecret> for OAuthClientSecret {
        fn from(value: &OAuthClientSecret) -> Self {
            value.clone()
        }
    }

    impl OAuthClientSecret {
        pub fn builder() -> builder::OAuthClientSecret {
            Default::default()
        }
    }

    /// `OAuthProviderInfo`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "auth_url_endpoint",
    ///    "client_id",
    ///    "device_code_endpoint",
    ///    "provider",
    ///    "scopes",
    ///    "token_endpoint"
    ///  ],
    ///  "properties": {
    ///    "auth_url_endpoint": {
    ///      "type": "string"
    ///    },
    ///    "client_id": {
    ///      "type": "string"
    ///    },
    ///    "device_code_endpoint": {
    ///      "type": "string"
    ///    },
    ///    "provider": {
    ///      "$ref": "#/components/schemas/OAuthProviderName"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }

    ///    },
    ///    "token_endpoint": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OAuthProviderInfo {
        pub auth_url_endpoint: ::std::string::String,
        pub client_id: ::std::string::String,
        pub device_code_endpoint: ::std::string::String,
        pub provider: OAuthProviderName,
        pub scopes: ::std::vec::Vec<::std::string::String>,
        pub token_endpoint: ::std::string::String,
    }

    impl ::std::convert::From<&OAuthProviderInfo> for OAuthProviderInfo {
        fn from(value: &OAuthProviderInfo) -> Self {
            value.clone()
        }
    }

    impl OAuthProviderInfo {
        pub fn builder() -> builder::OAuthProviderInfo {
            Default::default()
        }
    }

    /// `OAuthProviderName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "github",
    ///    "google"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum OAuthProviderName {
        #[serde(rename = "github")]
        Github,
        #[serde(rename = "google")]
        Google,
    }

    impl ::std::convert::From<&Self> for OAuthProviderName {
        fn from(value: &OAuthProviderName) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for OAuthProviderName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Github => write!(f, "github"),
                Self::Google => write!(f, "google"),
            }
        }
    }

    impl ::std::str::FromStr for OAuthProviderName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "github" => Ok(Self::Github),
                "google" => Ok(Self::Google),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for OAuthProviderName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for OAuthProviderName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for OAuthProviderName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// `OpenIdConfiguration`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "jwks_uri"
    ///  ],
    ///  "properties": {
    ///    "jwks_uri": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct OpenIdConfiguration {
        pub jwks_uri: ::std::string::String,
    }

    impl ::std::convert::From<&OpenIdConfiguration> for OpenIdConfiguration {
        fn from(value: &OpenIdConfiguration) -> Self {
            value.clone()
        }
    }

    impl OpenIdConfiguration {
        pub fn builder() -> builder::OpenIdConfiguration {
            Default::default()
        }
    }

    /// `PdfSource`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "github",
    ///    "google"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum PdfSource {
        #[serde(rename = "github")]
        Github,
        #[serde(rename = "google")]
        Google,
    }

    impl ::std::convert::From<&Self> for PdfSource {
        fn from(value: &PdfSource) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for PdfSource {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Github => write!(f, "github"),
                Self::Google => write!(f, "google"),
            }
        }
    }

    impl ::std::str::FromStr for PdfSource {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "github" => Ok(Self::Github),
                "google" => Ok(Self::Google),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for PdfSource {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for PdfSource {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for PdfSource {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// `PermissionsForRfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "array",
    ///  "items": {
    ///    "$ref": "#/components/schemas/RfdPermission"
    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct PermissionsForRfdPermission(pub ::std::vec::Vec<RfdPermission>);
    impl ::std::ops::Deref for PermissionsForRfdPermission {
        type Target = ::std::vec::Vec<RfdPermission>;
        fn deref(&self) -> &::std::vec::Vec<RfdPermission> {
            &self.0
        }
    }

    impl ::std::convert::From<PermissionsForRfdPermission> for ::std::vec::Vec<RfdPermission> {
        fn from(value: PermissionsForRfdPermission) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&PermissionsForRfdPermission> for PermissionsForRfdPermission {
        fn from(value: &PermissionsForRfdPermission) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::vec::Vec<RfdPermission>> for PermissionsForRfdPermission {
        fn from(value: ::std::vec::Vec<RfdPermission>) -> Self {
            Self(value)
        }
    }

    /// `ReserveRfdBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "title"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "description": "Optional contents of the RFD",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "title": {
    ///      "description": "Title of the RFD",
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ReserveRfdBody {
        /// Optional contents of the RFD
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        /// Title of the RFD
        pub title: ::std::string::String,
    }

    impl ::std::convert::From<&ReserveRfdBody> for ReserveRfdBody {
        fn from(value: &ReserveRfdBody) -> Self {
            value.clone()
        }
    }

    impl ReserveRfdBody {
        pub fn builder() -> builder::ReserveRfdBody {
            Default::default()
        }
    }

    /// `ReserveRfdResponse`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "number"
    ///  ],
    ///  "properties": {
    ///    "number": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct ReserveRfdResponse {
        pub number: i32,
    }

    impl ::std::convert::From<&ReserveRfdResponse> for ReserveRfdResponse {
        fn from(value: &ReserveRfdResponse) -> Self {
            value.clone()
        }
    }

    impl ReserveRfdResponse {
        pub fn builder() -> builder::ReserveRfdResponse {
            Default::default()
        }
    }

    /// `Rfd`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "id",
    ///    "rfd_number",
    ///    "updated_at",
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/RfdRevision"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdId"
    ///    },
    ///    "latest_major_change_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "link": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rfd_number": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct Rfd {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<RfdRevision>,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub id: TypedUuidForRfdId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latest_major_change_at:
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub link: ::std::option::Option<::std::string::String>,
        pub rfd_number: i32,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub visibility: Visibility,
    }

    impl ::std::convert::From<&Rfd> for Rfd {
        fn from(value: &Rfd) -> Self {
            value.clone()
        }
    }

    impl Rfd {
        pub fn builder() -> builder::Rfd {
            Default::default()
        }
    }

    /// `RfdAttr`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "oneOf": [
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "discussion"
    ///      ],
    ///      "properties": {
    ///        "discussion": {
    ///          "type": "string"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "labels"
    ///      ],
    ///      "properties": {
    ///        "labels": {
    ///          "type": "string"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "state"
    ///      ],
    ///      "properties": {
    ///        "state": {
    ///          "$ref": "#/components/schemas/RfdState"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    }

    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub enum RfdAttr {
        #[serde(rename = "discussion")]
        Discussion(::std::string::String),
        #[serde(rename = "labels")]
        Labels(::std::string::String),
        #[serde(rename = "state")]
        State(RfdState),
    }

    impl ::std::convert::From<&Self> for RfdAttr {
        fn from(value: &RfdAttr) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<RfdState> for RfdAttr {
        fn from(value: RfdState) -> Self {
            Self::State(value)
        }
    }

    /// `RfdAttrName`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "discussion",
    ///    "labels",
    ///    "state"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum RfdAttrName {
        #[serde(rename = "discussion")]
        Discussion,
        #[serde(rename = "labels")]
        Labels,
        #[serde(rename = "state")]
        State,
    }

    impl ::std::convert::From<&Self> for RfdAttrName {
        fn from(value: &RfdAttrName) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for RfdAttrName {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Discussion => write!(f, "discussion"),
                Self::Labels => write!(f, "labels"),
                Self::State => write!(f, "state"),
            }
        }
    }

    impl ::std::str::FromStr for RfdAttrName {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "discussion" => Ok(Self::Discussion),
                "labels" => Ok(Self::Labels),
                "state" => Ok(Self::State),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for RfdAttrName {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for RfdAttrName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RfdAttrName {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// `RfdAttrValue`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "value"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "description": "Optional Git commit message to send with this
    /// update (recommended)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "value": {
    ///      "description": "Full value to set this attribute to in the existing
    /// RFD contents",
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdAttrValue {
        /// Optional Git commit message to send with this update (recommended)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
        /// Full value to set this attribute to in the existing RFD contents
        pub value: ::std::string::String,
    }

    impl ::std::convert::From<&RfdAttrValue> for RfdAttrValue {
        fn from(value: &RfdAttrValue) -> Self {
            value.clone()
        }
    }

    impl RfdAttrValue {
        pub fn builder() -> builder::RfdAttrValue {
            Default::default()
        }
    }

    /// `RfdPdf`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "created_at",
    ///    "external_id",
    ///    "id",
    ///    "link",
    ///    "rfd_id",
    ///    "rfd_revision_id",
    ///    "source",
    ///    "updated_at"
    ///  ],
    ///  "properties": {
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "external_id": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdPdfId"
    ///    },
    ///    "link": {
    ///      "type": "string"
    ///    },
    ///    "rfd_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdId"
    ///    },
    ///    "rfd_revision_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdRevisionId"
    ///    },
    ///    "source": {
    ///      "$ref": "#/components/schemas/PdfSource"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdPdf {
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub external_id: ::std::string::String,
        pub id: TypedUuidForRfdPdfId,
        pub link: ::std::string::String,
        pub rfd_id: TypedUuidForRfdId,
        pub rfd_revision_id: TypedUuidForRfdRevisionId,
        pub source: PdfSource,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&RfdPdf> for RfdPdf {
        fn from(value: &RfdPdf) -> Self {
            value.clone()
        }
    }

    impl RfdPdf {
        pub fn builder() -> builder::RfdPdf {
            Default::default()
        }
    }

    /// `RfdPermission`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "oneOf": [
    ///    {
    ///      "type": "string",
    ///      "enum": [
    ///        "GetRfdsAssigned",
    ///        "GetRfdsAll",
    ///        "CreateRfd",
    ///        "UpdateRfdsAssigned",
    ///        "UpdateRfdsAll",
    ///        "ManageRfdsVisibilityAssigned",
    ///        "ManageRfdsVisibilityAll",
    ///        "GetDiscussionsAssigned",
    ///        "GetDiscussionsAll",
    ///        "SearchRfds",
    ///        "CreateApiUser",
    ///        "GetApiUserSelf",
    ///        "GetApiUsersAssigned",
    ///        "GetApiUsersAll",
    ///        "ManageApiUsersAssigned",
    ///        "ManageApiUsersAll",
    ///        "CreateApiKeySelf",
    ///        "CreateApiKeyAssigned",
    ///        "CreateApiKeyAll",
    ///        "GetApiKeysAssigned",
    ///        "GetApiKeysAll",
    ///        "ManageApiKeysAssigned",
    ///        "ManageApiKeysAll",
    ///        "CreateUserApiProviderLinkToken",
    ///        "CreateGroup",
    ///        "GetGroupsJoined",
    ///        "GetGroupsAll",
    ///        "ManageGroupsAssigned",
    ///        "ManageGroupsAll",
    ///        "ManageGroupMembershipsAssigned",
    ///        "ManageGroupMembershipsAll",
    ///        "CreateMapper",
    ///        "GetMappersAll",
    ///        "ManageMappersAssigned",
    ///        "ManageMappersAll",
    ///        "CreateOAuthClient",
    ///        "GetOAuthClientsAssigned",
    ///        "GetOAuthClientsAll",
    ///        "ManageOAuthClientsAssigned",
    ///        "ManageOAuthClientsAll",
    ///        "CreateMagicLinkClient",
    ///        "GetMagicLinkClientsAssigned",
    ///        "GetMagicLinkClientsAll",
    ///        "ManageMagicLinkClientsAssigned",
    ///        "ManageMagicLinkClientsAll",
    ///        "CreateAccessToken"
    ///      ]
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetRfd"
    ///      ],
    ///      "properties": {
    ///        "GetRfd": {
    ///          "type": "integer",
    ///          "format": "int32"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetRfds"
    ///      ],
    ///      "properties": {
    ///        "GetRfds": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "integer",
    ///            "format": "int32"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "UpdateRfd"
    ///      ],
    ///      "properties": {
    ///        "UpdateRfd": {
    ///          "type": "integer",
    ///          "format": "int32"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "UpdateRfds"
    ///      ],
    ///      "properties": {
    ///        "UpdateRfds": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "integer",
    ///            "format": "int32"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageRfdVisibility"
    ///      ],
    ///      "properties": {
    ///        "ManageRfdVisibility": {
    ///          "type": "integer",
    ///          "format": "int32"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageRfdsVisibility"
    ///      ],
    ///      "properties": {
    ///        "ManageRfdsVisibility": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "integer",
    ///            "format": "int32"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetDiscussion"
    ///      ],
    ///      "properties": {
    ///        "GetDiscussion": {
    ///          "type": "integer",
    ///          "format": "int32"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetDiscussions"
    ///      ],
    ///      "properties": {
    ///        "GetDiscussions": {
    ///          "type": "array",
    ///          "items": {
    ///            "type": "integer",
    ///            "format": "int32"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetApiUser"
    ///      ],
    ///      "properties": {
    ///        "GetApiUser": {
    ///          "$ref": "#/components/schemas/TypedUuidForUserId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetApiUsers"
    ///      ],
    ///      "properties": {
    ///        "GetApiUsers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForUserId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageApiUser"
    ///      ],
    ///      "properties": {
    ///        "ManageApiUser": {
    ///          "$ref": "#/components/schemas/TypedUuidForUserId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageApiUsers"
    ///      ],
    ///      "properties": {
    ///        "ManageApiUsers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForUserId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "CreateApiKey"
    ///      ],
    ///      "properties": {
    ///        "CreateApiKey": {
    ///          "$ref": "#/components/schemas/TypedUuidForUserId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetApiKey"
    ///      ],
    ///      "properties": {
    ///        "GetApiKey": {
    ///          "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetApiKeys"
    ///      ],
    ///      "properties": {
    ///        "GetApiKeys": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageApiKey"
    ///      ],
    ///      "properties": {
    ///        "ManageApiKey": {
    ///          "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageApiKeys"
    ///      ],
    ///      "properties": {
    ///        "ManageApiKeys": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForApiKeyId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetGroup"
    ///      ],
    ///      "properties": {
    ///        "GetGroup": {
    ///          "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageGroup"
    ///      ],
    ///      "properties": {
    ///        "ManageGroup": {
    ///          "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageGroups"
    ///      ],
    ///      "properties": {
    ///        "ManageGroups": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageGroupMembership"
    ///      ],
    ///      "properties": {
    ///        "ManageGroupMembership": {
    ///          "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageGroupMemberships"
    ///      ],
    ///      "properties": {
    ///        "ManageGroupMemberships": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForAccessGroupId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageMapper"
    ///      ],
    ///      "properties": {
    ///        "ManageMapper": {
    ///          "$ref": "#/components/schemas/TypedUuidForMapperId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageMappers"
    ///      ],
    ///      "properties": {
    ///        "ManageMappers": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForMapperId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetOAuthClient"
    ///      ],
    ///      "properties": {
    ///        "GetOAuthClient": {
    ///          "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetOAuthClients"
    ///      ],
    ///      "properties": {
    ///        "GetOAuthClients": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageOAuthClient"
    ///      ],
    ///      "properties": {
    ///        "ManageOAuthClient": {
    ///          "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageOAuthClients"
    ///      ],
    ///      "properties": {
    ///        "ManageOAuthClients": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForOAuthClientId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetMagicLinkClient"
    ///      ],
    ///      "properties": {
    ///        "GetMagicLinkClient": {
    ///          "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "GetMagicLinkClients"
    ///      ],
    ///      "properties": {
    ///        "GetMagicLinkClients": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageMagicLinkClient"
    ///      ],
    ///      "properties": {
    ///        "ManageMagicLinkClient": {
    ///          "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "ManageMagicLinkClients"
    ///      ],
    ///      "properties": {
    ///        "ManageMagicLinkClients": {
    ///          "type": "array",
    ///          "items": {
    ///            "$ref": "#/components/schemas/TypedUuidForMagicLinkId"
    ///          },
    ///          "uniqueItems": true
    ///        }

    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    {
    ///      "type": "object",
    ///      "required": [
    ///        "Unsupported"
    ///      ],
    ///      "properties": {
    ///        "Unsupported": {}

    ///      },
    ///      "additionalProperties": false
    ///    }

    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub enum RfdPermission {
        GetRfdsAssigned,
        GetRfdsAll,
        CreateRfd,
        UpdateRfdsAssigned,
        UpdateRfdsAll,
        ManageRfdsVisibilityAssigned,
        ManageRfdsVisibilityAll,
        GetDiscussionsAssigned,
        GetDiscussionsAll,
        SearchRfds,
        CreateApiUser,
        GetApiUserSelf,
        GetApiUsersAssigned,
        GetApiUsersAll,
        ManageApiUsersAssigned,
        ManageApiUsersAll,
        CreateApiKeySelf,
        CreateApiKeyAssigned,
        CreateApiKeyAll,
        GetApiKeysAssigned,
        GetApiKeysAll,
        ManageApiKeysAssigned,
        ManageApiKeysAll,
        CreateUserApiProviderLinkToken,
        CreateGroup,
        GetGroupsJoined,
        GetGroupsAll,
        ManageGroupsAssigned,
        ManageGroupsAll,
        ManageGroupMembershipsAssigned,
        ManageGroupMembershipsAll,
        CreateMapper,
        GetMappersAll,
        ManageMappersAssigned,
        ManageMappersAll,
        CreateOAuthClient,
        GetOAuthClientsAssigned,
        GetOAuthClientsAll,
        ManageOAuthClientsAssigned,
        ManageOAuthClientsAll,
        CreateMagicLinkClient,
        GetMagicLinkClientsAssigned,
        GetMagicLinkClientsAll,
        ManageMagicLinkClientsAssigned,
        ManageMagicLinkClientsAll,
        CreateAccessToken,
        GetRfd(i32),
        GetRfds(Vec<i32>),
        UpdateRfd(i32),
        UpdateRfds(Vec<i32>),
        ManageRfdVisibility(i32),
        ManageRfdsVisibility(Vec<i32>),
        GetDiscussion(i32),
        GetDiscussions(Vec<i32>),
        GetApiUser(TypedUuidForUserId),
        GetApiUsers(Vec<TypedUuidForUserId>),
        ManageApiUser(TypedUuidForUserId),
        ManageApiUsers(Vec<TypedUuidForUserId>),
        CreateApiKey(TypedUuidForUserId),
        GetApiKey(TypedUuidForApiKeyId),
        GetApiKeys(Vec<TypedUuidForApiKeyId>),
        ManageApiKey(TypedUuidForApiKeyId),
        ManageApiKeys(Vec<TypedUuidForApiKeyId>),
        GetGroup(TypedUuidForAccessGroupId),
        ManageGroup(TypedUuidForAccessGroupId),
        ManageGroups(Vec<TypedUuidForAccessGroupId>),
        ManageGroupMembership(TypedUuidForAccessGroupId),
        ManageGroupMemberships(Vec<TypedUuidForAccessGroupId>),
        ManageMapper(TypedUuidForMapperId),
        ManageMappers(Vec<TypedUuidForMapperId>),
        GetOAuthClient(TypedUuidForOAuthClientId),
        GetOAuthClients(Vec<TypedUuidForOAuthClientId>),
        ManageOAuthClient(TypedUuidForOAuthClientId),
        ManageOAuthClients(Vec<TypedUuidForOAuthClientId>),
        GetMagicLinkClient(TypedUuidForMagicLinkId),
        GetMagicLinkClients(Vec<TypedUuidForMagicLinkId>),
        ManageMagicLinkClient(TypedUuidForMagicLinkId),
        ManageMagicLinkClients(Vec<TypedUuidForMagicLinkId>),
        Unsupported(::serde_json::Value),
    }

    impl ::std::convert::From<&Self> for RfdPermission {
        fn from(value: &RfdPermission) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<TypedUuidForMapperId> for RfdPermission {
        fn from(value: TypedUuidForMapperId) -> Self {
            Self::ManageMapper(value)
        }
    }

    impl ::std::convert::From<Vec<TypedUuidForMapperId>> for RfdPermission {
        fn from(value: Vec<TypedUuidForMapperId>) -> Self {
            Self::ManageMappers(value)
        }
    }

    impl ::std::convert::From<::serde_json::Value> for RfdPermission {
        fn from(value: ::serde_json::Value) -> Self {
            Self::Unsupported(value)
        }
    }

    /// `RfdRevision`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "commit",
    ///    "committed_at",
    ///    "content",
    ///    "content_format",
    ///    "created_at",
    ///    "id",
    ///    "major_change",
    ///    "rfd_id",
    ///    "sha",
    ///    "title",
    ///    "updated_at"
    ///  ],
    ///  "properties": {
    ///    "authors": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "commit": {
    ///      "$ref": "#/components/schemas/CommitSha"
    ///    },
    ///    "committed_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "content": {
    ///      "type": "string"
    ///    },
    ///    "content_format": {
    ///      "$ref": "#/components/schemas/ContentFormat"
    ///    },
    ///    "created_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    },
    ///    "deleted_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "discussion": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdRevisionId"
    ///    },
    ///    "labels": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "major_change": {
    ///      "type": "boolean"
    ///    },
    ///    "rfd_id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdId"
    ///    },
    ///    "sha": {
    ///      "$ref": "#/components/schemas/FileSha"
    ///    },
    ///    "state": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "title": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdRevision {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub authors: ::std::option::Option<::std::string::String>,
        pub commit: CommitSha,
        pub committed_at: ::chrono::DateTime<::chrono::offset::Utc>,
        pub content: ::std::string::String,
        pub content_format: ContentFormat,
        pub created_at: ::chrono::DateTime<::chrono::offset::Utc>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discussion: ::std::option::Option<::std::string::String>,
        pub id: TypedUuidForRfdRevisionId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub labels: ::std::option::Option<::std::string::String>,
        pub major_change: bool,
        pub rfd_id: TypedUuidForRfdId,
        pub sha: FileSha,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        pub title: ::std::string::String,
        pub updated_at: ::chrono::DateTime<::chrono::offset::Utc>,
    }

    impl ::std::convert::From<&RfdRevision> for RfdRevision {
        fn from(value: &RfdRevision) -> Self {
            value.clone()
        }
    }

    impl RfdRevision {
        pub fn builder() -> builder::RfdRevision {
            Default::default()
        }
    }

    /// `RfdState`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "abandoned",
    ///    "committed",
    ///    "discussion",
    ///    "ideation",
    ///    "prediscussion",
    ///    "published"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum RfdState {
        #[serde(rename = "abandoned")]
        Abandoned,
        #[serde(rename = "committed")]
        Committed,
        #[serde(rename = "discussion")]
        Discussion,
        #[serde(rename = "ideation")]
        Ideation,
        #[serde(rename = "prediscussion")]
        Prediscussion,
        #[serde(rename = "published")]
        Published,
    }

    impl ::std::convert::From<&Self> for RfdState {
        fn from(value: &RfdState) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for RfdState {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Abandoned => write!(f, "abandoned"),
                Self::Committed => write!(f, "committed"),
                Self::Discussion => write!(f, "discussion"),
                Self::Ideation => write!(f, "ideation"),
                Self::Prediscussion => write!(f, "prediscussion"),
                Self::Published => write!(f, "published"),
            }
        }
    }

    impl ::std::str::FromStr for RfdState {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "abandoned" => Ok(Self::Abandoned),
                "committed" => Ok(Self::Committed),
                "discussion" => Ok(Self::Discussion),
                "ideation" => Ok(Self::Ideation),
                "prediscussion" => Ok(Self::Prediscussion),
                "published" => Ok(Self::Published),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for RfdState {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for RfdState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for RfdState {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// `RfdUpdateBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "document"
    ///  ],
    ///  "properties": {
    ///    "document": {
    ///      "description": "Full Asciidoc document to store for this RFD",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Optional Git commit message to send with this
    /// update (recommended)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdUpdateBody {
        /// Full Asciidoc document to store for this RFD
        pub document: ::std::string::String,
        /// Optional Git commit message to send with this update (recommended)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&RfdUpdateBody> for RfdUpdateBody {
        fn from(value: &RfdUpdateBody) -> Self {
            value.clone()
        }
    }

    impl RfdUpdateBody {
        pub fn builder() -> builder::RfdUpdateBody {
            Default::default()
        }
    }

    /// `RfdUpdateContentBody`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "content"
    ///  ],
    ///  "properties": {
    ///    "content": {
    ///      "description": "Asciidoc content to store for this RFD",
    ///      "type": "string"
    ///    },
    ///    "message": {
    ///      "description": "Optional Git commit message to send with this
    /// update (recommended)",
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdUpdateContentBody {
        /// Asciidoc content to store for this RFD
        pub content: ::std::string::String,
        /// Optional Git commit message to send with this update (recommended)
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub message: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&RfdUpdateContentBody> for RfdUpdateContentBody {
        fn from(value: &RfdUpdateContentBody) -> Self {
            value.clone()
        }
    }

    impl RfdUpdateContentBody {
        pub fn builder() -> builder::RfdUpdateContentBody {
            Default::default()
        }
    }

    /// `RfdVisibility`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdVisibility {
        pub visibility: Visibility,
    }

    impl ::std::convert::From<&RfdVisibility> for RfdVisibility {
        fn from(value: &RfdVisibility) -> Self {
            value.clone()
        }
    }

    impl RfdVisibility {
        pub fn builder() -> builder::RfdVisibility {
            Default::default()
        }
    }

    /// `RfdWithPdf`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "content",
    ///    "id",
    ///    "rfd_number",
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "authors": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "commit": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CommitSha"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "committed_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "content": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RfdPdf"
    ///      }

    ///    },
    ///    "discussion": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "format": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/ContentFormat"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdId"
    ///    },
    ///    "labels": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "latest_major_change_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "link": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rfd_number": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "sha": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/FileSha"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "state": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "title": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdWithPdf {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub authors: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit: ::std::option::Option<CommitSha>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        pub content: ::std::vec::Vec<RfdPdf>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discussion: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub format: ::std::option::Option<ContentFormat>,
        pub id: TypedUuidForRfdId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub labels: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latest_major_change_at:
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub link: ::std::option::Option<::std::string::String>,
        pub rfd_number: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha: ::std::option::Option<FileSha>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        pub visibility: Visibility,
    }

    impl ::std::convert::From<&RfdWithPdf> for RfdWithPdf {
        fn from(value: &RfdWithPdf) -> Self {
            value.clone()
        }
    }

    impl RfdWithPdf {
        pub fn builder() -> builder::RfdWithPdf {
            Default::default()
        }
    }

    /// `RfdWithRaw`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "rfd_number",
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "authors": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "commit": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CommitSha"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "committed_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "content": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "discussion": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "format": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/ContentFormat"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdId"
    ///    },
    ///    "labels": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "latest_major_change_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "link": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rfd_number": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "sha": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/FileSha"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "state": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "title": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdWithRaw {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub authors: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit: ::std::option::Option<CommitSha>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub content: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discussion: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub format: ::std::option::Option<ContentFormat>,
        pub id: TypedUuidForRfdId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub labels: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latest_major_change_at:
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub link: ::std::option::Option<::std::string::String>,
        pub rfd_number: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha: ::std::option::Option<FileSha>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        pub visibility: Visibility,
    }

    impl ::std::convert::From<&RfdWithRaw> for RfdWithRaw {
        fn from(value: &RfdWithRaw) -> Self {
            value.clone()
        }
    }

    impl RfdWithRaw {
        pub fn builder() -> builder::RfdWithRaw {
            Default::default()
        }
    }

    /// `RfdWithoutContent`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "id",
    ///    "rfd_number",
    ///    "visibility"
    ///  ],
    ///  "properties": {
    ///    "authors": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "commit": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/CommitSha"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "committed_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "discussion": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "format": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/ContentFormat"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "id": {
    ///      "$ref": "#/components/schemas/TypedUuidForRfdId"
    ///    },
    ///    "labels": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "latest_major_change_at": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ],
    ///      "format": "date-time"
    ///    },
    ///    "link": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "rfd_number": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "sha": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/FileSha"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "state": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "title": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "visibility": {
    ///      "$ref": "#/components/schemas/Visibility"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct RfdWithoutContent {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub authors: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub commit: ::std::option::Option<CommitSha>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub committed_at: ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub discussion: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub format: ::std::option::Option<ContentFormat>,
        pub id: TypedUuidForRfdId,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub labels: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub latest_major_change_at:
            ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub link: ::std::option::Option<::std::string::String>,
        pub rfd_number: i32,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub sha: ::std::option::Option<FileSha>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub state: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        pub visibility: Visibility,
    }

    impl ::std::convert::From<&RfdWithoutContent> for RfdWithoutContent {
        fn from(value: &RfdWithoutContent) -> Self {
            value.clone()
        }
    }

    impl RfdWithoutContent {
        pub fn builder() -> builder::RfdWithoutContent {
            Default::default()
        }
    }

    /// `SearchResultHit`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "content",
    ///    "hierarchy",
    ///    "hierarchy_radio",
    ///    "object_id",
    ///    "rfd_number"
    ///  ],
    ///  "properties": {
    ///    "anchor": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "content": {
    ///      "type": "string"
    ///    },
    ///    "formatted": {
    ///      "oneOf": [
    ///        {
    ///          "type": "null"
    ///        },
    ///        {
    ///          "allOf": [
    ///            {
    ///              "$ref": "#/components/schemas/FormattedSearchResultHit"
    ///            }

    ///          ]
    ///        }

    ///      ]
    ///    },
    ///    "hierarchy": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": [
    ///          "string",
    ///          "null"
    ///        ]
    ///      },
    ///      "maxItems": 6,
    ///      "minItems": 6
    ///    },
    ///    "hierarchy_radio": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": [
    ///          "string",
    ///          "null"
    ///        ]
    ///      },
    ///      "maxItems": 6,
    ///      "minItems": 6
    ///    },
    ///    "object_id": {
    ///      "type": "string"
    ///    },
    ///    "rfd_number": {
    ///      "type": "integer",
    ///      "format": "uint64",
    ///      "minimum": 0.0
    ///    },
    ///    "url": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct SearchResultHit {
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub anchor: ::std::option::Option<::std::string::String>,
        pub content: ::std::string::String,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub formatted: ::std::option::Option<FormattedSearchResultHit>,
        pub hierarchy: [::std::option::Option<::std::string::String>; 6usize],
        pub hierarchy_radio: [::std::option::Option<::std::string::String>; 6usize],
        pub object_id: ::std::string::String,
        pub rfd_number: u64,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub url: ::std::option::Option<::std::string::String>,
    }

    impl ::std::convert::From<&SearchResultHit> for SearchResultHit {
        fn from(value: &SearchResultHit) -> Self {
            value.clone()
        }
    }

    impl SearchResultHit {
        pub fn builder() -> builder::SearchResultHit {
            Default::default()
        }
    }

    /// `SearchResults`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "object",
    ///  "required": [
    ///    "hits",
    ///    "query"
    ///  ],
    ///  "properties": {
    ///    "hits": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SearchResultHit"
    ///      }

    ///    },
    ///    "limit": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "offset": {
    ///      "type": [
    ///        "integer",
    ///        "null"
    ///      ],
    ///      "format": "uint",
    ///      "minimum": 0.0
    ///    },
    ///    "query": {
    ///      "type": "string"
    ///    }

    ///  }

    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    pub struct SearchResults {
        pub hits: ::std::vec::Vec<SearchResultHit>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub limit: ::std::option::Option<u32>,
        #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
        pub offset: ::std::option::Option<u32>,
        pub query: ::std::string::String,
    }

    impl ::std::convert::From<&SearchResults> for SearchResults {
        fn from(value: &SearchResults) -> Self {
            value.clone()
        }
    }

    impl SearchResults {
        pub fn builder() -> builder::SearchResults {
            Default::default()
        }
    }

    /// `SecretString`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct SecretString(pub ::std::string::String);
    impl ::std::ops::Deref for SecretString {
        type Target = ::std::string::String;
        fn deref(&self) -> &::std::string::String {
            &self.0
        }
    }

    impl ::std::convert::From<SecretString> for ::std::string::String {
        fn from(value: SecretString) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&SecretString> for SecretString {
        fn from(value: &SecretString) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::std::string::String> for SecretString {
        fn from(value: ::std::string::String) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for SecretString {
        type Err = ::std::convert::Infallible;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.to_string()))
        }
    }

    impl ::std::fmt::Display for SecretString {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForAccessGroupId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForAccessGroupId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForAccessGroupId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForAccessGroupId> for ::uuid::Uuid {
        fn from(value: TypedUuidForAccessGroupId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForAccessGroupId> for TypedUuidForAccessGroupId {
        fn from(value: &TypedUuidForAccessGroupId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForAccessGroupId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForAccessGroupId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForAccessGroupId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForAccessGroupId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForAccessGroupId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForAccessGroupId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForApiKeyId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForApiKeyId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForApiKeyId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForApiKeyId> for ::uuid::Uuid {
        fn from(value: TypedUuidForApiKeyId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForApiKeyId> for TypedUuidForApiKeyId {
        fn from(value: &TypedUuidForApiKeyId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForApiKeyId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForApiKeyId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForApiKeyId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForApiKeyId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForApiKeyId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForApiKeyId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMagicLinkAttemptId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMagicLinkAttemptId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMagicLinkAttemptId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMagicLinkAttemptId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMagicLinkAttemptId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForMagicLinkAttemptId> for TypedUuidForMagicLinkAttemptId {
        fn from(value: &TypedUuidForMagicLinkAttemptId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMagicLinkAttemptId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMagicLinkAttemptId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMagicLinkAttemptId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForMagicLinkAttemptId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMagicLinkAttemptId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMagicLinkAttemptId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMagicLinkId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMagicLinkId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMagicLinkId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMagicLinkId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMagicLinkId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForMagicLinkId> for TypedUuidForMagicLinkId {
        fn from(value: &TypedUuidForMagicLinkId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMagicLinkId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMagicLinkId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMagicLinkId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForMagicLinkId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMagicLinkId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMagicLinkId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMagicLinkRedirectUriId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMagicLinkRedirectUriId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMagicLinkRedirectUriId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMagicLinkRedirectUriId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMagicLinkRedirectUriId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForMagicLinkRedirectUriId>
        for TypedUuidForMagicLinkRedirectUriId
    {
        fn from(value: &TypedUuidForMagicLinkRedirectUriId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMagicLinkRedirectUriId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMagicLinkRedirectUriId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMagicLinkRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForMagicLinkRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMagicLinkRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMagicLinkRedirectUriId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMagicLinkSecretId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMagicLinkSecretId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMagicLinkSecretId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMagicLinkSecretId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMagicLinkSecretId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForMagicLinkSecretId> for TypedUuidForMagicLinkSecretId {
        fn from(value: &TypedUuidForMagicLinkSecretId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMagicLinkSecretId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMagicLinkSecretId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMagicLinkSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForMagicLinkSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMagicLinkSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMagicLinkSecretId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForMapperId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForMapperId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForMapperId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForMapperId> for ::uuid::Uuid {
        fn from(value: TypedUuidForMapperId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForMapperId> for TypedUuidForMapperId {
        fn from(value: &TypedUuidForMapperId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForMapperId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForMapperId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForMapperId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForMapperId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForMapperId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForMapperId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForOAuthClientId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForOAuthClientId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForOAuthClientId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForOAuthClientId> for ::uuid::Uuid {
        fn from(value: TypedUuidForOAuthClientId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForOAuthClientId> for TypedUuidForOAuthClientId {
        fn from(value: &TypedUuidForOAuthClientId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForOAuthClientId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForOAuthClientId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForOAuthClientId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForOAuthClientId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForOAuthClientId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForOAuthClientId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForOAuthRedirectUriId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForOAuthRedirectUriId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForOAuthRedirectUriId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForOAuthRedirectUriId> for ::uuid::Uuid {
        fn from(value: TypedUuidForOAuthRedirectUriId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForOAuthRedirectUriId> for TypedUuidForOAuthRedirectUriId {
        fn from(value: &TypedUuidForOAuthRedirectUriId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForOAuthRedirectUriId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForOAuthRedirectUriId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForOAuthRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForOAuthRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForOAuthRedirectUriId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForOAuthRedirectUriId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForOAuthSecretId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForOAuthSecretId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForOAuthSecretId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForOAuthSecretId> for ::uuid::Uuid {
        fn from(value: TypedUuidForOAuthSecretId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForOAuthSecretId> for TypedUuidForOAuthSecretId {
        fn from(value: &TypedUuidForOAuthSecretId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForOAuthSecretId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForOAuthSecretId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForOAuthSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForOAuthSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForOAuthSecretId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForOAuthSecretId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForRfdId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForRfdId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForRfdId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForRfdId> for ::uuid::Uuid {
        fn from(value: TypedUuidForRfdId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForRfdId> for TypedUuidForRfdId {
        fn from(value: &TypedUuidForRfdId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForRfdId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForRfdId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForRfdId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForRfdId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForRfdId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForRfdId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForRfdPdfId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForRfdPdfId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForRfdPdfId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForRfdPdfId> for ::uuid::Uuid {
        fn from(value: TypedUuidForRfdPdfId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForRfdPdfId> for TypedUuidForRfdPdfId {
        fn from(value: &TypedUuidForRfdPdfId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForRfdPdfId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForRfdPdfId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForRfdPdfId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForRfdPdfId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForRfdPdfId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForRfdPdfId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForRfdRevisionId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForRfdRevisionId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForRfdRevisionId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForRfdRevisionId> for ::uuid::Uuid {
        fn from(value: TypedUuidForRfdRevisionId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForRfdRevisionId> for TypedUuidForRfdRevisionId {
        fn from(value: &TypedUuidForRfdRevisionId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForRfdRevisionId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForRfdRevisionId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForRfdRevisionId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForRfdRevisionId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForRfdRevisionId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForRfdRevisionId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForUserId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForUserId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForUserId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForUserId> for ::uuid::Uuid {
        fn from(value: TypedUuidForUserId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForUserId> for TypedUuidForUserId {
        fn from(value: &TypedUuidForUserId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForUserId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForUserId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForUserId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForUserId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForUserId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForUserId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForUserProviderId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForUserProviderId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForUserProviderId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForUserProviderId> for ::uuid::Uuid {
        fn from(value: TypedUuidForUserProviderId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForUserProviderId> for TypedUuidForUserProviderId {
        fn from(value: &TypedUuidForUserProviderId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForUserProviderId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForUserProviderId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForUserProviderId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForUserProviderId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForUserProviderId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForUserProviderId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `TypedUuidForWebhookDeliveryId`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "format": "uuid"
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, schemars :: JsonSchema,
    )]
    #[serde(transparent)]
    pub struct TypedUuidForWebhookDeliveryId(pub ::uuid::Uuid);
    impl ::std::ops::Deref for TypedUuidForWebhookDeliveryId {
        type Target = ::uuid::Uuid;
        fn deref(&self) -> &::uuid::Uuid {
            &self.0
        }
    }

    impl ::std::convert::From<TypedUuidForWebhookDeliveryId> for ::uuid::Uuid {
        fn from(value: TypedUuidForWebhookDeliveryId) -> Self {
            value.0
        }
    }

    impl ::std::convert::From<&TypedUuidForWebhookDeliveryId> for TypedUuidForWebhookDeliveryId {
        fn from(value: &TypedUuidForWebhookDeliveryId) -> Self {
            value.clone()
        }
    }

    impl ::std::convert::From<::uuid::Uuid> for TypedUuidForWebhookDeliveryId {
        fn from(value: ::uuid::Uuid) -> Self {
            Self(value)
        }
    }

    impl ::std::str::FromStr for TypedUuidForWebhookDeliveryId {
        type Err = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn from_str(value: &str) -> ::std::result::Result<Self, Self::Err> {
            Ok(Self(value.parse()?))
        }
    }

    impl ::std::convert::TryFrom<&str> for TypedUuidForWebhookDeliveryId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &str) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&String> for TypedUuidForWebhookDeliveryId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: &String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<String> for TypedUuidForWebhookDeliveryId {
        type Error = <::uuid::Uuid as ::std::str::FromStr>::Err;
        fn try_from(value: String) -> ::std::result::Result<Self, Self::Error> {
            value.parse()
        }
    }

    impl ::std::fmt::Display for TypedUuidForWebhookDeliveryId {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            self.0.fmt(f)
        }
    }

    /// `Visibility`
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    /// {
    ///  "type": "string",
    ///  "enum": [
    ///    "public",
    ///    "private"
    ///  ]
    /// }

    /// ```
    /// </details>
    #[derive(
        :: serde :: Deserialize,
        :: serde :: Serialize,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        schemars :: JsonSchema,
    )]
    pub enum Visibility {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
    }

    impl ::std::convert::From<&Self> for Visibility {
        fn from(value: &Visibility) -> Self {
            value.clone()
        }
    }

    impl ::std::fmt::Display for Visibility {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match *self {
                Self::Public => write!(f, "public"),
                Self::Private => write!(f, "private"),
            }
        }
    }

    impl ::std::str::FromStr for Visibility {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            match value {
                "public" => Ok(Self::Public),
                "private" => Ok(Self::Private),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl ::std::convert::TryFrom<&str> for Visibility {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<&::std::string::String> for Visibility {
        type Error = self::error::ConversionError;
        fn try_from(
            value: &::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl ::std::convert::TryFrom<::std::string::String> for Visibility {
        type Error = self::error::ConversionError;
        fn try_from(
            value: ::std::string::String,
        ) -> ::std::result::Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    /// Types for composing complex structures.
    pub mod builder {
        #[derive(Clone, Debug)]
        pub struct AccessGroupForRfdPermission {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForAccessGroupId, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            permissions:
                ::std::result::Result<super::PermissionsForRfdPermission, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for AccessGroupForRfdPermission {
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

        impl AccessGroupForRfdPermission {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForAccessGroupId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PermissionsForRfdPermission>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<AccessGroupForRfdPermission> for super::AccessGroupForRfdPermission {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccessGroupForRfdPermission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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

        impl ::std::convert::From<super::AccessGroupForRfdPermission> for AccessGroupForRfdPermission {
            fn from(value: super::AccessGroupForRfdPermission) -> Self {
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
        pub struct AccessGroupUpdateParamsForRfdPermission {
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            permissions:
                ::std::result::Result<super::PermissionsForRfdPermission, ::std::string::String>,
        }

        impl ::std::default::Default for AccessGroupUpdateParamsForRfdPermission {
            fn default() -> Self {
                Self {
                    name: Err("no value supplied for name".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl AccessGroupUpdateParamsForRfdPermission {
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PermissionsForRfdPermission>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<AccessGroupUpdateParamsForRfdPermission>
            for super::AccessGroupUpdateParamsForRfdPermission
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccessGroupUpdateParamsForRfdPermission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    name: value.name?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::AccessGroupUpdateParamsForRfdPermission>
            for AccessGroupUpdateParamsForRfdPermission
        {
            fn from(value: super::AccessGroupUpdateParamsForRfdPermission) -> Self {
                Self {
                    name: Ok(value.name),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AccessTokenExchangeRequest {
            device_code: ::std::result::Result<::std::string::String, ::std::string::String>,
            expires_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            grant_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for AccessTokenExchangeRequest {
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
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.device_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for device_code: {}", e));
                self
            }
            pub fn expires_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.expires_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_at: {}", e));
                self
            }
            pub fn grant_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.grant_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grant_type: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<AccessTokenExchangeRequest> for super::AccessTokenExchangeRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AccessTokenExchangeRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    device_code: value.device_code?,
                    expires_at: value.expires_at?,
                    grant_type: value.grant_type?,
                })
            }
        }

        impl ::std::convert::From<super::AccessTokenExchangeRequest> for AccessTokenExchangeRequest {
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
            group_id:
                ::std::result::Result<super::TypedUuidForAccessGroupId, ::std::string::String>,
        }

        impl ::std::default::Default for AddGroupBody {
            fn default() -> Self {
                Self {
                    group_id: Err("no value supplied for group_id".to_string()),
                }
            }
        }

        impl AddGroupBody {
            pub fn group_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForAccessGroupId>,
                T::Error: ::std::fmt::Display,
            {
                self.group_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for group_id: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<AddGroupBody> for super::AddGroupBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AddGroupBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    group_id: value.group_id?,
                })
            }
        }

        impl ::std::convert::From<super::AddGroupBody> for AddGroupBody {
            fn from(value: super::AddGroupBody) -> Self {
                Self {
                    group_id: Ok(value.group_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddMagicLinkRedirectBody {
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for AddMagicLinkRedirectBody {
            fn default() -> Self {
                Self {
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl AddMagicLinkRedirectBody {
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<AddMagicLinkRedirectBody> for super::AddMagicLinkRedirectBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AddMagicLinkRedirectBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::AddMagicLinkRedirectBody> for AddMagicLinkRedirectBody {
            fn from(value: super::AddMagicLinkRedirectBody) -> Self {
                Self {
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct AddOAuthClientRedirectBody {
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for AddOAuthClientRedirectBody {
            fn default() -> Self {
                Self {
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl AddOAuthClientRedirectBody {
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<AddOAuthClientRedirectBody> for super::AddOAuthClientRedirectBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: AddOAuthClientRedirectBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::AddOAuthClientRedirectBody> for AddOAuthClientRedirectBody {
            fn from(value: super::AddOAuthClientRedirectBody) -> Self {
                Self {
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiKeyCreateParamsForRfdPermission {
            expires_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            permissions: ::std::result::Result<
                ::std::option::Option<super::PermissionsForRfdPermission>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ApiKeyCreateParamsForRfdPermission {
            fn default() -> Self {
                Self {
                    expires_at: Err("no value supplied for expires_at".to_string()),
                    permissions: Ok(Default::default()),
                }
            }
        }

        impl ApiKeyCreateParamsForRfdPermission {
            pub fn expires_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_at: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PermissionsForRfdPermission>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiKeyCreateParamsForRfdPermission>
            for super::ApiKeyCreateParamsForRfdPermission
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiKeyCreateParamsForRfdPermission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    expires_at: value.expires_at?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::ApiKeyCreateParamsForRfdPermission>
            for ApiKeyCreateParamsForRfdPermission
        {
            fn from(value: super::ApiKeyCreateParamsForRfdPermission) -> Self {
                Self {
                    expires_at: Ok(value.expires_at),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiKeyResponseForRfdPermission {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForApiKeyId, ::std::string::String>,
            permissions: ::std::result::Result<
                ::std::option::Option<super::PermissionsForRfdPermission>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ApiKeyResponseForRfdPermission {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    permissions: Ok(Default::default()),
                }
            }
        }

        impl ApiKeyResponseForRfdPermission {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForApiKeyId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PermissionsForRfdPermission>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiKeyResponseForRfdPermission>
            for super::ApiKeyResponseForRfdPermission
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiKeyResponseForRfdPermission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::ApiKeyResponseForRfdPermission>
            for ApiKeyResponseForRfdPermission
        {
            fn from(value: super::ApiKeyResponseForRfdPermission) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserContactEmail {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<super::TypedUuidForUserProviderId, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<super::TypedUuidForUserId, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserContactEmail {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    email: Err("no value supplied for email".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                    user_id: Err("no value supplied for user_id".to_string()),
                }
            }
        }

        impl ApiUserContactEmail {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserProviderId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserId>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserContactEmail> for super::ApiUserContactEmail {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserContactEmail,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    email: value.email?,
                    id: value.id?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserContactEmail> for ApiUserContactEmail {
            fn from(value: super::ApiUserContactEmail) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    email: Ok(value.email),
                    id: Ok(value.id),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserEmailUpdateParams {
            email: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserEmailUpdateParams {
            fn default() -> Self {
                Self {
                    email: Err("no value supplied for email".to_string()),
                }
            }
        }

        impl ApiUserEmailUpdateParams {
            pub fn email<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.email = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for email: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserEmailUpdateParams> for super::ApiUserEmailUpdateParams {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserEmailUpdateParams,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    email: value.email?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserEmailUpdateParams> for ApiUserEmailUpdateParams {
            fn from(value: super::ApiUserEmailUpdateParams) -> Self {
                Self {
                    email: Ok(value.email),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserForRfdPermission {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            groups:
                ::std::result::Result<Vec<super::TypedUuidForAccessGroupId>, ::std::string::String>,
            id: ::std::result::Result<super::TypedUuidForUserId, ::std::string::String>,
            permissions:
                ::std::result::Result<super::PermissionsForRfdPermission, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for ApiUserForRfdPermission {
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

        impl ApiUserForRfdPermission {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn groups<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<Vec<super::TypedUuidForAccessGroupId>>,
                T::Error: ::std::fmt::Display,
            {
                self.groups = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for groups: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PermissionsForRfdPermission>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserForRfdPermission> for super::ApiUserForRfdPermission {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserForRfdPermission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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

        impl ::std::convert::From<super::ApiUserForRfdPermission> for ApiUserForRfdPermission {
            fn from(value: super::ApiUserForRfdPermission) -> Self {
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
            user_id: ::std::result::Result<super::TypedUuidForUserId, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserLinkRequestPayload {
            fn default() -> Self {
                Self {
                    user_id: Err("no value supplied for user_id".to_string()),
                }
            }
        }

        impl ApiUserLinkRequestPayload {
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserId>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserLinkRequestPayload> for super::ApiUserLinkRequestPayload {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserLinkRequestPayload,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    user_id: value.user_id?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserLinkRequestPayload> for ApiUserLinkRequestPayload {
            fn from(value: super::ApiUserLinkRequestPayload) -> Self {
                Self {
                    user_id: Ok(value.user_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserLinkRequestResponse {
            token: ::std::result::Result<super::SecretString, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserLinkRequestResponse {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl ApiUserLinkRequestResponse {
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SecretString>,
                T::Error: ::std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserLinkRequestResponse> for super::ApiUserLinkRequestResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserLinkRequestResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserLinkRequestResponse> for ApiUserLinkRequestResponse {
            fn from(value: super::ApiUserLinkRequestResponse) -> Self {
                Self {
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserProvider {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            display_names: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            emails: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForUserProviderId, ::std::string::String>,
            provider: ::std::result::Result<::std::string::String, ::std::string::String>,
            provider_id: ::std::result::Result<::std::string::String, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            user_id: ::std::result::Result<super::TypedUuidForUserId, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserProvider {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    display_names: Err("no value supplied for display_names".to_string()),
                    emails: Err("no value supplied for emails".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    provider: Err("no value supplied for provider".to_string()),
                    provider_id: Err("no value supplied for provider_id".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                    user_id: Err("no value supplied for user_id".to_string()),
                }
            }
        }

        impl ApiUserProvider {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn display_names<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.display_names = value.try_into().map_err(|e| {
                    format!("error converting supplied value for display_names: {}", e)
                });
                self
            }
            pub fn emails<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.emails = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for emails: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserProviderId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn provider<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.provider = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider: {}", e));
                self
            }
            pub fn provider_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.provider_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider_id: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn user_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForUserId>,
                T::Error: ::std::fmt::Display,
            {
                self.user_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for user_id: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserProvider> for super::ApiUserProvider {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserProvider,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    display_names: value.display_names?,
                    emails: value.emails?,
                    id: value.id?,
                    provider: value.provider?,
                    provider_id: value.provider_id?,
                    updated_at: value.updated_at?,
                    user_id: value.user_id?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserProvider> for ApiUserProvider {
            fn from(value: super::ApiUserProvider) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    display_names: Ok(value.display_names),
                    emails: Ok(value.emails),
                    id: Ok(value.id),
                    provider: Ok(value.provider),
                    provider_id: Ok(value.provider_id),
                    updated_at: Ok(value.updated_at),
                    user_id: Ok(value.user_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserProviderLinkPayload {
            token: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserProviderLinkPayload {
            fn default() -> Self {
                Self {
                    token: Err("no value supplied for token".to_string()),
                }
            }
        }

        impl ApiUserProviderLinkPayload {
            pub fn token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserProviderLinkPayload> for super::ApiUserProviderLinkPayload {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserProviderLinkPayload,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    token: value.token?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserProviderLinkPayload> for ApiUserProviderLinkPayload {
            fn from(value: super::ApiUserProviderLinkPayload) -> Self {
                Self {
                    token: Ok(value.token),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ApiUserUpdateParamsForRfdPermission {
            group_ids:
                ::std::result::Result<Vec<super::TypedUuidForAccessGroupId>, ::std::string::String>,
            permissions:
                ::std::result::Result<super::PermissionsForRfdPermission, ::std::string::String>,
        }

        impl ::std::default::Default for ApiUserUpdateParamsForRfdPermission {
            fn default() -> Self {
                Self {
                    group_ids: Err("no value supplied for group_ids".to_string()),
                    permissions: Err("no value supplied for permissions".to_string()),
                }
            }
        }

        impl ApiUserUpdateParamsForRfdPermission {
            pub fn group_ids<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<Vec<super::TypedUuidForAccessGroupId>>,
                T::Error: ::std::fmt::Display,
            {
                self.group_ids = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for group_ids: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PermissionsForRfdPermission>,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ApiUserUpdateParamsForRfdPermission>
            for super::ApiUserUpdateParamsForRfdPermission
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ApiUserUpdateParamsForRfdPermission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    group_ids: value.group_ids?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::ApiUserUpdateParamsForRfdPermission>
            for ApiUserUpdateParamsForRfdPermission
        {
            fn from(value: super::ApiUserUpdateParamsForRfdPermission) -> Self {
                Self {
                    group_ids: Ok(value.group_ids),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct CreateMapper {
            max_activations:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            rule: ::std::result::Result<::serde_json::Value, ::std::string::String>,
        }

        impl ::std::default::Default for CreateMapper {
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
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.max_activations = value.try_into().map_err(|e| {
                    format!("error converting supplied value for max_activations: {}", e)
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn rule<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.rule = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rule: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<CreateMapper> for super::CreateMapper {
            type Error = super::error::ConversionError;
            fn try_from(
                value: CreateMapper,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    max_activations: value.max_activations?,
                    name: value.name?,
                    rule: value.rule?,
                })
            }
        }

        impl ::std::convert::From<super::CreateMapper> for CreateMapper {
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
            error_code: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            message: ::std::result::Result<::std::string::String, ::std::string::String>,
            request_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Error {
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
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.error_code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for error_code: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn request_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.request_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for request_id: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Error> for super::Error {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Error,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    error_code: value.error_code?,
                    message: value.message?,
                    request_id: value.request_id?,
                })
            }
        }

        impl ::std::convert::From<super::Error> for Error {
            fn from(value: super::Error) -> Self {
                Self {
                    error_code: Ok(value.error_code),
                    message: Ok(value.message),
                    request_id: Ok(value.request_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct FormattedSearchResultHit {
            anchor: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            content: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            hierarchy: ::std::result::Result<
                [::std::option::Option<::std::string::String>; 6usize],
                ::std::string::String,
            >,
            hierarchy_radio: ::std::result::Result<
                [::std::option::Option<::std::string::String>; 6usize],
                ::std::string::String,
            >,
            object_id: ::std::result::Result<::std::string::String, ::std::string::String>,
            rfd_number: ::std::result::Result<u64, ::std::string::String>,
            url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for FormattedSearchResultHit {
            fn default() -> Self {
                Self {
                    anchor: Ok(Default::default()),
                    content: Ok(Default::default()),
                    hierarchy: Err("no value supplied for hierarchy".to_string()),
                    hierarchy_radio: Err("no value supplied for hierarchy_radio".to_string()),
                    object_id: Err("no value supplied for object_id".to_string()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    url: Ok(Default::default()),
                }
            }
        }

        impl FormattedSearchResultHit {
            pub fn anchor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.anchor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for anchor: {}", e));
                self
            }
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn hierarchy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<[::std::option::Option<::std::string::String>; 6usize]>,
                T::Error: ::std::fmt::Display,
            {
                self.hierarchy = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hierarchy: {}", e));
                self
            }
            pub fn hierarchy_radio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<[::std::option::Option<::std::string::String>; 6usize]>,
                T::Error: ::std::fmt::Display,
            {
                self.hierarchy_radio = value.try_into().map_err(|e| {
                    format!("error converting supplied value for hierarchy_radio: {}", e)
                });
                self
            }
            pub fn object_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.object_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for object_id: {}", e));
                self
            }
            pub fn rfd_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_number: {}", e));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<FormattedSearchResultHit> for super::FormattedSearchResultHit {
            type Error = super::error::ConversionError;
            fn try_from(
                value: FormattedSearchResultHit,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    anchor: value.anchor?,
                    content: value.content?,
                    hierarchy: value.hierarchy?,
                    hierarchy_radio: value.hierarchy_radio?,
                    object_id: value.object_id?,
                    rfd_number: value.rfd_number?,
                    url: value.url?,
                })
            }
        }

        impl ::std::convert::From<super::FormattedSearchResultHit> for FormattedSearchResultHit {
            fn from(value: super::FormattedSearchResultHit) -> Self {
                Self {
                    anchor: Ok(value.anchor),
                    content: Ok(value.content),
                    hierarchy: Ok(value.hierarchy),
                    hierarchy_radio: Ok(value.hierarchy_radio),
                    object_id: Ok(value.object_id),
                    rfd_number: Ok(value.rfd_number),
                    url: Ok(value.url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GetUserResponseForRfdPermission {
            info: ::std::result::Result<super::ApiUserForRfdPermission, ::std::string::String>,
            providers: ::std::result::Result<
                ::std::vec::Vec<super::ApiUserProvider>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GetUserResponseForRfdPermission {
            fn default() -> Self {
                Self {
                    info: Err("no value supplied for info".to_string()),
                    providers: Err("no value supplied for providers".to_string()),
                }
            }
        }

        impl GetUserResponseForRfdPermission {
            pub fn info<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ApiUserForRfdPermission>,
                T::Error: ::std::fmt::Display,
            {
                self.info = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for info: {}", e));
                self
            }
            pub fn providers<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::ApiUserProvider>>,
                T::Error: ::std::fmt::Display,
            {
                self.providers = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for providers: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<GetUserResponseForRfdPermission>
            for super::GetUserResponseForRfdPermission
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GetUserResponseForRfdPermission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    info: value.info?,
                    providers: value.providers?,
                })
            }
        }

        impl ::std::convert::From<super::GetUserResponseForRfdPermission>
            for GetUserResponseForRfdPermission
        {
            fn from(value: super::GetUserResponseForRfdPermission) -> Self {
                Self {
                    info: Ok(value.info),
                    providers: Ok(value.providers),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubCommit {
            added: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<::std::string::String, ::std::string::String>,
            modified: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            removed: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            timestamp: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for GitHubCommit {
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
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.added = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for added: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn modified<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.modified = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for modified: {}", e));
                self
            }
            pub fn removed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.removed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for removed: {}", e));
                self
            }
            pub fn timestamp<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.timestamp = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<GitHubCommit> for super::GitHubCommit {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GitHubCommit,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    added: value.added?,
                    id: value.id?,
                    modified: value.modified?,
                    removed: value.removed?,
                    timestamp: value.timestamp?,
                })
            }
        }

        impl ::std::convert::From<super::GitHubCommit> for GitHubCommit {
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
            commits:
                ::std::result::Result<::std::vec::Vec<super::GitHubCommit>, ::std::string::String>,
            head_commit: ::std::result::Result<
                ::std::option::Option<super::GitHubCommit>,
                ::std::string::String,
            >,
            installation: ::std::result::Result<super::GitHubInstallation, ::std::string::String>,
            ref_: ::std::result::Result<::std::string::String, ::std::string::String>,
            repository: ::std::result::Result<super::GitHubRepository, ::std::string::String>,
            sender: ::std::result::Result<super::GitHubSender, ::std::string::String>,
        }

        impl ::std::default::Default for GitHubCommitPayload {
            fn default() -> Self {
                Self {
                    commits: Err("no value supplied for commits".to_string()),
                    head_commit: Ok(Default::default()),
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
                T: ::std::convert::TryInto<::std::vec::Vec<super::GitHubCommit>>,
                T::Error: ::std::fmt::Display,
            {
                self.commits = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commits: {}", e));
                self
            }
            pub fn head_commit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::GitHubCommit>>,
                T::Error: ::std::fmt::Display,
            {
                self.head_commit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for head_commit: {}", e));
                self
            }
            pub fn installation<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GitHubInstallation>,
                T::Error: ::std::fmt::Display,
            {
                self.installation = value.try_into().map_err(|e| {
                    format!("error converting supplied value for installation: {}", e)
                });
                self
            }
            pub fn ref_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.ref_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for ref_: {}", e));
                self
            }
            pub fn repository<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GitHubRepository>,
                T::Error: ::std::fmt::Display,
            {
                self.repository = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for repository: {}", e));
                self
            }
            pub fn sender<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GitHubSender>,
                T::Error: ::std::fmt::Display,
            {
                self.sender = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sender: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<GitHubCommitPayload> for super::GitHubCommitPayload {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GitHubCommitPayload,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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

        impl ::std::convert::From<super::GitHubCommitPayload> for GitHubCommitPayload {
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
            id: ::std::result::Result<u64, ::std::string::String>,
            node_id: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GitHubInstallation {
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
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn node_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.node_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for node_id: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<GitHubInstallation> for super::GitHubInstallation {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GitHubInstallation,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    node_id: value.node_id?,
                })
            }
        }

        impl ::std::convert::From<super::GitHubInstallation> for GitHubInstallation {
            fn from(value: super::GitHubInstallation) -> Self {
                Self {
                    id: Ok(value.id),
                    node_id: Ok(value.node_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubRepository {
            default_branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<u64, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            node_id: ::std::result::Result<::std::string::String, ::std::string::String>,
            owner: ::std::result::Result<super::GitHubRepositoryOwner, ::std::string::String>,
        }

        impl ::std::default::Default for GitHubRepository {
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
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.default_branch = value.try_into().map_err(|e| {
                    format!("error converting supplied value for default_branch: {}", e)
                });
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn node_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.node_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for node_id: {}", e));
                self
            }
            pub fn owner<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::GitHubRepositoryOwner>,
                T::Error: ::std::fmt::Display,
            {
                self.owner = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for owner: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<GitHubRepository> for super::GitHubRepository {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GitHubRepository,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    default_branch: value.default_branch?,
                    id: value.id?,
                    name: value.name?,
                    node_id: value.node_id?,
                    owner: value.owner?,
                })
            }
        }

        impl ::std::convert::From<super::GitHubRepository> for GitHubRepository {
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
            login: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GitHubRepositoryOwner {
            fn default() -> Self {
                Self {
                    login: Err("no value supplied for login".to_string()),
                }
            }
        }

        impl GitHubRepositoryOwner {
            pub fn login<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.login = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for login: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<GitHubRepositoryOwner> for super::GitHubRepositoryOwner {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GitHubRepositoryOwner,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    login: value.login?,
                })
            }
        }

        impl ::std::convert::From<super::GitHubRepositoryOwner> for GitHubRepositoryOwner {
            fn from(value: super::GitHubRepositoryOwner) -> Self {
                Self {
                    login: Ok(value.login),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct GitHubSender {
            id: ::std::result::Result<u64, ::std::string::String>,
            login: ::std::result::Result<::std::string::String, ::std::string::String>,
            node_id: ::std::result::Result<::std::string::String, ::std::string::String>,
            type_: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for GitHubSender {
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
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn login<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.login = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for login: {}", e));
                self
            }
            pub fn node_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.node_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for node_id: {}", e));
                self
            }
            pub fn type_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.type_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for type_: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<GitHubSender> for super::GitHubSender {
            type Error = super::error::ConversionError;
            fn try_from(
                value: GitHubSender,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    id: value.id?,
                    login: value.login?,
                    node_id: value.node_id?,
                    type_: value.type_?,
                })
            }
        }

        impl ::std::convert::From<super::GitHubSender> for GitHubSender {
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
        pub struct InitialApiKeyResponseForRfdPermission {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForApiKeyId, ::std::string::String>,
            key: ::std::result::Result<super::SecretString, ::std::string::String>,
            permissions: ::std::result::Result<
                ::std::option::Option<super::PermissionsForRfdPermission>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for InitialApiKeyResponseForRfdPermission {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    key: Err("no value supplied for key".to_string()),
                    permissions: Ok(Default::default()),
                }
            }
        }

        impl InitialApiKeyResponseForRfdPermission {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForApiKeyId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SecretString>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
            pub fn permissions<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::PermissionsForRfdPermission>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.permissions = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for permissions: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InitialApiKeyResponseForRfdPermission>
            for super::InitialApiKeyResponseForRfdPermission
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InitialApiKeyResponseForRfdPermission,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                    permissions: value.permissions?,
                })
            }
        }

        impl ::std::convert::From<super::InitialApiKeyResponseForRfdPermission>
            for InitialApiKeyResponseForRfdPermission
        {
            fn from(value: super::InitialApiKeyResponseForRfdPermission) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                    permissions: Ok(value.permissions),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InitialMagicLinkSecretResponse {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForMagicLinkSecretId, ::std::string::String>,
            key: ::std::result::Result<super::SecretString, ::std::string::String>,
        }

        impl ::std::default::Default for InitialMagicLinkSecretResponse {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    key: Err("no value supplied for key".to_string()),
                }
            }
        }

        impl InitialMagicLinkSecretResponse {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkSecretId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SecretString>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InitialMagicLinkSecretResponse>
            for super::InitialMagicLinkSecretResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InitialMagicLinkSecretResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                })
            }
        }

        impl ::std::convert::From<super::InitialMagicLinkSecretResponse>
            for InitialMagicLinkSecretResponse
        {
            fn from(value: super::InitialMagicLinkSecretResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct InitialOAuthClientSecretResponse {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForOAuthSecretId, ::std::string::String>,
            key: ::std::result::Result<super::SecretString, ::std::string::String>,
        }

        impl ::std::default::Default for InitialOAuthClientSecretResponse {
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
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthSecretId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn key<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::SecretString>,
                T::Error: ::std::fmt::Display,
            {
                self.key = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for key: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<InitialOAuthClientSecretResponse>
            for super::InitialOAuthClientSecretResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: InitialOAuthClientSecretResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    id: value.id?,
                    key: value.key?,
                })
            }
        }

        impl ::std::convert::From<super::InitialOAuthClientSecretResponse>
            for InitialOAuthClientSecretResponse
        {
            fn from(value: super::InitialOAuthClientSecretResponse) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    key: Ok(value.key),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Job {
            branch: ::std::result::Result<::std::string::String, ::std::string::String>,
            committed_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            id: ::std::result::Result<i32, ::std::string::String>,
            owner: ::std::result::Result<::std::string::String, ::std::string::String>,
            processed: ::std::result::Result<bool, ::std::string::String>,
            repository: ::std::result::Result<::std::string::String, ::std::string::String>,
            rfd: ::std::result::Result<i32, ::std::string::String>,
            sha: ::std::result::Result<super::CommitSha, ::std::string::String>,
            started_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            webhook_delivery_id: ::std::result::Result<
                ::std::option::Option<super::TypedUuidForWebhookDeliveryId>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Job {
            fn default() -> Self {
                Self {
                    branch: Err("no value supplied for branch".to_string()),
                    committed_at: Err("no value supplied for committed_at".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    owner: Err("no value supplied for owner".to_string()),
                    processed: Err("no value supplied for processed".to_string()),
                    repository: Err("no value supplied for repository".to_string()),
                    rfd: Err("no value supplied for rfd".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                    started_at: Ok(Default::default()),
                    webhook_delivery_id: Ok(Default::default()),
                }
            }
        }

        impl Job {
            pub fn branch<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.branch = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for branch: {}", e));
                self
            }
            pub fn committed_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.committed_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_at: {}", e)
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn owner<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.owner = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for owner: {}", e));
                self
            }
            pub fn processed<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.processed = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for processed: {}", e));
                self
            }
            pub fn repository<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.repository = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for repository: {}", e));
                self
            }
            pub fn rfd<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd: {}", e));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::CommitSha>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {}", e));
                self
            }
            pub fn started_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.started_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for started_at: {}", e));
                self
            }
            pub fn webhook_delivery_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<super::TypedUuidForWebhookDeliveryId>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.webhook_delivery_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for webhook_delivery_id: {}",
                        e
                    )
                });
                self
            }
        }

        impl ::std::convert::TryFrom<Job> for super::Job {
            type Error = super::error::ConversionError;
            fn try_from(value: Job) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    branch: value.branch?,
                    committed_at: value.committed_at?,
                    created_at: value.created_at?,
                    id: value.id?,
                    owner: value.owner?,
                    processed: value.processed?,
                    repository: value.repository?,
                    rfd: value.rfd?,
                    sha: value.sha?,
                    started_at: value.started_at?,
                    webhook_delivery_id: value.webhook_delivery_id?,
                })
            }
        }

        impl ::std::convert::From<super::Job> for Job {
            fn from(value: super::Job) -> Self {
                Self {
                    branch: Ok(value.branch),
                    committed_at: Ok(value.committed_at),
                    created_at: Ok(value.created_at),
                    id: Ok(value.id),
                    owner: Ok(value.owner),
                    processed: Ok(value.processed),
                    repository: Ok(value.repository),
                    rfd: Ok(value.rfd),
                    sha: Ok(value.sha),
                    started_at: Ok(value.started_at),
                    webhook_delivery_id: Ok(value.webhook_delivery_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Jwk {
            e: ::std::result::Result<::std::string::String, ::std::string::String>,
            kid: ::std::result::Result<::std::string::String, ::std::string::String>,
            kty: ::std::result::Result<::std::string::String, ::std::string::String>,
            n: ::std::result::Result<::std::string::String, ::std::string::String>,
            use_: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for Jwk {
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
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.e = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for e: {}", e));
                self
            }
            pub fn kid<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.kid = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kid: {}", e));
                self
            }
            pub fn kty<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.kty = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for kty: {}", e));
                self
            }
            pub fn n<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.n = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for n: {}", e));
                self
            }
            pub fn use_<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.use_ = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for use_: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Jwk> for super::Jwk {
            type Error = super::error::ConversionError;
            fn try_from(value: Jwk) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    e: value.e?,
                    kid: value.kid?,
                    kty: value.kty?,
                    n: value.n?,
                    use_: value.use_?,
                })
            }
        }

        impl ::std::convert::From<super::Jwk> for Jwk {
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
            keys: ::std::result::Result<::std::vec::Vec<super::Jwk>, ::std::string::String>,
        }

        impl ::std::default::Default for Jwks {
            fn default() -> Self {
                Self {
                    keys: Err("no value supplied for keys".to_string()),
                }
            }
        }

        impl Jwks {
            pub fn keys<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::Jwk>>,
                T::Error: ::std::fmt::Display,
            {
                self.keys = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for keys: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Jwks> for super::Jwks {
            type Error = super::error::ConversionError;
            fn try_from(value: Jwks) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self { keys: value.keys? })
            }
        }

        impl ::std::convert::From<super::Jwks> for Jwks {
            fn from(value: super::Jwks) -> Self {
                Self {
                    keys: Ok(value.keys),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLink {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForMagicLinkId, ::std::string::String>,
            redirect_uris: ::std::result::Result<
                ::std::vec::Vec<super::MagicLinkRedirectUri>,
                ::std::string::String,
            >,
            secrets: ::std::result::Result<
                ::std::vec::Vec<super::MagicLinkSecret>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for MagicLink {
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

        impl MagicLink {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn redirect_uris<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MagicLinkRedirectUri>>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uris = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uris: {}", e)
                });
                self
            }
            pub fn secrets<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::MagicLinkSecret>>,
                T::Error: ::std::fmt::Display,
            {
                self.secrets = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secrets: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLink> for super::MagicLink {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLink,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    redirect_uris: value.redirect_uris?,
                    secrets: value.secrets?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLink> for MagicLink {
            fn from(value: super::MagicLink) -> Self {
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
        pub struct MagicLinkExchangeRequest {
            attempt_id:
                ::std::result::Result<super::TypedUuidForMagicLinkAttemptId, ::std::string::String>,
            recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
            secret: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkExchangeRequest {
            fn default() -> Self {
                Self {
                    attempt_id: Err("no value supplied for attempt_id".to_string()),
                    recipient: Err("no value supplied for recipient".to_string()),
                    secret: Err("no value supplied for secret".to_string()),
                }
            }
        }

        impl MagicLinkExchangeRequest {
            pub fn attempt_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkAttemptId>,
                T::Error: ::std::fmt::Display,
            {
                self.attempt_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attempt_id: {}", e));
                self
            }
            pub fn recipient<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.recipient = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recipient: {}", e));
                self
            }
            pub fn secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkExchangeRequest> for super::MagicLinkExchangeRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkExchangeRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attempt_id: value.attempt_id?,
                    recipient: value.recipient?,
                    secret: value.secret?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkExchangeRequest> for MagicLinkExchangeRequest {
            fn from(value: super::MagicLinkExchangeRequest) -> Self {
                Self {
                    attempt_id: Ok(value.attempt_id),
                    recipient: Ok(value.recipient),
                    secret: Ok(value.secret),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkExchangeResponse {
            access_token: ::std::result::Result<::std::string::String, ::std::string::String>,
            expires_in: ::std::result::Result<i64, ::std::string::String>,
            token_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkExchangeResponse {
            fn default() -> Self {
                Self {
                    access_token: Err("no value supplied for access_token".to_string()),
                    expires_in: Err("no value supplied for expires_in".to_string()),
                    token_type: Err("no value supplied for token_type".to_string()),
                }
            }
        }

        impl MagicLinkExchangeResponse {
            pub fn access_token<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.access_token = value.try_into().map_err(|e| {
                    format!("error converting supplied value for access_token: {}", e)
                });
                self
            }
            pub fn expires_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i64>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_in = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_in: {}", e));
                self
            }
            pub fn token_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token_type: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkExchangeResponse> for super::MagicLinkExchangeResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkExchangeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    access_token: value.access_token?,
                    expires_in: value.expires_in?,
                    token_type: value.token_type?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkExchangeResponse> for MagicLinkExchangeResponse {
            fn from(value: super::MagicLinkExchangeResponse) -> Self {
                Self {
                    access_token: Ok(value.access_token),
                    expires_in: Ok(value.expires_in),
                    token_type: Ok(value.token_type),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkRedirectUri {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<
                super::TypedUuidForMagicLinkRedirectUriId,
                ::std::string::String,
            >,
            magic_link_client_id:
                ::std::result::Result<super::TypedUuidForMagicLinkId, ::std::string::String>,
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkRedirectUri {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    magic_link_client_id: Err(
                        "no value supplied for magic_link_client_id".to_string()
                    ),
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                }
            }
        }

        impl MagicLinkRedirectUri {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkRedirectUriId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn magic_link_client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkId>,
                T::Error: ::std::fmt::Display,
            {
                self.magic_link_client_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for magic_link_client_id: {}",
                        e
                    )
                });
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkRedirectUri> for super::MagicLinkRedirectUri {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkRedirectUri,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    magic_link_client_id: value.magic_link_client_id?,
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkRedirectUri> for MagicLinkRedirectUri {
            fn from(value: super::MagicLinkRedirectUri) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    magic_link_client_id: Ok(value.magic_link_client_id),
                    redirect_uri: Ok(value.redirect_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkSecret {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForMagicLinkSecretId, ::std::string::String>,
            magic_link_client_id:
                ::std::result::Result<super::TypedUuidForMagicLinkId, ::std::string::String>,
            secret_signature: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkSecret {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    magic_link_client_id: Err(
                        "no value supplied for magic_link_client_id".to_string()
                    ),
                    secret_signature: Err("no value supplied for secret_signature".to_string()),
                }
            }
        }

        impl MagicLinkSecret {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkSecretId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn magic_link_client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkId>,
                T::Error: ::std::fmt::Display,
            {
                self.magic_link_client_id = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for magic_link_client_id: {}",
                        e
                    )
                });
                self
            }
            pub fn secret_signature<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
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

        impl ::std::convert::TryFrom<MagicLinkSecret> for super::MagicLinkSecret {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkSecret,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    magic_link_client_id: value.magic_link_client_id?,
                    secret_signature: value.secret_signature?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkSecret> for MagicLinkSecret {
            fn from(value: super::MagicLinkSecret) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    magic_link_client_id: Ok(value.magic_link_client_id),
                    secret_signature: Ok(value.secret_signature),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkSendRequest {
            expires_in: ::std::result::Result<i64, ::std::string::String>,
            medium: ::std::result::Result<super::MagicLinkMedium, ::std::string::String>,
            recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
            scope: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            secret: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkSendRequest {
            fn default() -> Self {
                Self {
                    expires_in: Err("no value supplied for expires_in".to_string()),
                    medium: Err("no value supplied for medium".to_string()),
                    recipient: Err("no value supplied for recipient".to_string()),
                    redirect_uri: Err("no value supplied for redirect_uri".to_string()),
                    scope: Ok(Default::default()),
                    secret: Err("no value supplied for secret".to_string()),
                }
            }
        }

        impl MagicLinkSendRequest {
            pub fn expires_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i64>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_in = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_in: {}", e));
                self
            }
            pub fn medium<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::MagicLinkMedium>,
                T::Error: ::std::fmt::Display,
            {
                self.medium = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for medium: {}", e));
                self
            }
            pub fn recipient<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.recipient = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for recipient: {}", e));
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
            pub fn scope<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.scope = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scope: {}", e));
                self
            }
            pub fn secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.secret = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secret: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkSendRequest> for super::MagicLinkSendRequest {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkSendRequest,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    expires_in: value.expires_in?,
                    medium: value.medium?,
                    recipient: value.recipient?,
                    redirect_uri: value.redirect_uri?,
                    scope: value.scope?,
                    secret: value.secret?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkSendRequest> for MagicLinkSendRequest {
            fn from(value: super::MagicLinkSendRequest) -> Self {
                Self {
                    expires_in: Ok(value.expires_in),
                    medium: Ok(value.medium),
                    recipient: Ok(value.recipient),
                    redirect_uri: Ok(value.redirect_uri),
                    scope: Ok(value.scope),
                    secret: Ok(value.secret),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct MagicLinkSendResponse {
            attempt_id:
                ::std::result::Result<super::TypedUuidForMagicLinkAttemptId, ::std::string::String>,
        }

        impl ::std::default::Default for MagicLinkSendResponse {
            fn default() -> Self {
                Self {
                    attempt_id: Err("no value supplied for attempt_id".to_string()),
                }
            }
        }

        impl MagicLinkSendResponse {
            pub fn attempt_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMagicLinkAttemptId>,
                T::Error: ::std::fmt::Display,
            {
                self.attempt_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for attempt_id: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<MagicLinkSendResponse> for super::MagicLinkSendResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: MagicLinkSendResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    attempt_id: value.attempt_id?,
                })
            }
        }

        impl ::std::convert::From<super::MagicLinkSendResponse> for MagicLinkSendResponse {
            fn from(value: super::MagicLinkSendResponse) -> Self {
                Self {
                    attempt_id: Ok(value.attempt_id),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Mapper {
            activations: ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            depleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForMapperId, ::std::string::String>,
            max_activations:
                ::std::result::Result<::std::option::Option<i32>, ::std::string::String>,
            name: ::std::result::Result<::std::string::String, ::std::string::String>,
            rule: ::std::result::Result<::serde_json::Value, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for Mapper {
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
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl Mapper {
            pub fn activations<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.activations = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for activations: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn depleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.depleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for depleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForMapperId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn max_activations<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<i32>>,
                T::Error: ::std::fmt::Display,
            {
                self.max_activations = value.try_into().map_err(|e| {
                    format!("error converting supplied value for max_activations: {}", e)
                });
                self
            }
            pub fn name<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.name = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for name: {}", e));
                self
            }
            pub fn rule<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::serde_json::Value>,
                T::Error: ::std::fmt::Display,
            {
                self.rule = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rule: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Mapper> for super::Mapper {
            type Error = super::error::ConversionError;
            fn try_from(
                value: Mapper,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    activations: value.activations?,
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    depleted_at: value.depleted_at?,
                    id: value.id?,
                    max_activations: value.max_activations?,
                    name: value.name?,
                    rule: value.rule?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl ::std::convert::From<super::Mapper> for Mapper {
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
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct OAuthAuthzCodeExchangeBody {
            client_id: ::std::result::Result<
                ::std::option::Option<super::TypedUuidForOAuthClientId>,
                ::std::string::String,
            >,
            client_secret: ::std::result::Result<
                ::std::option::Option<super::SecretString>,
                ::std::string::String,
            >,
            code: ::std::result::Result<::std::string::String, ::std::string::String>,
            grant_type: ::std::result::Result<::std::string::String, ::std::string::String>,
            pkce_verifier: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthAuthzCodeExchangeBody {
            fn default() -> Self {
                Self {
                    client_id: Ok(Default::default()),
                    client_secret: Ok(Default::default()),
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
                T: ::std::convert::TryInto<::std::option::Option<super::TypedUuidForOAuthClientId>>,
                T::Error: ::std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {}", e));
                self
            }
            pub fn client_secret<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::SecretString>>,
                T::Error: ::std::fmt::Display,
            {
                self.client_secret = value.try_into().map_err(|e| {
                    format!("error converting supplied value for client_secret: {}", e)
                });
                self
            }
            pub fn code<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.code = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for code: {}", e));
                self
            }
            pub fn grant_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.grant_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for grant_type: {}", e));
                self
            }
            pub fn pkce_verifier<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.pkce_verifier = value.try_into().map_err(|e| {
                    format!("error converting supplied value for pkce_verifier: {}", e)
                });
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthAuthzCodeExchangeBody> for super::OAuthAuthzCodeExchangeBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthAuthzCodeExchangeBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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

        impl ::std::convert::From<super::OAuthAuthzCodeExchangeBody> for OAuthAuthzCodeExchangeBody {
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
            access_token: ::std::result::Result<::std::string::String, ::std::string::String>,
            expires_in: ::std::result::Result<i64, ::std::string::String>,
            token_type: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthAuthzCodeExchangeResponse {
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
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.access_token = value.try_into().map_err(|e| {
                    format!("error converting supplied value for access_token: {}", e)
                });
                self
            }
            pub fn expires_in<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i64>,
                T::Error: ::std::fmt::Display,
            {
                self.expires_in = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for expires_in: {}", e));
                self
            }
            pub fn token_type<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token_type = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for token_type: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthAuthzCodeExchangeResponse>
            for super::OAuthAuthzCodeExchangeResponse
        {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthAuthzCodeExchangeResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    access_token: value.access_token?,
                    expires_in: value.expires_in?,
                    token_type: value.token_type?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthAuthzCodeExchangeResponse>
            for OAuthAuthzCodeExchangeResponse
        {
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
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            redirect_uris: ::std::result::Result<
                ::std::vec::Vec<super::OAuthClientRedirectUri>,
                ::std::string::String,
            >,
            secrets: ::std::result::Result<
                ::std::vec::Vec<super::OAuthClientSecret>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for OAuthClient {
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
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn redirect_uris<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OAuthClientRedirectUri>>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uris = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uris: {}", e)
                });
                self
            }
            pub fn secrets<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::OAuthClientSecret>>,
                T::Error: ::std::fmt::Display,
            {
                self.secrets = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for secrets: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthClient> for super::OAuthClient {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthClient,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    redirect_uris: value.redirect_uris?,
                    secrets: value.secrets?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthClient> for OAuthClient {
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
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForOAuthRedirectUriId, ::std::string::String>,
            oauth_client_id:
                ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            redirect_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthClientRedirectUri {
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
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthRedirectUriId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn oauth_client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.oauth_client_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for oauth_client_id: {}", e)
                });
                self
            }
            pub fn redirect_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.redirect_uri = value.try_into().map_err(|e| {
                    format!("error converting supplied value for redirect_uri: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthClientRedirectUri> for super::OAuthClientRedirectUri {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthClientRedirectUri,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    oauth_client_id: value.oauth_client_id?,
                    redirect_uri: value.redirect_uri?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthClientRedirectUri> for OAuthClientRedirectUri {
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
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForOAuthSecretId, ::std::string::String>,
            oauth_client_id:
                ::std::result::Result<super::TypedUuidForOAuthClientId, ::std::string::String>,
            secret_signature: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthClientSecret {
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
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthSecretId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn oauth_client_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForOAuthClientId>,
                T::Error: ::std::fmt::Display,
            {
                self.oauth_client_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for oauth_client_id: {}", e)
                });
                self
            }
            pub fn secret_signature<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
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

        impl ::std::convert::TryFrom<OAuthClientSecret> for super::OAuthClientSecret {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthClientSecret,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    oauth_client_id: value.oauth_client_id?,
                    secret_signature: value.secret_signature?,
                })
            }
        }

        impl ::std::convert::From<super::OAuthClientSecret> for OAuthClientSecret {
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
            auth_url_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
            client_id: ::std::result::Result<::std::string::String, ::std::string::String>,
            device_code_endpoint:
                ::std::result::Result<::std::string::String, ::std::string::String>,
            provider: ::std::result::Result<super::OAuthProviderName, ::std::string::String>,
            scopes: ::std::result::Result<
                ::std::vec::Vec<::std::string::String>,
                ::std::string::String,
            >,
            token_endpoint: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OAuthProviderInfo {
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
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
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
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.client_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for client_id: {}", e));
                self
            }
            pub fn device_code_endpoint<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
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
                T: ::std::convert::TryInto<super::OAuthProviderName>,
                T::Error: ::std::fmt::Display,
            {
                self.provider = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for provider: {}", e));
                self
            }
            pub fn scopes<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.scopes = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for scopes: {}", e));
                self
            }
            pub fn token_endpoint<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.token_endpoint = value.try_into().map_err(|e| {
                    format!("error converting supplied value for token_endpoint: {}", e)
                });
                self
            }
        }

        impl ::std::convert::TryFrom<OAuthProviderInfo> for super::OAuthProviderInfo {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OAuthProviderInfo,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
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

        impl ::std::convert::From<super::OAuthProviderInfo> for OAuthProviderInfo {
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
            jwks_uri: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for OpenIdConfiguration {
            fn default() -> Self {
                Self {
                    jwks_uri: Err("no value supplied for jwks_uri".to_string()),
                }
            }
        }

        impl OpenIdConfiguration {
            pub fn jwks_uri<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.jwks_uri = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for jwks_uri: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<OpenIdConfiguration> for super::OpenIdConfiguration {
            type Error = super::error::ConversionError;
            fn try_from(
                value: OpenIdConfiguration,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    jwks_uri: value.jwks_uri?,
                })
            }
        }

        impl ::std::convert::From<super::OpenIdConfiguration> for OpenIdConfiguration {
            fn from(value: super::OpenIdConfiguration) -> Self {
                Self {
                    jwks_uri: Ok(value.jwks_uri),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReserveRfdBody {
            content: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            title: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for ReserveRfdBody {
            fn default() -> Self {
                Self {
                    content: Ok(Default::default()),
                    title: Err("no value supplied for title".to_string()),
                }
            }
        }

        impl ReserveRfdBody {
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ReserveRfdBody> for super::ReserveRfdBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReserveRfdBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    content: value.content?,
                    title: value.title?,
                })
            }
        }

        impl ::std::convert::From<super::ReserveRfdBody> for ReserveRfdBody {
            fn from(value: super::ReserveRfdBody) -> Self {
                Self {
                    content: Ok(value.content),
                    title: Ok(value.title),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct ReserveRfdResponse {
            number: ::std::result::Result<i32, ::std::string::String>,
        }

        impl ::std::default::Default for ReserveRfdResponse {
            fn default() -> Self {
                Self {
                    number: Err("no value supplied for number".to_string()),
                }
            }
        }

        impl ReserveRfdResponse {
            pub fn number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for number: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<ReserveRfdResponse> for super::ReserveRfdResponse {
            type Error = super::error::ConversionError;
            fn try_from(
                value: ReserveRfdResponse,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    number: value.number?,
                })
            }
        }

        impl ::std::convert::From<super::ReserveRfdResponse> for ReserveRfdResponse {
            fn from(value: super::ReserveRfdResponse) -> Self {
                Self {
                    number: Ok(value.number),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct Rfd {
            content: ::std::result::Result<
                ::std::option::Option<super::RfdRevision>,
                ::std::string::String,
            >,
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForRfdId, ::std::string::String>,
            latest_major_change_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            link: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            rfd_number: ::std::result::Result<i32, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            visibility: ::std::result::Result<super::Visibility, ::std::string::String>,
        }

        impl ::std::default::Default for Rfd {
            fn default() -> Self {
                Self {
                    content: Ok(Default::default()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    latest_major_change_at: Ok(Default::default()),
                    link: Ok(Default::default()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                    visibility: Err("no value supplied for visibility".to_string()),
                }
            }
        }

        impl Rfd {
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::RfdRevision>>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn latest_major_change_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.latest_major_change_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for latest_major_change_at: {}",
                        e
                    )
                });
                self
            }
            pub fn link<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.link = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link: {}", e));
                self
            }
            pub fn rfd_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_number: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Visibility>,
                T::Error: ::std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<Rfd> for super::Rfd {
            type Error = super::error::ConversionError;
            fn try_from(value: Rfd) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    content: value.content?,
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    id: value.id?,
                    latest_major_change_at: value.latest_major_change_at?,
                    link: value.link?,
                    rfd_number: value.rfd_number?,
                    updated_at: value.updated_at?,
                    visibility: value.visibility?,
                })
            }
        }

        impl ::std::convert::From<super::Rfd> for Rfd {
            fn from(value: super::Rfd) -> Self {
                Self {
                    content: Ok(value.content),
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    id: Ok(value.id),
                    latest_major_change_at: Ok(value.latest_major_change_at),
                    link: Ok(value.link),
                    rfd_number: Ok(value.rfd_number),
                    updated_at: Ok(value.updated_at),
                    visibility: Ok(value.visibility),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RfdAttrValue {
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            value: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for RfdAttrValue {
            fn default() -> Self {
                Self {
                    message: Ok(Default::default()),
                    value: Err("no value supplied for value".to_string()),
                }
            }
        }

        impl RfdAttrValue {
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
            pub fn value<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.value = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for value: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdAttrValue> for super::RfdAttrValue {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdAttrValue,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    message: value.message?,
                    value: value.value?,
                })
            }
        }

        impl ::std::convert::From<super::RfdAttrValue> for RfdAttrValue {
            fn from(value: super::RfdAttrValue) -> Self {
                Self {
                    message: Ok(value.message),
                    value: Ok(value.value),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RfdPdf {
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            external_id: ::std::result::Result<::std::string::String, ::std::string::String>,
            id: ::std::result::Result<super::TypedUuidForRfdPdfId, ::std::string::String>,
            link: ::std::result::Result<::std::string::String, ::std::string::String>,
            rfd_id: ::std::result::Result<super::TypedUuidForRfdId, ::std::string::String>,
            rfd_revision_id:
                ::std::result::Result<super::TypedUuidForRfdRevisionId, ::std::string::String>,
            source: ::std::result::Result<super::PdfSource, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for RfdPdf {
            fn default() -> Self {
                Self {
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    external_id: Err("no value supplied for external_id".to_string()),
                    id: Err("no value supplied for id".to_string()),
                    link: Err("no value supplied for link".to_string()),
                    rfd_id: Err("no value supplied for rfd_id".to_string()),
                    rfd_revision_id: Err("no value supplied for rfd_revision_id".to_string()),
                    source: Err("no value supplied for source".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl RfdPdf {
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn external_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.external_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for external_id: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdPdfId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn link<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.link = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link: {}", e));
                self
            }
            pub fn rfd_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdId>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_id: {}", e));
                self
            }
            pub fn rfd_revision_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdRevisionId>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_revision_id = value.try_into().map_err(|e| {
                    format!("error converting supplied value for rfd_revision_id: {}", e)
                });
                self
            }
            pub fn source<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::PdfSource>,
                T::Error: ::std::fmt::Display,
            {
                self.source = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for source: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdPdf> for super::RfdPdf {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdPdf,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    external_id: value.external_id?,
                    id: value.id?,
                    link: value.link?,
                    rfd_id: value.rfd_id?,
                    rfd_revision_id: value.rfd_revision_id?,
                    source: value.source?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl ::std::convert::From<super::RfdPdf> for RfdPdf {
            fn from(value: super::RfdPdf) -> Self {
                Self {
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    external_id: Ok(value.external_id),
                    id: Ok(value.id),
                    link: Ok(value.link),
                    rfd_id: Ok(value.rfd_id),
                    rfd_revision_id: Ok(value.rfd_revision_id),
                    source: Ok(value.source),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RfdRevision {
            authors: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            commit: ::std::result::Result<super::CommitSha, ::std::string::String>,
            committed_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            content: ::std::result::Result<::std::string::String, ::std::string::String>,
            content_format: ::std::result::Result<super::ContentFormat, ::std::string::String>,
            created_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
            deleted_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            discussion: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForRfdRevisionId, ::std::string::String>,
            labels: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            major_change: ::std::result::Result<bool, ::std::string::String>,
            rfd_id: ::std::result::Result<super::TypedUuidForRfdId, ::std::string::String>,
            sha: ::std::result::Result<super::FileSha, ::std::string::String>,
            state: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            title: ::std::result::Result<::std::string::String, ::std::string::String>,
            updated_at: ::std::result::Result<
                ::chrono::DateTime<::chrono::offset::Utc>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for RfdRevision {
            fn default() -> Self {
                Self {
                    authors: Ok(Default::default()),
                    commit: Err("no value supplied for commit".to_string()),
                    committed_at: Err("no value supplied for committed_at".to_string()),
                    content: Err("no value supplied for content".to_string()),
                    content_format: Err("no value supplied for content_format".to_string()),
                    created_at: Err("no value supplied for created_at".to_string()),
                    deleted_at: Ok(Default::default()),
                    discussion: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    labels: Ok(Default::default()),
                    major_change: Err("no value supplied for major_change".to_string()),
                    rfd_id: Err("no value supplied for rfd_id".to_string()),
                    sha: Err("no value supplied for sha".to_string()),
                    state: Ok(Default::default()),
                    title: Err("no value supplied for title".to_string()),
                    updated_at: Err("no value supplied for updated_at".to_string()),
                }
            }
        }

        impl RfdRevision {
            pub fn authors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.authors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for authors: {}", e));
                self
            }
            pub fn commit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::CommitSha>,
                T::Error: ::std::fmt::Display,
            {
                self.commit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit: {}", e));
                self
            }
            pub fn committed_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.committed_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_at: {}", e)
                });
                self
            }
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn content_format<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::ContentFormat>,
                T::Error: ::std::fmt::Display,
            {
                self.content_format = value.try_into().map_err(|e| {
                    format!("error converting supplied value for content_format: {}", e)
                });
                self
            }
            pub fn created_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.created_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for created_at: {}", e));
                self
            }
            pub fn deleted_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.deleted_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for deleted_at: {}", e));
                self
            }
            pub fn discussion<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.discussion = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for discussion: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdRevisionId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn labels<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.labels = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for labels: {}", e));
                self
            }
            pub fn major_change<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<bool>,
                T::Error: ::std::fmt::Display,
            {
                self.major_change = value.try_into().map_err(|e| {
                    format!("error converting supplied value for major_change: {}", e)
                });
                self
            }
            pub fn rfd_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdId>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_id: {}", e));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::FileSha>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn updated_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::chrono::DateTime<::chrono::offset::Utc>>,
                T::Error: ::std::fmt::Display,
            {
                self.updated_at = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for updated_at: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdRevision> for super::RfdRevision {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdRevision,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    authors: value.authors?,
                    commit: value.commit?,
                    committed_at: value.committed_at?,
                    content: value.content?,
                    content_format: value.content_format?,
                    created_at: value.created_at?,
                    deleted_at: value.deleted_at?,
                    discussion: value.discussion?,
                    id: value.id?,
                    labels: value.labels?,
                    major_change: value.major_change?,
                    rfd_id: value.rfd_id?,
                    sha: value.sha?,
                    state: value.state?,
                    title: value.title?,
                    updated_at: value.updated_at?,
                })
            }
        }

        impl ::std::convert::From<super::RfdRevision> for RfdRevision {
            fn from(value: super::RfdRevision) -> Self {
                Self {
                    authors: Ok(value.authors),
                    commit: Ok(value.commit),
                    committed_at: Ok(value.committed_at),
                    content: Ok(value.content),
                    content_format: Ok(value.content_format),
                    created_at: Ok(value.created_at),
                    deleted_at: Ok(value.deleted_at),
                    discussion: Ok(value.discussion),
                    id: Ok(value.id),
                    labels: Ok(value.labels),
                    major_change: Ok(value.major_change),
                    rfd_id: Ok(value.rfd_id),
                    sha: Ok(value.sha),
                    state: Ok(value.state),
                    title: Ok(value.title),
                    updated_at: Ok(value.updated_at),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RfdUpdateBody {
            document: ::std::result::Result<::std::string::String, ::std::string::String>,
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for RfdUpdateBody {
            fn default() -> Self {
                Self {
                    document: Err("no value supplied for document".to_string()),
                    message: Ok(Default::default()),
                }
            }
        }

        impl RfdUpdateBody {
            pub fn document<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.document = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for document: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdUpdateBody> for super::RfdUpdateBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdUpdateBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    document: value.document?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::RfdUpdateBody> for RfdUpdateBody {
            fn from(value: super::RfdUpdateBody) -> Self {
                Self {
                    document: Ok(value.document),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RfdUpdateContentBody {
            content: ::std::result::Result<::std::string::String, ::std::string::String>,
            message: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for RfdUpdateContentBody {
            fn default() -> Self {
                Self {
                    content: Err("no value supplied for content".to_string()),
                    message: Ok(Default::default()),
                }
            }
        }

        impl RfdUpdateContentBody {
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn message<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.message = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for message: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdUpdateContentBody> for super::RfdUpdateContentBody {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdUpdateContentBody,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    content: value.content?,
                    message: value.message?,
                })
            }
        }

        impl ::std::convert::From<super::RfdUpdateContentBody> for RfdUpdateContentBody {
            fn from(value: super::RfdUpdateContentBody) -> Self {
                Self {
                    content: Ok(value.content),
                    message: Ok(value.message),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RfdVisibility {
            visibility: ::std::result::Result<super::Visibility, ::std::string::String>,
        }

        impl ::std::default::Default for RfdVisibility {
            fn default() -> Self {
                Self {
                    visibility: Err("no value supplied for visibility".to_string()),
                }
            }
        }

        impl RfdVisibility {
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Visibility>,
                T::Error: ::std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdVisibility> for super::RfdVisibility {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdVisibility,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    visibility: value.visibility?,
                })
            }
        }

        impl ::std::convert::From<super::RfdVisibility> for RfdVisibility {
            fn from(value: super::RfdVisibility) -> Self {
                Self {
                    visibility: Ok(value.visibility),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct RfdWithPdf {
            authors: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            commit: ::std::result::Result<
                ::std::option::Option<super::CommitSha>,
                ::std::string::String,
            >,
            committed_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            content: ::std::result::Result<::std::vec::Vec<super::RfdPdf>, ::std::string::String>,
            discussion: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            format: ::std::result::Result<
                ::std::option::Option<super::ContentFormat>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForRfdId, ::std::string::String>,
            labels: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            latest_major_change_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            link: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            rfd_number: ::std::result::Result<i32, ::std::string::String>,
            sha:
                ::std::result::Result<::std::option::Option<super::FileSha>, ::std::string::String>,
            state: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            visibility: ::std::result::Result<super::Visibility, ::std::string::String>,
        }

        impl ::std::default::Default for RfdWithPdf {
            fn default() -> Self {
                Self {
                    authors: Ok(Default::default()),
                    commit: Ok(Default::default()),
                    committed_at: Ok(Default::default()),
                    content: Err("no value supplied for content".to_string()),
                    discussion: Ok(Default::default()),
                    format: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    labels: Ok(Default::default()),
                    latest_major_change_at: Ok(Default::default()),
                    link: Ok(Default::default()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    sha: Ok(Default::default()),
                    state: Ok(Default::default()),
                    title: Ok(Default::default()),
                    visibility: Err("no value supplied for visibility".to_string()),
                }
            }
        }

        impl RfdWithPdf {
            pub fn authors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.authors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for authors: {}", e));
                self
            }
            pub fn commit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::CommitSha>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit: {}", e));
                self
            }
            pub fn committed_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.committed_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_at: {}", e)
                });
                self
            }
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::RfdPdf>>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn discussion<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.discussion = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for discussion: {}", e));
                self
            }
            pub fn format<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ContentFormat>>,
                T::Error: ::std::fmt::Display,
            {
                self.format = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for format: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn labels<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.labels = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for labels: {}", e));
                self
            }
            pub fn latest_major_change_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.latest_major_change_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for latest_major_change_at: {}",
                        e
                    )
                });
                self
            }
            pub fn link<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.link = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link: {}", e));
                self
            }
            pub fn rfd_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_number: {}", e));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FileSha>>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Visibility>,
                T::Error: ::std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdWithPdf> for super::RfdWithPdf {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdWithPdf,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    authors: value.authors?,
                    commit: value.commit?,
                    committed_at: value.committed_at?,
                    content: value.content?,
                    discussion: value.discussion?,
                    format: value.format?,
                    id: value.id?,
                    labels: value.labels?,
                    latest_major_change_at: value.latest_major_change_at?,
                    link: value.link?,
                    rfd_number: value.rfd_number?,
                    sha: value.sha?,
                    state: value.state?,
                    title: value.title?,
                    visibility: value.visibility?,
                })
            }
        }

        impl ::std::convert::From<super::RfdWithPdf> for RfdWithPdf {
            fn from(value: super::RfdWithPdf) -> Self {
                Self {
                    authors: Ok(value.authors),
                    commit: Ok(value.commit),
                    committed_at: Ok(value.committed_at),
                    content: Ok(value.content),
                    discussion: Ok(value.discussion),
                    format: Ok(value.format),
                    id: Ok(value.id),
                    labels: Ok(value.labels),
                    latest_major_change_at: Ok(value.latest_major_change_at),
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
        pub struct RfdWithRaw {
            authors: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            commit: ::std::result::Result<
                ::std::option::Option<super::CommitSha>,
                ::std::string::String,
            >,
            committed_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            content: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            discussion: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            format: ::std::result::Result<
                ::std::option::Option<super::ContentFormat>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForRfdId, ::std::string::String>,
            labels: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            latest_major_change_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            link: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            rfd_number: ::std::result::Result<i32, ::std::string::String>,
            sha:
                ::std::result::Result<::std::option::Option<super::FileSha>, ::std::string::String>,
            state: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            visibility: ::std::result::Result<super::Visibility, ::std::string::String>,
        }

        impl ::std::default::Default for RfdWithRaw {
            fn default() -> Self {
                Self {
                    authors: Ok(Default::default()),
                    commit: Ok(Default::default()),
                    committed_at: Ok(Default::default()),
                    content: Ok(Default::default()),
                    discussion: Ok(Default::default()),
                    format: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    labels: Ok(Default::default()),
                    latest_major_change_at: Ok(Default::default()),
                    link: Ok(Default::default()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    sha: Ok(Default::default()),
                    state: Ok(Default::default()),
                    title: Ok(Default::default()),
                    visibility: Err("no value supplied for visibility".to_string()),
                }
            }
        }

        impl RfdWithRaw {
            pub fn authors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.authors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for authors: {}", e));
                self
            }
            pub fn commit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::CommitSha>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit: {}", e));
                self
            }
            pub fn committed_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.committed_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_at: {}", e)
                });
                self
            }
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn discussion<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.discussion = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for discussion: {}", e));
                self
            }
            pub fn format<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ContentFormat>>,
                T::Error: ::std::fmt::Display,
            {
                self.format = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for format: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn labels<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.labels = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for labels: {}", e));
                self
            }
            pub fn latest_major_change_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.latest_major_change_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for latest_major_change_at: {}",
                        e
                    )
                });
                self
            }
            pub fn link<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.link = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link: {}", e));
                self
            }
            pub fn rfd_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_number: {}", e));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FileSha>>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Visibility>,
                T::Error: ::std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdWithRaw> for super::RfdWithRaw {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdWithRaw,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    authors: value.authors?,
                    commit: value.commit?,
                    committed_at: value.committed_at?,
                    content: value.content?,
                    discussion: value.discussion?,
                    format: value.format?,
                    id: value.id?,
                    labels: value.labels?,
                    latest_major_change_at: value.latest_major_change_at?,
                    link: value.link?,
                    rfd_number: value.rfd_number?,
                    sha: value.sha?,
                    state: value.state?,
                    title: value.title?,
                    visibility: value.visibility?,
                })
            }
        }

        impl ::std::convert::From<super::RfdWithRaw> for RfdWithRaw {
            fn from(value: super::RfdWithRaw) -> Self {
                Self {
                    authors: Ok(value.authors),
                    commit: Ok(value.commit),
                    committed_at: Ok(value.committed_at),
                    content: Ok(value.content),
                    discussion: Ok(value.discussion),
                    format: Ok(value.format),
                    id: Ok(value.id),
                    labels: Ok(value.labels),
                    latest_major_change_at: Ok(value.latest_major_change_at),
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
        pub struct RfdWithoutContent {
            authors: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            commit: ::std::result::Result<
                ::std::option::Option<super::CommitSha>,
                ::std::string::String,
            >,
            committed_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            discussion: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            format: ::std::result::Result<
                ::std::option::Option<super::ContentFormat>,
                ::std::string::String,
            >,
            id: ::std::result::Result<super::TypedUuidForRfdId, ::std::string::String>,
            labels: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            latest_major_change_at: ::std::result::Result<
                ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                ::std::string::String,
            >,
            link: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            rfd_number: ::std::result::Result<i32, ::std::string::String>,
            sha:
                ::std::result::Result<::std::option::Option<super::FileSha>, ::std::string::String>,
            state: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            title: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            visibility: ::std::result::Result<super::Visibility, ::std::string::String>,
        }

        impl ::std::default::Default for RfdWithoutContent {
            fn default() -> Self {
                Self {
                    authors: Ok(Default::default()),
                    commit: Ok(Default::default()),
                    committed_at: Ok(Default::default()),
                    discussion: Ok(Default::default()),
                    format: Ok(Default::default()),
                    id: Err("no value supplied for id".to_string()),
                    labels: Ok(Default::default()),
                    latest_major_change_at: Ok(Default::default()),
                    link: Ok(Default::default()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    sha: Ok(Default::default()),
                    state: Ok(Default::default()),
                    title: Ok(Default::default()),
                    visibility: Err("no value supplied for visibility".to_string()),
                }
            }
        }

        impl RfdWithoutContent {
            pub fn authors<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.authors = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for authors: {}", e));
                self
            }
            pub fn commit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::CommitSha>>,
                T::Error: ::std::fmt::Display,
            {
                self.commit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for commit: {}", e));
                self
            }
            pub fn committed_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.committed_at = value.try_into().map_err(|e| {
                    format!("error converting supplied value for committed_at: {}", e)
                });
                self
            }
            pub fn discussion<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.discussion = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for discussion: {}", e));
                self
            }
            pub fn format<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::ContentFormat>>,
                T::Error: ::std::fmt::Display,
            {
                self.format = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for format: {}", e));
                self
            }
            pub fn id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::TypedUuidForRfdId>,
                T::Error: ::std::fmt::Display,
            {
                self.id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for id: {}", e));
                self
            }
            pub fn labels<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.labels = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for labels: {}", e));
                self
            }
            pub fn latest_major_change_at<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<
                    ::std::option::Option<::chrono::DateTime<::chrono::offset::Utc>>,
                >,
                T::Error: ::std::fmt::Display,
            {
                self.latest_major_change_at = value.try_into().map_err(|e| {
                    format!(
                        "error converting supplied value for latest_major_change_at: {}",
                        e
                    )
                });
                self
            }
            pub fn link<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.link = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for link: {}", e));
                self
            }
            pub fn rfd_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<i32>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_number: {}", e));
                self
            }
            pub fn sha<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FileSha>>,
                T::Error: ::std::fmt::Display,
            {
                self.sha = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for sha: {}", e));
                self
            }
            pub fn state<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.state = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for state: {}", e));
                self
            }
            pub fn title<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.title = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for title: {}", e));
                self
            }
            pub fn visibility<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<super::Visibility>,
                T::Error: ::std::fmt::Display,
            {
                self.visibility = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for visibility: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<RfdWithoutContent> for super::RfdWithoutContent {
            type Error = super::error::ConversionError;
            fn try_from(
                value: RfdWithoutContent,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    authors: value.authors?,
                    commit: value.commit?,
                    committed_at: value.committed_at?,
                    discussion: value.discussion?,
                    format: value.format?,
                    id: value.id?,
                    labels: value.labels?,
                    latest_major_change_at: value.latest_major_change_at?,
                    link: value.link?,
                    rfd_number: value.rfd_number?,
                    sha: value.sha?,
                    state: value.state?,
                    title: value.title?,
                    visibility: value.visibility?,
                })
            }
        }

        impl ::std::convert::From<super::RfdWithoutContent> for RfdWithoutContent {
            fn from(value: super::RfdWithoutContent) -> Self {
                Self {
                    authors: Ok(value.authors),
                    commit: Ok(value.commit),
                    committed_at: Ok(value.committed_at),
                    discussion: Ok(value.discussion),
                    format: Ok(value.format),
                    id: Ok(value.id),
                    labels: Ok(value.labels),
                    latest_major_change_at: Ok(value.latest_major_change_at),
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
        pub struct SearchResultHit {
            anchor: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
            content: ::std::result::Result<::std::string::String, ::std::string::String>,
            formatted: ::std::result::Result<
                ::std::option::Option<super::FormattedSearchResultHit>,
                ::std::string::String,
            >,
            hierarchy: ::std::result::Result<
                [::std::option::Option<::std::string::String>; 6usize],
                ::std::string::String,
            >,
            hierarchy_radio: ::std::result::Result<
                [::std::option::Option<::std::string::String>; 6usize],
                ::std::string::String,
            >,
            object_id: ::std::result::Result<::std::string::String, ::std::string::String>,
            rfd_number: ::std::result::Result<u64, ::std::string::String>,
            url: ::std::result::Result<
                ::std::option::Option<::std::string::String>,
                ::std::string::String,
            >,
        }

        impl ::std::default::Default for SearchResultHit {
            fn default() -> Self {
                Self {
                    anchor: Ok(Default::default()),
                    content: Err("no value supplied for content".to_string()),
                    formatted: Ok(Default::default()),
                    hierarchy: Err("no value supplied for hierarchy".to_string()),
                    hierarchy_radio: Err("no value supplied for hierarchy_radio".to_string()),
                    object_id: Err("no value supplied for object_id".to_string()),
                    rfd_number: Err("no value supplied for rfd_number".to_string()),
                    url: Ok(Default::default()),
                }
            }
        }

        impl SearchResultHit {
            pub fn anchor<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.anchor = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for anchor: {}", e));
                self
            }
            pub fn content<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.content = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for content: {}", e));
                self
            }
            pub fn formatted<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<super::FormattedSearchResultHit>>,
                T::Error: ::std::fmt::Display,
            {
                self.formatted = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for formatted: {}", e));
                self
            }
            pub fn hierarchy<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<[::std::option::Option<::std::string::String>; 6usize]>,
                T::Error: ::std::fmt::Display,
            {
                self.hierarchy = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hierarchy: {}", e));
                self
            }
            pub fn hierarchy_radio<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<[::std::option::Option<::std::string::String>; 6usize]>,
                T::Error: ::std::fmt::Display,
            {
                self.hierarchy_radio = value.try_into().map_err(|e| {
                    format!("error converting supplied value for hierarchy_radio: {}", e)
                });
                self
            }
            pub fn object_id<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.object_id = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for object_id: {}", e));
                self
            }
            pub fn rfd_number<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<u64>,
                T::Error: ::std::fmt::Display,
            {
                self.rfd_number = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for rfd_number: {}", e));
                self
            }
            pub fn url<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
                T::Error: ::std::fmt::Display,
            {
                self.url = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for url: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<SearchResultHit> for super::SearchResultHit {
            type Error = super::error::ConversionError;
            fn try_from(
                value: SearchResultHit,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    anchor: value.anchor?,
                    content: value.content?,
                    formatted: value.formatted?,
                    hierarchy: value.hierarchy?,
                    hierarchy_radio: value.hierarchy_radio?,
                    object_id: value.object_id?,
                    rfd_number: value.rfd_number?,
                    url: value.url?,
                })
            }
        }

        impl ::std::convert::From<super::SearchResultHit> for SearchResultHit {
            fn from(value: super::SearchResultHit) -> Self {
                Self {
                    anchor: Ok(value.anchor),
                    content: Ok(value.content),
                    formatted: Ok(value.formatted),
                    hierarchy: Ok(value.hierarchy),
                    hierarchy_radio: Ok(value.hierarchy_radio),
                    object_id: Ok(value.object_id),
                    rfd_number: Ok(value.rfd_number),
                    url: Ok(value.url),
                }
            }
        }

        #[derive(Clone, Debug)]
        pub struct SearchResults {
            hits: ::std::result::Result<
                ::std::vec::Vec<super::SearchResultHit>,
                ::std::string::String,
            >,
            limit: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
            offset: ::std::result::Result<::std::option::Option<u32>, ::std::string::String>,
            query: ::std::result::Result<::std::string::String, ::std::string::String>,
        }

        impl ::std::default::Default for SearchResults {
            fn default() -> Self {
                Self {
                    hits: Err("no value supplied for hits".to_string()),
                    limit: Ok(Default::default()),
                    offset: Ok(Default::default()),
                    query: Err("no value supplied for query".to_string()),
                }
            }
        }

        impl SearchResults {
            pub fn hits<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::vec::Vec<super::SearchResultHit>>,
                T::Error: ::std::fmt::Display,
            {
                self.hits = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for hits: {}", e));
                self
            }
            pub fn limit<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<u32>>,
                T::Error: ::std::fmt::Display,
            {
                self.limit = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for limit: {}", e));
                self
            }
            pub fn offset<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::option::Option<u32>>,
                T::Error: ::std::fmt::Display,
            {
                self.offset = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for offset: {}", e));
                self
            }
            pub fn query<T>(mut self, value: T) -> Self
            where
                T: ::std::convert::TryInto<::std::string::String>,
                T::Error: ::std::fmt::Display,
            {
                self.query = value
                    .try_into()
                    .map_err(|e| format!("error converting supplied value for query: {}", e));
                self
            }
        }

        impl ::std::convert::TryFrom<SearchResults> for super::SearchResults {
            type Error = super::error::ConversionError;
            fn try_from(
                value: SearchResults,
            ) -> ::std::result::Result<Self, super::error::ConversionError> {
                Ok(Self {
                    hits: value.hits?,
                    limit: value.limit?,
                    offset: value.offset?,
                    query: value.query?,
                })
            }
        }

        impl ::std::convert::From<super::SearchResults> for SearchResults {
            fn from(value: super::SearchResults) -> Self {
                Self {
                    hits: Ok(value.hits),
                    limit: Ok(value.limit),
                    offset: Ok(value.offset),
                    query: Ok(value.query),
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
/// Version: 0.12.0
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
}

impl ClientInfo<()> for Client {
    fn api_version() -> &'static str {
        "0.12.0"
    }

    fn baseurl(&self) -> &str {
        self.baseurl.as_str()
    }

    fn client(&self) -> &reqwest::Client {
        &self.client
    }

    fn inner(&self) -> &() {
        &()
    }
}

impl ClientHooks<()> for &Client {}

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

    /// List details for users
    ///
    /// Sends a `GET` request to `/api-user`
    ///
    /// ```ignore
    /// let response = client.list_api_users()
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_api_users(&self) -> builder::ListApiUsers {
        builder::ListApiUsers::new(self)
    }

    /// Create a new user
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

    /// View details for a user
    ///
    /// Sends a `GET` request to `/api-user/{user_id}`
    ///
    /// ```ignore
    /// let response = client.get_api_user()
    ///    .user_id(user_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_api_user(&self) -> builder::GetApiUser {
        builder::GetApiUser::new(self)
    }

    /// Update the permissions assigned to a given user
    ///
    /// Sends a `POST` request to `/api-user/{user_id}`
    ///
    /// ```ignore
    /// let response = client.update_api_user()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_api_user(&self) -> builder::UpdateApiUser {
        builder::UpdateApiUser::new(self)
    }

    /// Set the contact email for a user
    ///
    /// Sends a `PUT` request to `/api-user/{user_id}/contact/email`
    ///
    /// ```ignore
    /// let response = client.set_api_user_contact_email()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn set_api_user_contact_email(&self) -> builder::SetApiUserContactEmail {
        builder::SetApiUserContactEmail::new(self)
    }

    /// Add a user to a group
    ///
    /// Sends a `POST` request to `/api-user/{user_id}/group`
    ///
    /// ```ignore
    /// let response = client.add_api_user_to_group()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn add_api_user_to_group(&self) -> builder::AddApiUserToGroup {
        builder::AddApiUserToGroup::new(self)
    }

    /// Remove a user from a group
    ///
    /// Sends a `DELETE` request to `/api-user/{user_id}/group/{group_id}`
    ///
    /// ```ignore
    /// let response = client.remove_api_user_from_group()
    ///    .user_id(user_id)
    ///    .group_id(group_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn remove_api_user_from_group(&self) -> builder::RemoveApiUserFromGroup {
        builder::RemoveApiUserFromGroup::new(self)
    }

    /// Link an existing login provider to this user
    ///
    /// Sends a `POST` request to `/api-user/{user_id}/link`
    ///
    /// ```ignore
    /// let response = client.link_provider()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn link_provider(&self) -> builder::LinkProvider {
        builder::LinkProvider::new(self)
    }

    /// List api keys for a user
    ///
    /// Sends a `GET` request to `/api-user/{user_id}/token`
    ///
    /// ```ignore
    /// let response = client.list_api_user_tokens()
    ///    .user_id(user_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_api_user_tokens(&self) -> builder::ListApiUserTokens {
        builder::ListApiUserTokens::new(self)
    }

    /// Create a new api key for a user
    ///
    /// Sends a `POST` request to `/api-user/{user_id}/token`
    ///
    /// ```ignore
    /// let response = client.create_api_user_token()
    ///    .user_id(user_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_api_user_token(&self) -> builder::CreateApiUserToken {
        builder::CreateApiUserToken::new(self)
    }

    /// View details of an api key for a user
    ///
    /// Sends a `GET` request to `/api-user/{user_id}/token/{api_key_id}`
    ///
    /// ```ignore
    /// let response = client.get_api_user_token()
    ///    .user_id(user_id)
    ///    .api_key_id(api_key_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_api_user_token(&self) -> builder::GetApiUserToken {
        builder::GetApiUserToken::new(self)
    }

    /// Revoke an api key for a user
    ///
    /// Sends a `DELETE` request to `/api-user/{user_id}/token/{api_key_id}`
    ///
    /// ```ignore
    /// let response = client.delete_api_user_token()
    ///    .user_id(user_id)
    ///    .api_key_id(api_key_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_api_user_token(&self) -> builder::DeleteApiUserToken {
        builder::DeleteApiUserToken::new(self)
    }

    /// Create a new link token for linking this provider to a different api
    /// user
    ///
    /// Sends a `POST` request to `/api-user-provider/{provider_id}/link-token`
    ///
    /// ```ignore
    /// let response = client.create_link_token()
    ///    .provider_id(provider_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_link_token(&self) -> builder::CreateLinkToken {
        builder::CreateLinkToken::new(self)
    }

    /// List all groups
    ///
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

    /// Create a group
    ///
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

    /// Update a group
    ///
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

    /// Delete a group
    ///
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

    /// List all jobs for a RFD
    ///
    /// Sends a `GET` request to `/job`
    ///
    /// ```ignore
    /// let response = client.list_jobs()
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .rfd(rfd)
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_jobs(&self) -> builder::ListJobs {
        builder::ListJobs::new(self)
    }

    /// Exchange a magic link access code for an access token
    ///
    /// Sends a `POST` request to `/login/magic/{channel}/exchange`
    ///
    /// ```ignore
    /// let response = client.magic_link_exchange()
    ///    .channel(channel)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn magic_link_exchange(&self) -> builder::MagicLinkExchange {
        builder::MagicLinkExchange::new(self)
    }

    /// Send a new magic link authentication link
    ///
    /// Sends a `POST` request to `/login/magic/{channel}/send`
    ///
    /// ```ignore
    /// let response = client.magic_link_send()
    ///    .channel(channel)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn magic_link_send(&self) -> builder::MagicLinkSend {
        builder::MagicLinkSend::new(self)
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

    /// Retrieve the metadata about an OAuth provider
    ///
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

    /// Exchange an OAuth device code request for an access token
    ///
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

    /// List Magic Link clients
    ///
    /// Sends a `GET` request to `/magic/client`
    ///
    /// ```ignore
    /// let response = client.list_magic_links()
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_magic_links(&self) -> builder::ListMagicLinks {
        builder::ListMagicLinks::new(self)
    }

    /// Create a new Magic Link Client
    ///
    /// Sends a `POST` request to `/magic/client`
    ///
    /// ```ignore
    /// let response = client.create_magic_link()
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_magic_link(&self) -> builder::CreateMagicLink {
        builder::CreateMagicLink::new(self)
    }

    /// Get a Magic Link Client
    ///
    /// Sends a `GET` request to `/magic/client/{client_id}`
    ///
    /// ```ignore
    /// let response = client.get_magic_link()
    ///    .client_id(client_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_magic_link(&self) -> builder::GetMagicLink {
        builder::GetMagicLink::new(self)
    }

    /// Add a Magic Link client redirect uri
    ///
    /// Sends a `POST` request to `/magic/client/{client_id}/redirect_uri`
    ///
    /// ```ignore
    /// let response = client.create_magic_link_redirect_uri()
    ///    .client_id(client_id)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_magic_link_redirect_uri(&self) -> builder::CreateMagicLinkRedirectUri {
        builder::CreateMagicLinkRedirectUri::new(self)
    }

    /// Delete a Magic Link client redirect uri
    ///
    /// Sends a `DELETE` request to
    /// `/magic/client/{client_id}/redirect_uri/{redirect_uri_id}`
    ///
    /// ```ignore
    /// let response = client.delete_magic_link_redirect_uri()
    ///    .client_id(client_id)
    ///    .redirect_uri_id(redirect_uri_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_magic_link_redirect_uri(&self) -> builder::DeleteMagicLinkRedirectUri {
        builder::DeleteMagicLinkRedirectUri::new(self)
    }

    /// Add a Magic Link client secret
    ///
    /// Sends a `POST` request to `/magic/client/{client_id}/secret`
    ///
    /// ```ignore
    /// let response = client.create_magic_link_secret()
    ///    .client_id(client_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn create_magic_link_secret(&self) -> builder::CreateMagicLinkSecret {
        builder::CreateMagicLinkSecret::new(self)
    }

    /// Delete a Magic Link client secret
    ///
    /// Sends a `DELETE` request to
    /// `/magic/client/{client_id}/secret/{secret_id}`
    ///
    /// ```ignore
    /// let response = client.delete_magic_link_secret()
    ///    .client_id(client_id)
    ///    .secret_id(secret_id)
    ///    .send()
    ///    .await;
    /// ```
    pub fn delete_magic_link_secret(&self) -> builder::DeleteMagicLinkSecret {
        builder::DeleteMagicLinkSecret::new(self)
    }

    /// List all mappers
    ///
    /// Sends a `GET` request to `/mapper`
    ///
    /// Arguments:
    /// - `include_depleted`: Include depleted mappers in the returned results
    /// ```ignore
    /// let response = client.get_mappers()
    ///    .include_depleted(include_depleted)
    ///    .send()
    ///    .await;
    /// ```
    pub fn get_mappers(&self) -> builder::GetMappers {
        builder::GetMappers::new(self)
    }

    /// Create a mapper
    ///
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

    /// Delete a mapper
    ///
    /// Sends a `DELETE` request to `/mapper/{mapper_id}`
    ///
    /// ```ignore
    /// let response = client.delete_mapper()
    ///    .mapper_id(mapper_id)
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
    /// let response = client.list_rfds()
    ///    .send()
    ///    .await;
    /// ```
    pub fn list_rfds(&self) -> builder::ListRfds {
        builder::ListRfds::new(self)
    }

    /// Create a new RFD
    ///
    /// Sends a `POST` request to `/rfd`
    ///
    /// ```ignore
    /// let response = client.reserve_rfd()
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn reserve_rfd(&self) -> builder::ReserveRfd {
        builder::ReserveRfd::new(self)
    }

    /// Get the latest representation of an RFD's metadata
    ///
    /// Sends a `GET` request to `/rfd/{number}`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// ```ignore
    /// let response = client.view_rfd_meta()
    ///    .number(number)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_meta(&self) -> builder::ViewRfdMeta {
        builder::ViewRfdMeta::new(self)
    }

    /// Get the an attribute of the latest revision of a RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}/attr/{attr}`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `attr`: An attribute that can be defined in an RFD document
    /// ```ignore
    /// let response = client.view_rfd_attr()
    ///    .number(number)
    ///    .attr(attr)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_attr(&self) -> builder::ViewRfdAttr {
        builder::ViewRfdAttr::new(self)
    }

    /// Set an attribute of a RFD
    ///
    /// Sends a `POST` request to `/rfd/{number}/attr/{attr}`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `attr`: An attribute that can be defined in an RFD document
    /// - `body`
    /// ```ignore
    /// let response = client.set_rfd_attr()
    ///    .number(number)
    ///    .attr(attr)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn set_rfd_attr(&self) -> builder::SetRfdAttr {
        builder::SetRfdAttr::new(self)
    }

    /// Replace the contents of a RFD
    ///
    /// Sends a `POST` request to `/rfd/{number}/content`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `body`
    /// ```ignore
    /// let response = client.set_rfd_content()
    ///    .number(number)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn set_rfd_content(&self) -> builder::SetRfdContent {
        builder::SetRfdContent::new(self)
    }

    /// Get the comments related to the latest revision of a RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}/discussion`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// ```ignore
    /// let response = client.view_rfd_discussion()
    ///    .number(number)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_discussion(&self) -> builder::ViewRfdDiscussion {
        builder::ViewRfdDiscussion::new(self)
    }

    /// Get the PDF locations of the latest revision of a RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}/pdf`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// ```ignore
    /// let response = client.view_rfd_pdf()
    ///    .number(number)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_pdf(&self) -> builder::ViewRfdPdf {
        builder::ViewRfdPdf::new(self)
    }

    /// Get the raw contents of the latest revision of a RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}/raw`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// ```ignore
    /// let response = client.view_rfd()
    ///    .number(number)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd(&self) -> builder::ViewRfd {
        builder::ViewRfd::new(self)
    }

    /// Replace the full document of a RFD
    ///
    /// Sends a `POST` request to `/rfd/{number}/raw`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `body`
    /// ```ignore
    /// let response = client.set_rfd_document()
    ///    .number(number)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn set_rfd_document(&self) -> builder::SetRfdDocument {
        builder::SetRfdDocument::new(self)
    }

    /// Get an RFD revision's metadata
    ///
    /// Sends a `GET` request to `/rfd/{number}/revision/{revision}`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `revision`: The revision id of the RFD
    /// ```ignore
    /// let response = client.view_rfd_revision_meta()
    ///    .number(number)
    ///    .revision(revision)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_revision_meta(&self) -> builder::ViewRfdRevisionMeta {
        builder::ViewRfdRevisionMeta::new(self)
    }

    /// Get the an attribute of a revision of a RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}/revision/{revision}/attr/{attr}`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `revision`: The revision id of the RFD
    /// - `attr`: An attribute that can be defined in an RFD document
    /// ```ignore
    /// let response = client.view_rfd_revision_attr()
    ///    .number(number)
    ///    .revision(revision)
    ///    .attr(attr)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_revision_attr(&self) -> builder::ViewRfdRevisionAttr {
        builder::ViewRfdRevisionAttr::new(self)
    }

    /// Get the comments related to a revision of a RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}/revision/{revision}/discussion`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `revision`: The revision id of the RFD
    /// ```ignore
    /// let response = client.view_rfd_revision_discussion()
    ///    .number(number)
    ///    .revision(revision)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_revision_discussion(&self) -> builder::ViewRfdRevisionDiscussion {
        builder::ViewRfdRevisionDiscussion::new(self)
    }

    /// Get the PDF locations of a revision of a RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}/revision/{revision}/pdf`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `revision`: The revision id of the RFD
    /// ```ignore
    /// let response = client.view_rfd_revision_pdf()
    ///    .number(number)
    ///    .revision(revision)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_revision_pdf(&self) -> builder::ViewRfdRevisionPdf {
        builder::ViewRfdRevisionPdf::new(self)
    }

    /// Get the raw contents of a revision of a RFD
    ///
    /// Sends a `GET` request to `/rfd/{number}/revision/{revision}/raw`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `revision`: The revision id of the RFD
    /// ```ignore
    /// let response = client.view_rfd_revision()
    ///    .number(number)
    ///    .revision(revision)
    ///    .send()
    ///    .await;
    /// ```
    pub fn view_rfd_revision(&self) -> builder::ViewRfdRevision {
        builder::ViewRfdRevision::new(self)
    }

    /// Open a RFD for discussion
    ///
    /// Sends a `POST` request to `/rfd/{number}/state/discuss`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// ```ignore
    /// let response = client.discuss_rfd()
    ///    .number(number)
    ///    .send()
    ///    .await;
    /// ```
    pub fn discuss_rfd(&self) -> builder::DiscussRfd {
        builder::DiscussRfd::new(self)
    }

    /// Publish a RFD
    ///
    /// Sends a `POST` request to `/rfd/{number}/state/publish`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// ```ignore
    /// let response = client.publish_rfd()
    ///    .number(number)
    ///    .send()
    ///    .await;
    /// ```
    pub fn publish_rfd(&self) -> builder::PublishRfd {
        builder::PublishRfd::new(self)
    }

    /// Modify the visibility of a RFD
    ///
    /// Sends a `POST` request to `/rfd/{number}/visibility`
    ///
    /// Arguments:
    /// - `number`: The RFD number (examples: 1 or 123)
    /// - `body`
    /// ```ignore
    /// let response = client.update_rfd_visibility()
    ///    .number(number)
    ///    .body(body)
    ///    .send()
    ///    .await;
    /// ```
    pub fn update_rfd_visibility(&self) -> builder::UpdateRfdVisibility {
        builder::UpdateRfdVisibility::new(self)
    }

    /// Search the RFD index and get a list of results
    ///
    /// Sends a `GET` request to `/rfd-search`
    ///
    /// ```ignore
    /// let response = client.search_rfds()
    ///    .attributes_to_crop(attributes_to_crop)
    ///    .highlight_post_tag(highlight_post_tag)
    ///    .highlight_pre_tag(highlight_pre_tag)
    ///    .limit(limit)
    ///    .offset(offset)
    ///    .q(q)
    ///    .send()
    ///    .await;
    /// ```
    pub fn search_rfds(&self) -> builder::SearchRfds {
        builder::SearchRfds::new(self)
    }

    /// View details for the calling user
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

/// Internal endpoints
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

/// Types for composing operation parameters.
#[allow(clippy::all)]
pub mod builder {
    use super::types;
    #[allow(unused_imports)]
    use super::{
        encode_path, ByteStream, ClientHooks, ClientInfo, Error, OperationInfo, RequestBuilderExt,
        ResponseValue,
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
            Self { client: client }
        }

        /// Sends a `GET` request to `/.well-known/jwks.json`
        pub async fn send(self) -> Result<ResponseValue<types::Jwks>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/.well-known/jwks.json", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "jwks_json",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
            Self { client: client }
        }

        /// Sends a `GET` request to `/.well-known/openid-configuration`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::OpenIdConfiguration>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/.well-known/openid-configuration", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "openid_configuration",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::list_api_users`]
    ///
    /// [`Client::list_api_users`]: super::Client::list_api_users
    #[derive(Debug, Clone)]
    pub struct ListApiUsers<'a> {
        client: &'a super::Client,
    }

    impl<'a> ListApiUsers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/api-user`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<::std::vec::Vec<types::GetUserResponseForRfdPermission>>,
            Error<types::Error>,
        > {
            let Self { client } = self;
            let url = format!("{}/api-user", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_api_users",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        body: Result<types::builder::ApiUserUpdateParamsForRfdPermission, String>,
    }

    impl<'a> CreateApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserUpdateParamsForRfdPermission>,
            <V as std::convert::TryInto<types::ApiUserUpdateParamsForRfdPermission>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserUpdateParamsForRfdPermission` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiUserUpdateParamsForRfdPermission,
            ) -> types::builder::ApiUserUpdateParamsForRfdPermission,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserForRfdPermission>, Error<types::Error>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| {
                    types::ApiUserUpdateParamsForRfdPermission::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/api-user", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_api_user",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
    }

    impl<'a> GetApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        /// Sends a `GET` request to `/api-user/{user_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetUserResponseForRfdPermission>, Error<types::Error>>
        {
            let Self { client, user_id } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_api_user",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::ApiUserUpdateParamsForRfdPermission, String>,
    }

    impl<'a> UpdateApiUser<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserUpdateParamsForRfdPermission>,
            <V as std::convert::TryInto<types::ApiUserUpdateParamsForRfdPermission>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserUpdateParamsForRfdPermission` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiUserUpdateParamsForRfdPermission,
            ) -> types::builder::ApiUserUpdateParamsForRfdPermission,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{user_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserForRfdPermission>, Error<types::Error>> {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiUserUpdateParamsForRfdPermission::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "update_api_user",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::set_api_user_contact_email`]
    ///
    /// [`Client::set_api_user_contact_email`]: super::Client::set_api_user_contact_email
    #[derive(Debug, Clone)]
    pub struct SetApiUserContactEmail<'a> {
        client: &'a super::Client,
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::ApiUserEmailUpdateParams, String>,
    }

    impl<'a> SetApiUserContactEmail<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserEmailUpdateParams>,
            <V as std::convert::TryInto<types::ApiUserEmailUpdateParams>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserEmailUpdateParams` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiUserEmailUpdateParams,
            ) -> types::builder::ApiUserEmailUpdateParams,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `PUT` request to `/api-user/{user_id}/contact/email`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserContactEmail>, Error<types::Error>> {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiUserEmailUpdateParams::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/contact/email",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "set_api_user_contact_email",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::AddGroupBody, String>,
    }

    impl<'a> AddApiUserToGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddGroupBody>,
            <V as std::convert::TryInto<types::AddGroupBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `AddGroupBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::AddGroupBody) -> types::builder::AddGroupBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{user_id}/group`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserForRfdPermission>, Error<types::Error>> {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::AddGroupBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/group",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "add_api_user_to_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
        group_id: Result<types::TypedUuidForAccessGroupId, String>,
    }

    impl<'a> RemoveApiUserFromGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                group_id: Err("group_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForAccessGroupId>,
        {
            self.group_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForAccessGroupId` for group_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to `/api-user/{user_id}/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserForRfdPermission>, Error<types::Error>> {
            let Self {
                client,
                user_id,
                group_id,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/group/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
                encode_path(&group_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "remove_api_user_from_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::ApiUserProviderLinkPayload, String>,
    }

    impl<'a> LinkProvider<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserProviderLinkPayload>,
            <V as std::convert::TryInto<types::ApiUserProviderLinkPayload>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserProviderLinkPayload` for body failed: {}",
                    s
                )
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

        /// Sends a `POST` request to `/api-user/{user_id}/link`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiUserProviderLinkPayload::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/link",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "link_provider",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
    }

    impl<'a> ListApiUserTokens<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        /// Sends a `GET` request to `/api-user/{user_id}/token`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<::std::vec::Vec<types::ApiKeyResponseForRfdPermission>>,
            Error<types::Error>,
        > {
            let Self { client, user_id } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_api_user_tokens",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
        body: Result<types::builder::ApiKeyCreateParamsForRfdPermission, String>,
    }

    impl<'a> CreateApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiKeyCreateParamsForRfdPermission>,
            <V as std::convert::TryInto<types::ApiKeyCreateParamsForRfdPermission>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiKeyCreateParamsForRfdPermission` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::ApiKeyCreateParamsForRfdPermission,
            ) -> types::builder::ApiKeyCreateParamsForRfdPermission,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/api-user/{user_id}/token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InitialApiKeyResponseForRfdPermission>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                body,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiKeyCreateParamsForRfdPermission::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token",
                client.baseurl,
                encode_path(&user_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_api_user_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
        api_key_id: Result<types::TypedUuidForApiKeyId, String>,
    }

    impl<'a> GetApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                api_key_id: Err("api_key_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn api_key_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForApiKeyId>,
        {
            self.api_key_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForApiKeyId` for api_key_id failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/api-user/{user_id}/token/{api_key_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiKeyResponseForRfdPermission>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                api_key_id,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let api_key_id = api_key_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
                encode_path(&api_key_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_api_user_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        user_id: Result<types::TypedUuidForUserId, String>,
        api_key_id: Result<types::TypedUuidForApiKeyId, String>,
    }

    impl<'a> DeleteApiUserToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                user_id: Err("user_id was not initialized".to_string()),
                api_key_id: Err("api_key_id was not initialized".to_string()),
            }
        }

        pub fn user_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserId>,
        {
            self.user_id = value
                .try_into()
                .map_err(|_| "conversion to `TypedUuidForUserId` for user_id failed".to_string());
            self
        }

        pub fn api_key_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForApiKeyId>,
        {
            self.api_key_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForApiKeyId` for api_key_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to `/api-user/{user_id}/token/{api_key_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiKeyResponseForRfdPermission>, Error<types::Error>>
        {
            let Self {
                client,
                user_id,
                api_key_id,
            } = self;
            let user_id = user_id.map_err(Error::InvalidRequest)?;
            let api_key_id = api_key_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user/{}/token/{}",
                client.baseurl,
                encode_path(&user_id.to_string()),
                encode_path(&api_key_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_api_user_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        provider_id: Result<types::TypedUuidForUserProviderId, String>,
        body: Result<types::builder::ApiUserLinkRequestPayload, String>,
    }

    impl<'a> CreateLinkToken<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                provider_id: Err("provider_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn provider_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForUserProviderId>,
        {
            self.provider_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForUserProviderId` for provider_id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ApiUserLinkRequestPayload>,
            <V as std::convert::TryInto<types::ApiUserLinkRequestPayload>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `ApiUserLinkRequestPayload` for body failed: {}",
                    s
                )
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
        /// `/api-user-provider/{provider_id}/link-token`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ApiUserLinkRequestResponse>, Error<types::Error>> {
            let Self {
                client,
                provider_id,
                body,
            } = self;
            let provider_id = provider_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::ApiUserLinkRequestPayload::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/api-user-provider/{}/link-token",
                client.baseurl,
                encode_path(&provider_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_link_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::GitHubCommitPayload>,
            <V as std::convert::TryInto<types::GitHubCommitPayload>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `GitHubCommitPayload` for body failed: {}", s));
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
                .and_then(|v| types::GitHubCommitPayload::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/github", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "github_webhook",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
            Self { client: client }
        }

        /// Sends a `GET` request to `/group`
        pub async fn send(
            self,
        ) -> Result<
            ResponseValue<::std::vec::Vec<types::AccessGroupForRfdPermission>>,
            Error<types::Error>,
        > {
            let Self { client } = self;
            let url = format!("{}/group", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_groups",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        body: Result<types::builder::AccessGroupUpdateParamsForRfdPermission, String>,
    }

    impl<'a> CreateGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessGroupUpdateParamsForRfdPermission>,
            <V as std::convert::TryInto<types::AccessGroupUpdateParamsForRfdPermission>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AccessGroupUpdateParamsForRfdPermission` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AccessGroupUpdateParamsForRfdPermission,
            )
                -> types::builder::AccessGroupUpdateParamsForRfdPermission,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/group`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForRfdPermission>, Error<types::Error>>
        {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| {
                    types::AccessGroupUpdateParamsForRfdPermission::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/group", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::update_group`]
    ///
    /// [`Client::update_group`]: super::Client::update_group
    #[derive(Debug, Clone)]
    pub struct UpdateGroup<'a> {
        client: &'a super::Client,
        group_id: Result<types::TypedUuidForAccessGroupId, String>,
        body: Result<types::builder::AccessGroupUpdateParamsForRfdPermission, String>,
    }

    impl<'a> UpdateGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                group_id: Err("group_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForAccessGroupId>,
        {
            self.group_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForAccessGroupId` for group_id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AccessGroupUpdateParamsForRfdPermission>,
            <V as std::convert::TryInto<types::AccessGroupUpdateParamsForRfdPermission>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AccessGroupUpdateParamsForRfdPermission` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AccessGroupUpdateParamsForRfdPermission,
            )
                -> types::builder::AccessGroupUpdateParamsForRfdPermission,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `PUT` request to `/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForRfdPermission>, Error<types::Error>>
        {
            let Self {
                client,
                group_id,
                body,
            } = self;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::AccessGroupUpdateParamsForRfdPermission::try_from(v)
                        .map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/group/{}",
                client.baseurl,
                encode_path(&group_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .put(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "update_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        group_id: Result<types::TypedUuidForAccessGroupId, String>,
    }

    impl<'a> DeleteGroup<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                group_id: Err("group_id was not initialized".to_string()),
            }
        }

        pub fn group_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForAccessGroupId>,
        {
            self.group_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForAccessGroupId` for group_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to `/group/{group_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::AccessGroupForRfdPermission>, Error<types::Error>>
        {
            let Self { client, group_id } = self;
            let group_id = group_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/group/{}",
                client.baseurl,
                encode_path(&group_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_group",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::list_jobs`]
    ///
    /// [`Client::list_jobs`]: super::Client::list_jobs
    #[derive(Debug, Clone)]
    pub struct ListJobs<'a> {
        client: &'a super::Client,
        limit: Result<Option<i64>, String>,
        offset: Result<Option<i64>, String>,
        rfd: Result<::std::string::String, String>,
    }

    impl<'a> ListJobs<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                limit: Ok(None),
                offset: Ok(None),
                rfd: Err("rfd was not initialized".to_string()),
            }
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.limit = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for limit failed".to_string());
            self
        }

        pub fn offset<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<i64>,
        {
            self.offset = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `i64` for offset failed".to_string());
            self
        }

        pub fn rfd<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.rfd = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for rfd failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/job`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Job>>, Error<types::Error>> {
            let Self {
                client,
                limit,
                offset,
                rfd,
            } = self;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let offset = offset.map_err(Error::InvalidRequest)?;
            let rfd = rfd.map_err(Error::InvalidRequest)?;
            let url = format!("{}/job", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new("limit", &limit))
                .query(&progenitor_client::QueryParam::new("offset", &offset))
                .query(&progenitor_client::QueryParam::new("rfd", &rfd))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_jobs",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::magic_link_exchange`]
    ///
    /// [`Client::magic_link_exchange`]: super::Client::magic_link_exchange
    #[derive(Debug, Clone)]
    pub struct MagicLinkExchange<'a> {
        client: &'a super::Client,
        channel: Result<::std::string::String, String>,
        body: Result<types::builder::MagicLinkExchangeRequest, String>,
    }

    impl<'a> MagicLinkExchange<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                channel: Err("channel was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn channel<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.channel = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for channel failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::MagicLinkExchangeRequest>,
            <V as std::convert::TryInto<types::MagicLinkExchangeRequest>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `MagicLinkExchangeRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::MagicLinkExchangeRequest,
            ) -> types::builder::MagicLinkExchangeRequest,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/magic/{channel}/exchange`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkExchangeResponse>, Error<types::Error>> {
            let Self {
                client,
                channel,
                body,
            } = self;
            let channel = channel.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::MagicLinkExchangeRequest::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/magic/{}/exchange",
                client.baseurl,
                encode_path(&channel.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "magic_link_exchange",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::magic_link_send`]
    ///
    /// [`Client::magic_link_send`]: super::Client::magic_link_send
    #[derive(Debug, Clone)]
    pub struct MagicLinkSend<'a> {
        client: &'a super::Client,
        channel: Result<::std::string::String, String>,
        body: Result<types::builder::MagicLinkSendRequest, String>,
    }

    impl<'a> MagicLinkSend<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                channel: Err("channel was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn channel<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.channel = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for channel failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::MagicLinkSendRequest>,
            <V as std::convert::TryInto<types::MagicLinkSendRequest>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `MagicLinkSendRequest` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::MagicLinkSendRequest,
            ) -> types::builder::MagicLinkSendRequest,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/login/magic/{channel}/send`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkSendResponse>, Error<types::Error>> {
            let Self {
                client,
                channel,
                body,
            } = self;
            let channel = channel.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::MagicLinkSendRequest::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/magic/{}/send",
                client.baseurl,
                encode_path(&channel.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "magic_link_send",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
        redirect_uri: Result<::std::string::String, String>,
        response_type: Result<::std::string::String, String>,
        scope: Result<Option<::std::string::String>, String>,
        state: Result<::std::string::String, String>,
    }

    impl<'a> AuthzCodeRedirect<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
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
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        pub fn redirect_uri<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.redirect_uri = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for redirect_uri failed".to_string()
            });
            self
        }

        pub fn response_type<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.response_type = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for response_type failed".to_string()
            });
            self
        }

        pub fn scope<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.scope = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for scope failed".to_string()
            });
            self
        }

        pub fn state<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.state = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for state failed".to_string()
            });
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .query(&progenitor_client::QueryParam::new("client_id", &client_id))
                .query(&progenitor_client::QueryParam::new(
                    "redirect_uri",
                    &redirect_uri,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "response_type",
                    &response_type,
                ))
                .query(&progenitor_client::QueryParam::new("scope", &scope))
                .query(&progenitor_client::QueryParam::new("state", &state))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "authz_code_redirect",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        code: Result<Option<::std::string::String>, String>,
        error: Result<Option<::std::string::String>, String>,
        state: Result<Option<::std::string::String>, String>,
    }

    impl<'a> AuthzCodeCallback<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
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
            V: std::convert::TryInto<::std::string::String>,
        {
            self.code = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for code failed".to_string()
            });
            self
        }

        pub fn error<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.error = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for error failed".to_string()
            });
            self
        }

        pub fn state<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.state = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for state failed".to_string()
            });
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .query(&progenitor_client::QueryParam::new("code", &code))
                .query(&progenitor_client::QueryParam::new("error", &error))
                .query(&progenitor_client::QueryParam::new("state", &state))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "authz_code_callback",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
                client: client,
                provider: Err("provider was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
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
            <V as std::convert::TryInto<types::OAuthAuthzCodeExchangeBody>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `OAuthAuthzCodeExchangeBody` for body failed: {}",
                    s
                )
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
                .and_then(|v| {
                    types::OAuthAuthzCodeExchangeBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/code/token",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "authz_code_exchange",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
                client: client,
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_device_provider",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
                client: client,
                provider: Err("provider was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
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
            <V as std::convert::TryInto<types::AccessTokenExchangeRequest>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AccessTokenExchangeRequest` for body failed: {}",
                    s
                )
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
                .and_then(|v| {
                    types::AccessTokenExchangeRequest::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/login/oauth/{}/device/exchange",
                client.baseurl,
                encode_path(&provider.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .form_urlencoded(&body)?
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "exchange_device_token",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
            let response = result?;
            match response.status().as_u16() {
                200..=299 => Ok(ResponseValue::stream(response)),
                _ => Err(Error::ErrorResponse(ResponseValue::stream(response))),
            }
        }
    }

    /// Builder for [`Client::list_magic_links`]
    ///
    /// [`Client::list_magic_links`]: super::Client::list_magic_links
    #[derive(Debug, Clone)]
    pub struct ListMagicLinks<'a> {
        client: &'a super::Client,
    }

    impl<'a> ListMagicLinks<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/magic/client`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::MagicLink>>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/magic/client", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_magic_links",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::create_magic_link`]
    ///
    /// [`Client::create_magic_link`]: super::Client::create_magic_link
    #[derive(Debug, Clone)]
    pub struct CreateMagicLink<'a> {
        client: &'a super::Client,
    }

    impl<'a> CreateMagicLink<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `POST` request to `/magic/client`
        pub async fn send(self) -> Result<ResponseValue<types::MagicLink>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/magic/client", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_magic_link",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::get_magic_link`]
    ///
    /// [`Client::get_magic_link`]: super::Client::get_magic_link
    #[derive(Debug, Clone)]
    pub struct GetMagicLink<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
    }

    impl<'a> GetMagicLink<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/magic/client/{client_id}`
        pub async fn send(self) -> Result<ResponseValue<types::MagicLink>, Error<types::Error>> {
            let Self { client, client_id } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_magic_link",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::create_magic_link_redirect_uri`]
    ///
    /// [`Client::create_magic_link_redirect_uri`]: super::Client::create_magic_link_redirect_uri
    #[derive(Debug, Clone)]
    pub struct CreateMagicLinkRedirectUri<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
        body: Result<types::builder::AddMagicLinkRedirectBody, String>,
    }

    impl<'a> CreateMagicLinkRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddMagicLinkRedirectBody>,
            <V as std::convert::TryInto<types::AddMagicLinkRedirectBody>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AddMagicLinkRedirectBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::AddMagicLinkRedirectBody,
            ) -> types::builder::AddMagicLinkRedirectBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/magic/client/{client_id}/redirect_uri`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkRedirectUri>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                body,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| {
                    types::AddMagicLinkRedirectBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}/redirect_uri",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_magic_link_redirect_uri",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::delete_magic_link_redirect_uri`]
    ///
    /// [`Client::delete_magic_link_redirect_uri`]: super::Client::delete_magic_link_redirect_uri
    #[derive(Debug, Clone)]
    pub struct DeleteMagicLinkRedirectUri<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
        redirect_uri_id: Result<types::TypedUuidForMagicLinkRedirectUriId, String>,
    }

    impl<'a> DeleteMagicLinkRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                redirect_uri_id: Err("redirect_uri_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        pub fn redirect_uri_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkRedirectUriId>,
        {
            self.redirect_uri_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkRedirectUriId` for redirect_uri_id failed"
                    .to_string()
            });
            self
        }

        /// Sends a `DELETE` request to
        /// `/magic/client/{client_id}/redirect_uri/{redirect_uri_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkRedirectUri>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                redirect_uri_id,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let redirect_uri_id = redirect_uri_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}/redirect_uri/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
                encode_path(&redirect_uri_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_magic_link_redirect_uri",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::create_magic_link_secret`]
    ///
    /// [`Client::create_magic_link_secret`]: super::Client::create_magic_link_secret
    #[derive(Debug, Clone)]
    pub struct CreateMagicLinkSecret<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
    }

    impl<'a> CreateMagicLinkSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        /// Sends a `POST` request to `/magic/client/{client_id}/secret`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::InitialMagicLinkSecretResponse>, Error<types::Error>>
        {
            let Self { client, client_id } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}/secret",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_magic_link_secret",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::delete_magic_link_secret`]
    ///
    /// [`Client::delete_magic_link_secret`]: super::Client::delete_magic_link_secret
    #[derive(Debug, Clone)]
    pub struct DeleteMagicLinkSecret<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForMagicLinkId, String>,
        secret_id: Result<types::TypedUuidForMagicLinkSecretId, String>,
    }

    impl<'a> DeleteMagicLinkSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                secret_id: Err("secret_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkId` for client_id failed".to_string()
            });
            self
        }

        pub fn secret_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMagicLinkSecretId>,
        {
            self.secret_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMagicLinkSecretId` for secret_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to
        /// `/magic/client/{client_id}/secret/{secret_id}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::MagicLinkSecret>, Error<types::Error>> {
            let Self {
                client,
                client_id,
                secret_id,
            } = self;
            let client_id = client_id.map_err(Error::InvalidRequest)?;
            let secret_id = secret_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/magic/client/{}/secret/{}",
                client.baseurl,
                encode_path(&client_id.to_string()),
                encode_path(&secret_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_magic_link_secret",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::get_mappers`]
    ///
    /// [`Client::get_mappers`]: super::Client::get_mappers
    #[derive(Debug, Clone)]
    pub struct GetMappers<'a> {
        client: &'a super::Client,
        include_depleted: Result<Option<bool>, String>,
    }

    impl<'a> GetMappers<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                include_depleted: Ok(None),
            }
        }

        pub fn include_depleted<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<bool>,
        {
            self.include_depleted = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `bool` for include_depleted failed".to_string());
            self
        }

        /// Sends a `GET` request to `/mapper`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::Mapper>>, Error<types::Error>> {
            let Self {
                client,
                include_depleted,
            } = self;
            let include_depleted = include_depleted.map_err(Error::InvalidRequest)?;
            let url = format!("{}/mapper", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new(
                    "include_depleted",
                    &include_depleted,
                ))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_mappers",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::CreateMapper>,
            <V as std::convert::TryInto<types::CreateMapper>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `CreateMapper` for body failed: {}", s));
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
                .and_then(|v| types::CreateMapper::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/mapper", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_mapper",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::delete_mapper`]
    ///
    /// [`Client::delete_mapper`]: super::Client::delete_mapper
    #[derive(Debug, Clone)]
    pub struct DeleteMapper<'a> {
        client: &'a super::Client,
        mapper_id: Result<types::TypedUuidForMapperId, String>,
    }

    impl<'a> DeleteMapper<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                mapper_id: Err("mapper_id was not initialized".to_string()),
            }
        }

        pub fn mapper_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForMapperId>,
        {
            self.mapper_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForMapperId` for mapper_id failed".to_string()
            });
            self
        }

        /// Sends a `DELETE` request to `/mapper/{mapper_id}`
        pub async fn send(self) -> Result<ResponseValue<types::Mapper>, Error<types::Error>> {
            let Self { client, mapper_id } = self;
            let mapper_id = mapper_id.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/mapper/{}",
                client.baseurl,
                encode_path(&mapper_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_mapper",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
            Self { client: client }
        }

        /// Sends a `GET` request to `/oauth/client`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::OAuthClient>>, Error<types::Error>>
        {
            let Self { client } = self;
            let url = format!("{}/oauth/client", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_oauth_clients",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
            Self { client: client }
        }

        /// Sends a `POST` request to `/oauth/client`
        pub async fn send(self) -> Result<ResponseValue<types::OAuthClient>, Error<types::Error>> {
            let Self { client } = self;
            let url = format!("{}/oauth/client", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_oauth_client",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::get_oauth_client`]
    ///
    /// [`Client::get_oauth_client`]: super::Client::get_oauth_client
    #[derive(Debug, Clone)]
    pub struct GetOauthClient<'a> {
        client: &'a super::Client,
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
    }

    impl<'a> GetOauthClient<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_oauth_client",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
        body: Result<types::builder::AddOAuthClientRedirectBody, String>,
    }

    impl<'a> CreateOauthClientRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::AddOAuthClientRedirectBody>,
            <V as std::convert::TryInto<types::AddOAuthClientRedirectBody>>::Error:
                std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `AddOAuthClientRedirectBody` for body failed: {}",
                    s
                )
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
                .and_then(|v| {
                    types::AddOAuthClientRedirectBody::try_from(v).map_err(|e| e.to_string())
                })
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/oauth/client/{}/redirect_uri",
                client.baseurl,
                encode_path(&client_id.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_oauth_client_redirect_uri",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
        redirect_uri_id: Result<types::TypedUuidForOAuthRedirectUriId, String>,
    }

    impl<'a> DeleteOauthClientRedirectUri<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                redirect_uri_id: Err("redirect_uri_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        pub fn redirect_uri_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthRedirectUriId>,
        {
            self.redirect_uri_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthRedirectUriId` for redirect_uri_id failed"
                    .to_string()
            });
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_oauth_client_redirect_uri",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
    }

    impl<'a> CreateOauthClientSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "create_oauth_client_secret",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        client_id: Result<types::TypedUuidForOAuthClientId, String>,
        secret_id: Result<types::TypedUuidForOAuthSecretId, String>,
    }

    impl<'a> DeleteOauthClientSecret<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                client_id: Err("client_id was not initialized".to_string()),
                secret_id: Err("secret_id was not initialized".to_string()),
            }
        }

        pub fn client_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthClientId>,
        {
            self.client_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthClientId` for client_id failed".to_string()
            });
            self
        }

        pub fn secret_id<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForOAuthSecretId>,
        {
            self.secret_id = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForOAuthSecretId` for secret_id failed".to_string()
            });
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
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .delete(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "delete_oauth_client_secret",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::list_rfds`]
    ///
    /// [`Client::list_rfds`]: super::Client::list_rfds
    #[derive(Debug, Clone)]
    pub struct ListRfds<'a> {
        client: &'a super::Client,
    }

    impl<'a> ListRfds<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self { client: client }
        }

        /// Sends a `GET` request to `/rfd`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<::std::vec::Vec<types::RfdWithoutContent>>, Error<types::Error>>
        {
            let Self { client } = self;
            let url = format!("{}/rfd", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "list_rfds",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::reserve_rfd`]
    ///
    /// [`Client::reserve_rfd`]: super::Client::reserve_rfd
    #[derive(Debug, Clone)]
    pub struct ReserveRfd<'a> {
        client: &'a super::Client,
        body: Result<types::builder::ReserveRfdBody, String>,
    }

    impl<'a> ReserveRfd<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::ReserveRfdBody>,
            <V as std::convert::TryInto<types::ReserveRfdBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `ReserveRfdBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::ReserveRfdBody) -> types::builder::ReserveRfdBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/rfd`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::ReserveRfdResponse>, Error<types::Error>> {
            let Self { client, body } = self;
            let body = body
                .and_then(|v| types::ReserveRfdBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!("{}/rfd", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "reserve_rfd",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_meta`]
    ///
    /// [`Client::view_rfd_meta`]: super::Client::view_rfd_meta
    #[derive(Debug, Clone)]
    pub struct ViewRfdMeta<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
    }

    impl<'a> ViewRfdMeta<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/rfd/{number}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::RfdWithoutContent>, Error<types::Error>> {
            let Self { client, number } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_meta",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_attr`]
    ///
    /// [`Client::view_rfd_attr`]: super::Client::view_rfd_attr
    #[derive(Debug, Clone)]
    pub struct ViewRfdAttr<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        attr: Result<types::RfdAttrName, String>,
    }

    impl<'a> ViewRfdAttr<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                attr: Err("attr was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn attr<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RfdAttrName>,
        {
            self.attr = value
                .try_into()
                .map_err(|_| "conversion to `RfdAttrName` for attr failed".to_string());
            self
        }

        /// Sends a `GET` request to `/rfd/{number}/attr/{attr}`
        pub async fn send(self) -> Result<ResponseValue<types::RfdAttr>, Error<types::Error>> {
            let Self {
                client,
                number,
                attr,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let attr = attr.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/attr/{}",
                client.baseurl,
                encode_path(&number.to_string()),
                encode_path(&attr.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_attr",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::set_rfd_attr`]
    ///
    /// [`Client::set_rfd_attr`]: super::Client::set_rfd_attr
    #[derive(Debug, Clone)]
    pub struct SetRfdAttr<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        attr: Result<types::RfdAttrName, String>,
        body: Result<types::builder::RfdAttrValue, String>,
    }

    impl<'a> SetRfdAttr<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                attr: Err("attr was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn attr<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RfdAttrName>,
        {
            self.attr = value
                .try_into()
                .map_err(|_| "conversion to `RfdAttrName` for attr failed".to_string());
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RfdAttrValue>,
            <V as std::convert::TryInto<types::RfdAttrValue>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `RfdAttrValue` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::RfdAttrValue) -> types::builder::RfdAttrValue,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/rfd/{number}/attr/{attr}`
        pub async fn send(self) -> Result<ResponseValue<types::RfdAttr>, Error<types::Error>> {
            let Self {
                client,
                number,
                attr,
                body,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let attr = attr.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::RfdAttrValue::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/attr/{}",
                client.baseurl,
                encode_path(&number.to_string()),
                encode_path(&attr.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "set_rfd_attr",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::set_rfd_content`]
    ///
    /// [`Client::set_rfd_content`]: super::Client::set_rfd_content
    #[derive(Debug, Clone)]
    pub struct SetRfdContent<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        body: Result<types::builder::RfdUpdateContentBody, String>,
    }

    impl<'a> SetRfdContent<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RfdUpdateContentBody>,
            <V as std::convert::TryInto<types::RfdUpdateContentBody>>::Error: std::fmt::Display,
        {
            self.body = value.try_into().map(From::from).map_err(|s| {
                format!(
                    "conversion to `RfdUpdateContentBody` for body failed: {}",
                    s
                )
            });
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(
                types::builder::RfdUpdateContentBody,
            ) -> types::builder::RfdUpdateContentBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/rfd/{number}/content`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self {
                client,
                number,
                body,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::RfdUpdateContentBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/content",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "set_rfd_content",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_discussion`]
    ///
    /// [`Client::view_rfd_discussion`]: super::Client::view_rfd_discussion
    #[derive(Debug, Clone)]
    pub struct ViewRfdDiscussion<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
    }

    impl<'a> ViewRfdDiscussion<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/rfd/{number}/discussion`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self { client, number } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/discussion",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_discussion",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_pdf`]
    ///
    /// [`Client::view_rfd_pdf`]: super::Client::view_rfd_pdf
    #[derive(Debug, Clone)]
    pub struct ViewRfdPdf<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
    }

    impl<'a> ViewRfdPdf<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/rfd/{number}/pdf`
        pub async fn send(self) -> Result<ResponseValue<types::RfdWithPdf>, Error<types::Error>> {
            let Self { client, number } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/pdf",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_pdf",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd`]
    ///
    /// [`Client::view_rfd`]: super::Client::view_rfd
    #[derive(Debug, Clone)]
    pub struct ViewRfd<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
    }

    impl<'a> ViewRfd<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/rfd/{number}/raw`
        pub async fn send(self) -> Result<ResponseValue<types::RfdWithRaw>, Error<types::Error>> {
            let Self { client, number } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/raw",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::set_rfd_document`]
    ///
    /// [`Client::set_rfd_document`]: super::Client::set_rfd_document
    #[derive(Debug, Clone)]
    pub struct SetRfdDocument<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        body: Result<types::builder::RfdUpdateBody, String>,
    }

    impl<'a> SetRfdDocument<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RfdUpdateBody>,
            <V as std::convert::TryInto<types::RfdUpdateBody>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `RfdUpdateBody` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::RfdUpdateBody) -> types::builder::RfdUpdateBody,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/rfd/{number}/raw`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self {
                client,
                number,
                body,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::RfdUpdateBody::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/raw",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "set_rfd_document",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_revision_meta`]
    ///
    /// [`Client::view_rfd_revision_meta`]: super::Client::view_rfd_revision_meta
    #[derive(Debug, Clone)]
    pub struct ViewRfdRevisionMeta<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        revision: Result<types::TypedUuidForRfdRevisionId, String>,
    }

    impl<'a> ViewRfdRevisionMeta<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                revision: Err("revision was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn revision<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForRfdRevisionId>,
        {
            self.revision = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForRfdRevisionId` for revision failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/rfd/{number}/revision/{revision}`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::RfdWithoutContent>, Error<types::Error>> {
            let Self {
                client,
                number,
                revision,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let revision = revision.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/revision/{}",
                client.baseurl,
                encode_path(&number.to_string()),
                encode_path(&revision.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_revision_meta",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_revision_attr`]
    ///
    /// [`Client::view_rfd_revision_attr`]: super::Client::view_rfd_revision_attr
    #[derive(Debug, Clone)]
    pub struct ViewRfdRevisionAttr<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        revision: Result<types::TypedUuidForRfdRevisionId, String>,
        attr: Result<types::RfdAttrName, String>,
    }

    impl<'a> ViewRfdRevisionAttr<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                revision: Err("revision was not initialized".to_string()),
                attr: Err("attr was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn revision<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForRfdRevisionId>,
        {
            self.revision = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForRfdRevisionId` for revision failed".to_string()
            });
            self
        }

        pub fn attr<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RfdAttrName>,
        {
            self.attr = value
                .try_into()
                .map_err(|_| "conversion to `RfdAttrName` for attr failed".to_string());
            self
        }

        /// Sends a `GET` request to
        /// `/rfd/{number}/revision/{revision}/attr/{attr}`
        pub async fn send(self) -> Result<ResponseValue<types::RfdAttr>, Error<types::Error>> {
            let Self {
                client,
                number,
                revision,
                attr,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let revision = revision.map_err(Error::InvalidRequest)?;
            let attr = attr.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/revision/{}/attr/{}",
                client.baseurl,
                encode_path(&number.to_string()),
                encode_path(&revision.to_string()),
                encode_path(&attr.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_revision_attr",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_revision_discussion`]
    ///
    /// [`Client::view_rfd_revision_discussion`]: super::Client::view_rfd_revision_discussion
    #[derive(Debug, Clone)]
    pub struct ViewRfdRevisionDiscussion<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        revision: Result<types::TypedUuidForRfdRevisionId, String>,
    }

    impl<'a> ViewRfdRevisionDiscussion<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                revision: Err("revision was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn revision<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForRfdRevisionId>,
        {
            self.revision = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForRfdRevisionId` for revision failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to
        /// `/rfd/{number}/revision/{revision}/discussion`
        pub async fn send(self) -> Result<ResponseValue<()>, Error<types::Error>> {
            let Self {
                client,
                number,
                revision,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let revision = revision.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/revision/{}/discussion",
                client.baseurl,
                encode_path(&number.to_string()),
                encode_path(&revision.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_revision_discussion",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_revision_pdf`]
    ///
    /// [`Client::view_rfd_revision_pdf`]: super::Client::view_rfd_revision_pdf
    #[derive(Debug, Clone)]
    pub struct ViewRfdRevisionPdf<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        revision: Result<types::TypedUuidForRfdRevisionId, String>,
    }

    impl<'a> ViewRfdRevisionPdf<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                revision: Err("revision was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn revision<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForRfdRevisionId>,
        {
            self.revision = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForRfdRevisionId` for revision failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/rfd/{number}/revision/{revision}/pdf`
        pub async fn send(self) -> Result<ResponseValue<types::RfdWithPdf>, Error<types::Error>> {
            let Self {
                client,
                number,
                revision,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let revision = revision.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/revision/{}/pdf",
                client.baseurl,
                encode_path(&number.to_string()),
                encode_path(&revision.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_revision_pdf",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::view_rfd_revision`]
    ///
    /// [`Client::view_rfd_revision`]: super::Client::view_rfd_revision
    #[derive(Debug, Clone)]
    pub struct ViewRfdRevision<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        revision: Result<types::TypedUuidForRfdRevisionId, String>,
    }

    impl<'a> ViewRfdRevision<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                revision: Err("revision was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn revision<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::TypedUuidForRfdRevisionId>,
        {
            self.revision = value.try_into().map_err(|_| {
                "conversion to `TypedUuidForRfdRevisionId` for revision failed".to_string()
            });
            self
        }

        /// Sends a `GET` request to `/rfd/{number}/revision/{revision}/raw`
        pub async fn send(self) -> Result<ResponseValue<types::RfdWithRaw>, Error<types::Error>> {
            let Self {
                client,
                number,
                revision,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let revision = revision.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/revision/{}/raw",
                client.baseurl,
                encode_path(&number.to_string()),
                encode_path(&revision.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "view_rfd_revision",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::discuss_rfd`]
    ///
    /// [`Client::discuss_rfd`]: super::Client::discuss_rfd
    #[derive(Debug, Clone)]
    pub struct DiscussRfd<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
    }

    impl<'a> DiscussRfd<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        /// Sends a `POST` request to `/rfd/{number}/state/discuss`
        pub async fn send(self) -> Result<ResponseValue<types::RfdAttr>, Error<types::Error>> {
            let Self { client, number } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/state/discuss",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "discuss_rfd",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::publish_rfd`]
    ///
    /// [`Client::publish_rfd`]: super::Client::publish_rfd
    #[derive(Debug, Clone)]
    pub struct PublishRfd<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
    }

    impl<'a> PublishRfd<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        /// Sends a `POST` request to `/rfd/{number}/state/publish`
        pub async fn send(self) -> Result<ResponseValue<types::RfdAttr>, Error<types::Error>> {
            let Self { client, number } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/state/publish",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "publish_rfd",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

    /// Builder for [`Client::update_rfd_visibility`]
    ///
    /// [`Client::update_rfd_visibility`]: super::Client::update_rfd_visibility
    #[derive(Debug, Clone)]
    pub struct UpdateRfdVisibility<'a> {
        client: &'a super::Client,
        number: Result<::std::string::String, String>,
        body: Result<types::builder::RfdVisibility, String>,
    }

    impl<'a> UpdateRfdVisibility<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                number: Err("number was not initialized".to_string()),
                body: Ok(::std::default::Default::default()),
            }
        }

        pub fn number<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.number = value.try_into().map_err(|_| {
                "conversion to `:: std :: string :: String` for number failed".to_string()
            });
            self
        }

        pub fn body<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<types::RfdVisibility>,
            <V as std::convert::TryInto<types::RfdVisibility>>::Error: std::fmt::Display,
        {
            self.body = value
                .try_into()
                .map(From::from)
                .map_err(|s| format!("conversion to `RfdVisibility` for body failed: {}", s));
            self
        }

        pub fn body_map<F>(mut self, f: F) -> Self
        where
            F: std::ops::FnOnce(types::builder::RfdVisibility) -> types::builder::RfdVisibility,
        {
            self.body = self.body.map(f);
            self
        }

        /// Sends a `POST` request to `/rfd/{number}/visibility`
        pub async fn send(self) -> Result<ResponseValue<types::Rfd>, Error<types::Error>> {
            let Self {
                client,
                number,
                body,
            } = self;
            let number = number.map_err(Error::InvalidRequest)?;
            let body = body
                .and_then(|v| types::RfdVisibility::try_from(v).map_err(|e| e.to_string()))
                .map_err(Error::InvalidRequest)?;
            let url = format!(
                "{}/rfd/{}/visibility",
                client.baseurl,
                encode_path(&number.to_string()),
            );
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .post(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .json(&body)
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "update_rfd_visibility",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
        attributes_to_crop: Result<Option<::std::string::String>, String>,
        highlight_post_tag: Result<Option<::std::string::String>, String>,
        highlight_pre_tag: Result<Option<::std::string::String>, String>,
        limit: Result<Option<u32>, String>,
        offset: Result<Option<u32>, String>,
        q: Result<::std::string::String, String>,
    }

    impl<'a> SearchRfds<'a> {
        pub fn new(client: &'a super::Client) -> Self {
            Self {
                client: client,
                attributes_to_crop: Ok(None),
                highlight_post_tag: Ok(None),
                highlight_pre_tag: Ok(None),
                limit: Ok(None),
                offset: Ok(None),
                q: Err("q was not initialized".to_string()),
            }
        }

        pub fn attributes_to_crop<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.attributes_to_crop = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for attributes_to_crop failed"
                    .to_string()
            });
            self
        }

        pub fn highlight_post_tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.highlight_post_tag = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for highlight_post_tag failed"
                    .to_string()
            });
            self
        }

        pub fn highlight_pre_tag<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.highlight_pre_tag = value.try_into().map(Some).map_err(|_| {
                "conversion to `:: std :: string :: String` for highlight_pre_tag failed"
                    .to_string()
            });
            self
        }

        pub fn limit<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<u32>,
        {
            self.limit = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `u32` for limit failed".to_string());
            self
        }

        pub fn offset<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<u32>,
        {
            self.offset = value
                .try_into()
                .map(Some)
                .map_err(|_| "conversion to `u32` for offset failed".to_string());
            self
        }

        pub fn q<V>(mut self, value: V) -> Self
        where
            V: std::convert::TryInto<::std::string::String>,
        {
            self.q = value
                .try_into()
                .map_err(|_| "conversion to `:: std :: string :: String` for q failed".to_string());
            self
        }

        /// Sends a `GET` request to `/rfd-search`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::SearchResults>, Error<types::Error>> {
            let Self {
                client,
                attributes_to_crop,
                highlight_post_tag,
                highlight_pre_tag,
                limit,
                offset,
                q,
            } = self;
            let attributes_to_crop = attributes_to_crop.map_err(Error::InvalidRequest)?;
            let highlight_post_tag = highlight_post_tag.map_err(Error::InvalidRequest)?;
            let highlight_pre_tag = highlight_pre_tag.map_err(Error::InvalidRequest)?;
            let limit = limit.map_err(Error::InvalidRequest)?;
            let offset = offset.map_err(Error::InvalidRequest)?;
            let q = q.map_err(Error::InvalidRequest)?;
            let url = format!("{}/rfd-search", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .query(&progenitor_client::QueryParam::new(
                    "attributes_to_crop",
                    &attributes_to_crop,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "highlight_post_tag",
                    &highlight_post_tag,
                ))
                .query(&progenitor_client::QueryParam::new(
                    "highlight_pre_tag",
                    &highlight_pre_tag,
                ))
                .query(&progenitor_client::QueryParam::new("limit", &limit))
                .query(&progenitor_client::QueryParam::new("offset", &offset))
                .query(&progenitor_client::QueryParam::new("q", &q))
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "search_rfds",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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
            Self { client: client }
        }

        /// Sends a `GET` request to `/self`
        pub async fn send(
            self,
        ) -> Result<ResponseValue<types::GetUserResponseForRfdPermission>, Error<types::Error>>
        {
            let Self { client } = self;
            let url = format!("{}/self", client.baseurl,);
            let mut header_map = ::reqwest::header::HeaderMap::with_capacity(1usize);
            header_map.append(
                ::reqwest::header::HeaderName::from_static("api-version"),
                ::reqwest::header::HeaderValue::from_static(super::Client::api_version()),
            );
            #[allow(unused_mut)]
            let mut request = client
                .client
                .get(url)
                .header(
                    ::reqwest::header::ACCEPT,
                    ::reqwest::header::HeaderValue::from_static("application/json"),
                )
                .headers(header_map)
                .build()?;
            let info = OperationInfo {
                operation_id: "get_self",
            };
            client.pre(&mut request, &info).await?;
            let result = client.exec(request, &info).await;
            client.post(&result, &info).await?;
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

/// Items consumers will typically use such as the Client and
/// extension traits.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
    pub use super::ClientHiddenExt;
}
