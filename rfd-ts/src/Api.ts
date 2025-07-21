/* eslint-disable */

/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, you can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright Oxide Computer Company
 */

import type { FetchParams } from './http-client'
import { HttpClient, toQueryString } from './http-client'

export type { ApiConfig, ApiResult, ErrorBody, ErrorResult } from './http-client'

export type TypedUuidForUserId = string

export type TypedUuidForApiKeyId = string

export type TypedUuidForAccessGroupId = string

export type TypedUuidForMapperId = string

export type TypedUuidForOAuthClientId = string

export type TypedUuidForMagicLinkId = string

export type RfdPermission =
  | 'GetRfdsAssigned'
  | 'GetRfdsAll'
  | 'CreateRfd'
  | 'UpdateRfdsAssigned'
  | 'UpdateRfdsAll'
  | 'ManageRfdsVisibilityAssigned'
  | 'ManageRfdsVisibilityAll'
  | 'GetDiscussionsAssigned'
  | 'GetDiscussionsAll'
  | 'SearchRfds'
  | 'CreateApiUser'
  | 'GetApiUserSelf'
  | 'GetApiUsersAssigned'
  | 'GetApiUsersAll'
  | 'ManageApiUsersAssigned'
  | 'ManageApiUsersAll'
  | 'CreateApiKeySelf'
  | 'CreateApiKeyAssigned'
  | 'CreateApiKeyAll'
  | 'GetApiKeysAssigned'
  | 'GetApiKeysAll'
  | 'ManageApiKeysAssigned'
  | 'ManageApiKeysAll'
  | 'CreateUserApiProviderLinkToken'
  | 'CreateGroup'
  | 'GetGroupsJoined'
  | 'GetGroupsAll'
  | 'ManageGroupsAssigned'
  | 'ManageGroupsAll'
  | 'ManageGroupMembershipsAssigned'
  | 'ManageGroupMembershipsAll'
  | 'CreateMapper'
  | 'GetMappersAll'
  | 'ManageMappersAssigned'
  | 'ManageMappersAll'
  | 'CreateOAuthClient'
  | 'GetOAuthClientsAssigned'
  | 'GetOAuthClientsAll'
  | 'ManageOAuthClientsAssigned'
  | 'ManageOAuthClientsAll'
  | 'CreateMagicLinkClient'
  | 'GetMagicLinkClientsAssigned'
  | 'GetMagicLinkClientsAll'
  | 'ManageMagicLinkClientsAssigned'
  | 'ManageMagicLinkClientsAll'
  | 'CreateAccessToken'
  | { 'GetRfd': number }
  | { 'GetRfds': number[] }
  | { 'UpdateRfd': number }
  | { 'UpdateRfds': number[] }
  | { 'ManageRfdVisibility': number }
  | { 'ManageRfdsVisibility': number[] }
  | { 'GetDiscussion': number }
  | { 'GetDiscussions': number[] }
  | { 'GetApiUser': TypedUuidForUserId }
  | { 'GetApiUsers': TypedUuidForUserId[] }
  | { 'ManageApiUser': TypedUuidForUserId }
  | { 'ManageApiUsers': TypedUuidForUserId[] }
  | { 'CreateApiKey': TypedUuidForUserId }
  | { 'GetApiKey': TypedUuidForApiKeyId }
  | { 'GetApiKeys': TypedUuidForApiKeyId[] }
  | { 'ManageApiKey': TypedUuidForApiKeyId }
  | { 'ManageApiKeys': TypedUuidForApiKeyId[] }
  | { 'GetGroup': TypedUuidForAccessGroupId }
  | { 'ManageGroup': TypedUuidForAccessGroupId }
  | { 'ManageGroups': TypedUuidForAccessGroupId[] }
  | { 'ManageGroupMembership': TypedUuidForAccessGroupId }
  | { 'ManageGroupMemberships': TypedUuidForAccessGroupId[] }
  | { 'ManageMapper': TypedUuidForMapperId }
  | { 'ManageMappers': TypedUuidForMapperId[] }
  | { 'GetOAuthClient': TypedUuidForOAuthClientId }
  | { 'GetOAuthClients': TypedUuidForOAuthClientId[] }
  | { 'ManageOAuthClient': TypedUuidForOAuthClientId }
  | { 'ManageOAuthClients': TypedUuidForOAuthClientId[] }
  | { 'GetMagicLinkClient': TypedUuidForMagicLinkId }
  | { 'GetMagicLinkClients': TypedUuidForMagicLinkId[] }
  | { 'ManageMagicLinkClient': TypedUuidForMagicLinkId }
  | { 'ManageMagicLinkClients': TypedUuidForMagicLinkId[] }
  | { 'Unsupported': Record<string, unknown> }

export type Permissions_for_RfdPermission = RfdPermission[]

export type AccessGroupUpdateParams_for_RfdPermission = { 'name': string; 'permissions': Permissions_for_RfdPermission }

export type AccessGroup_for_RfdPermission = {
  'createdAt': Date
  'deletedAt'?: Date
  'id': TypedUuidForAccessGroupId
  'name': string
  'permissions': Permissions_for_RfdPermission
  'updatedAt': Date
}

export type AccessTokenExchangeRequest = { 'deviceCode': string; 'expiresAt'?: Date; 'grantType': string }

export type AddGroupBody = { 'groupId': TypedUuidForAccessGroupId }

export type AddMagicLinkRedirectBody = { 'redirectUri': string }

export type AddOAuthClientRedirectBody = { 'redirectUri': string }

export type ApiKeyCreateParams_for_RfdPermission = { 'expiresAt': Date; 'permissions'?: Permissions_for_RfdPermission }

export type ApiKeyResponse_for_RfdPermission = {
  'createdAt': Date
  'id': TypedUuidForApiKeyId
  'permissions'?: Permissions_for_RfdPermission
}

export type TypedUuidForUserProviderId = string

export type ApiUserContactEmail = {
  'createdAt': Date
  'deletedAt'?: Date
  'email': string
  'id': TypedUuidForUserProviderId
  'updatedAt': Date
  'userId': TypedUuidForUserId
}

export type ApiUserEmailUpdateParams = { 'email': string }

export type ApiUserLinkRequestPayload = { 'userId': TypedUuidForUserId }

export type SecretString = string

export type ApiUserLinkRequestResponse = { 'token': SecretString }

export type ApiUserProvider = {
  'createdAt': Date
  'deletedAt'?: Date
  'displayNames': string[]
  'emails': string[]
  'id': TypedUuidForUserProviderId
  'provider': string
  'providerId': string
  'updatedAt': Date
  'userId': TypedUuidForUserId
}

export type ApiUserProviderLinkPayload = { 'token': string }

export type ApiUserUpdateParams_for_RfdPermission = {
  'groupIds': TypedUuidForAccessGroupId[]
  'permissions': Permissions_for_RfdPermission
}

export type ApiUser_for_RfdPermission = {
  'createdAt': Date
  'deletedAt'?: Date
  'groups': TypedUuidForAccessGroupId[]
  'id': TypedUuidForUserId
  'permissions': Permissions_for_RfdPermission
  'updatedAt': Date
}

export type CommitSha = string

export type ContentFormat =
  | 'asciidoc'
  | 'markdown'

export type CreateMapper = { 'maxActivations'?: number; 'name': string; 'rule': Record<string, unknown> }

export type FileSha = string

export type FormattedSearchResultHit = {
  'anchor'?: string
  'content'?: string
  'hierarchy': string[]
  'hierarchyRadio': string[]
  'objectId': string
  'rfdNumber': number
  'url'?: string
}

export type GetUserResponse_for_RfdPermission = { 'info': ApiUser_for_RfdPermission; 'providers': ApiUserProvider[] }

export type GitHubCommit = {
  'added': string[]
  'id': string
  'modified': string[]
  'removed': string[]
  'timestamp': Date
}

export type GitHubInstallation = { 'id': number; 'nodeId': string }

export type GitHubRepositoryOwner = { 'login': string }

export type GitHubRepository = {
  'defaultBranch': string
  'id': number
  'name': string
  'nodeId': string
  'owner': GitHubRepositoryOwner
}

export type GitHubSender = { 'id': number; 'login': string; 'nodeId': string; 'type': string }

export type GitHubCommitPayload = {
  'commits': GitHubCommit[]
  'headCommit'?: GitHubCommit
  'installation': GitHubInstallation
  'ref': string
  'repository': GitHubRepository
  'sender': GitHubSender
}

export type InitialApiKeyResponse_for_RfdPermission = {
  'createdAt': Date
  'id': TypedUuidForApiKeyId
  'key': SecretString
  'permissions'?: Permissions_for_RfdPermission
}

export type TypedUuidForMagicLinkSecretId = string

export type InitialMagicLinkSecretResponse = {
  'createdAt': Date
  'id': TypedUuidForMagicLinkSecretId
  'key': SecretString
}

export type TypedUuidForOAuthSecretId = string

export type InitialOAuthClientSecretResponse = {
  'createdAt': Date
  'id': TypedUuidForOAuthSecretId
  'key': SecretString
}

export type TypedUuidForWebhookDeliveryId = string

export type Job = {
  'branch': string
  'committedAt': Date
  'createdAt': Date
  'id': number
  'owner': string
  'processed': boolean
  'repository': string
  'rfd': number
  'sha': CommitSha
  'startedAt'?: Date
  'webhookDeliveryId'?: TypedUuidForWebhookDeliveryId
}

export type Jwk = { 'e': string; 'kid': string; 'kty': string; 'n': string; 'use': string }

export type Jwks = { 'keys': Jwk[] }

export type TypedUuidForMagicLinkRedirectUriId = string

export type MagicLinkRedirectUri = {
  'createdAt': Date
  'deletedAt'?: Date
  'id': TypedUuidForMagicLinkRedirectUriId
  'magicLinkClientId': TypedUuidForMagicLinkId
  'redirectUri': string
}

export type MagicLinkSecret = {
  'createdAt': Date
  'deletedAt'?: Date
  'id': TypedUuidForMagicLinkSecretId
  'magicLinkClientId': TypedUuidForMagicLinkId
  'secretSignature': string
}

export type MagicLink = {
  'createdAt': Date
  'deletedAt'?: Date
  'id': TypedUuidForMagicLinkId
  'redirectUris': MagicLinkRedirectUri[]
  'secrets': MagicLinkSecret[]
}

export type TypedUuidForMagicLinkAttemptId = string

export type MagicLinkExchangeRequest = {
  'attemptId': TypedUuidForMagicLinkAttemptId
  'recipient': string
  'secret': string
}

export type MagicLinkExchangeResponse = { 'accessToken': string; 'expiresIn': number; 'tokenType': string }

export type MagicLinkMedium = 'email'

export type MagicLinkSendRequest = {
  'expiresIn': number
  'medium': MagicLinkMedium
  'recipient': string
  'redirectUri': string
  'scope'?: string
  'secret': string
}

export type MagicLinkSendResponse = { 'attemptId': TypedUuidForMagicLinkAttemptId }

export type Mapper = {
  'activations'?: number
  'createdAt': Date
  'deletedAt'?: Date
  'depletedAt'?: Date
  'id': TypedUuidForMapperId
  'maxActivations'?: number
  'name': string
  'rule': Record<string, unknown>
  'updatedAt': Date
}

export type OAuthAuthzCodeExchangeBody = {
  'clientId'?: TypedUuidForOAuthClientId
  'clientSecret'?: SecretString
  'code': string
  'grantType': string
  'pkceVerifier'?: string
  'redirectUri': string
}

export type OAuthAuthzCodeExchangeResponse = { 'accessToken': string; 'expiresIn': number; 'tokenType': string }

export type TypedUuidForOAuthRedirectUriId = string

export type OAuthClientRedirectUri = {
  'createdAt': Date
  'deletedAt'?: Date
  'id': TypedUuidForOAuthRedirectUriId
  'oauthClientId': TypedUuidForOAuthClientId
  'redirectUri': string
}

export type OAuthClientSecret = {
  'createdAt': Date
  'deletedAt'?: Date
  'id': TypedUuidForOAuthSecretId
  'oauthClientId': TypedUuidForOAuthClientId
  'secretSignature': string
}

export type OAuthClient = {
  'createdAt': Date
  'deletedAt'?: Date
  'id': TypedUuidForOAuthClientId
  'redirectUris': OAuthClientRedirectUri[]
  'secrets': OAuthClientSecret[]
}

export type OAuthProviderName =
  | 'github'
  | 'google'

export type OAuthProviderInfo = {
  'authUrlEndpoint': string
  'clientId': string
  'deviceCodeEndpoint': string
  'provider': OAuthProviderName
  'scopes': string[]
  'tokenEndpoint': string
}

export type OpenIdConfiguration = { 'jwksUri': string }

export type PdfSource =
  | 'github'
  | 'google'

export type ReserveRfdBody = {
  /** Optional contents of the RFD */
  'content'?: string
  /** Title of the RFD */
  'title': string
}

export type ReserveRfdResponse = { 'number': number }

export type TypedUuidForRfdRevisionId = string

export type TypedUuidForRfdId = string

export type RfdRevision = {
  'authors'?: string
  'commit': CommitSha
  'committedAt': Date
  'content': string
  'contentFormat': ContentFormat
  'createdAt': Date
  'deletedAt'?: Date
  'discussion'?: string
  'id': TypedUuidForRfdRevisionId
  'labels'?: string
  'rfdId': TypedUuidForRfdId
  'sha': FileSha
  'state'?: string
  'title': string
  'updatedAt': Date
}

export type Visibility =
  | 'public'
  | 'private'

export type Rfd = {
  'content'?: RfdRevision
  'createdAt': Date
  'deletedAt'?: Date
  'id': TypedUuidForRfdId
  'link'?: string
  'rfdNumber': number
  'updatedAt': Date
  'visibility': Visibility
}

export type RfdState =
  | 'abandoned'
  | 'committed'
  | 'discussion'
  | 'ideation'
  | 'prediscussion'
  | 'published'

export type RfdAttr =
  | { 'discussion': string }
  | { 'labels': string }
  | { 'state': RfdState }

export type RfdAttrValue = {
  /** Optional Git commit message to send with this update (recommended) */
  'message'?: string
  /** Full value to set this attribute to in the existing RFD contents */
  'value': string
}

export type TypedUuidForRfdPdfId = string

export type RfdPdf = {
  'createdAt': Date
  'deletedAt'?: Date
  'externalId': string
  'id': TypedUuidForRfdPdfId
  'link': string
  'rfdId': TypedUuidForRfdId
  'rfdRevisionId': TypedUuidForRfdRevisionId
  'source': PdfSource
  'updatedAt': Date
}

export type RfdUpdateBody = {
  /** Full Asciidoc document to store for this RFD */
  'document': string
  /** Optional Git commit message to send with this update (recommended) */
  'message'?: string
}

export type RfdUpdateContentBody = {
  /** Asciidoc content to store for this RFD */
  'content': string
  /** Optional Git commit message to send with this update (recommended) */
  'message'?: string
}

export type RfdVisibility = { 'visibility': Visibility }

export type RfdWithPdf = {
  'authors'?: string
  'commit'?: CommitSha
  'committedAt'?: Date
  'content': RfdPdf[]
  'discussion'?: string
  'format'?: ContentFormat
  'id': TypedUuidForRfdId
  'labels'?: string
  'link'?: string
  'rfdNumber': number
  'sha'?: FileSha
  'state'?: string
  'title'?: string
  'visibility': Visibility
}

export type RfdWithRaw = {
  'authors'?: string
  'commit'?: CommitSha
  'committedAt'?: Date
  'content'?: string
  'discussion'?: string
  'format'?: ContentFormat
  'id': TypedUuidForRfdId
  'labels'?: string
  'link'?: string
  'rfdNumber': number
  'sha'?: FileSha
  'state'?: string
  'title'?: string
  'visibility': Visibility
}

export type RfdWithoutContent = {
  'authors'?: string
  'commit'?: CommitSha
  'committedAt'?: Date
  'discussion'?: string
  'format'?: ContentFormat
  'id': TypedUuidForRfdId
  'labels'?: string
  'link'?: string
  'rfdNumber': number
  'sha'?: FileSha
  'state'?: string
  'title'?: string
  'visibility': Visibility
}

export type SearchResultHit = {
  'anchor'?: string
  'content': string
  'formatted'?: FormattedSearchResultHit
  'hierarchy': string[]
  'hierarchyRadio': string[]
  'objectId': string
  'rfdNumber': number
  'url'?: string
}

export type SearchResults = { 'hits': SearchResultHit[]; 'limit'?: number; 'offset'?: number; 'query': string }

export type RfdAttrName =
  | 'discussion'
  | 'labels'
  | 'state'

export interface GetApiUserPathParams {
  userId: TypedUuidForUserId
}

export interface UpdateApiUserPathParams {
  userId: TypedUuidForUserId
}

export interface SetApiUserContactEmailPathParams {
  userId: TypedUuidForUserId
}

export interface AddApiUserToGroupPathParams {
  userId: TypedUuidForUserId
}

export interface RemoveApiUserFromGroupPathParams {
  groupId: TypedUuidForAccessGroupId
  userId: TypedUuidForUserId
}

export interface LinkProviderPathParams {
  userId: TypedUuidForUserId
}

export interface ListApiUserTokensPathParams {
  userId: TypedUuidForUserId
}

export interface CreateApiUserTokenPathParams {
  userId: TypedUuidForUserId
}

export interface GetApiUserTokenPathParams {
  apiKeyId: TypedUuidForApiKeyId
  userId: TypedUuidForUserId
}

export interface DeleteApiUserTokenPathParams {
  apiKeyId: TypedUuidForApiKeyId
  userId: TypedUuidForUserId
}

export interface CreateLinkTokenPathParams {
  providerId: TypedUuidForUserProviderId
}

export interface UpdateGroupPathParams {
  groupId: TypedUuidForAccessGroupId
}

export interface DeleteGroupPathParams {
  groupId: TypedUuidForAccessGroupId
}

export interface ListJobsQueryParams {
  limit?: number
  offset?: number
  rfd: string
}

export interface MagicLinkExchangePathParams {
  channel: string
}

export interface MagicLinkSendPathParams {
  channel: string
}

export interface AuthzCodeRedirectPathParams {
  provider: OAuthProviderName
}

export interface AuthzCodeRedirectQueryParams {
  clientId: TypedUuidForOAuthClientId
  redirectUri: string
  responseType: string
  scope?: string
  state: string
}

export interface AuthzCodeCallbackPathParams {
  provider: OAuthProviderName
}

export interface AuthzCodeCallbackQueryParams {
  code?: string
  error?: string
  state?: string
}

export interface AuthzCodeExchangePathParams {
  provider: OAuthProviderName
}

export interface GetDeviceProviderPathParams {
  provider: OAuthProviderName
}

export interface ExchangeDeviceTokenPathParams {
  provider: OAuthProviderName
}

export interface GetMagicLinkPathParams {
  clientId: TypedUuidForMagicLinkId
}

export interface CreateMagicLinkRedirectUriPathParams {
  clientId: TypedUuidForMagicLinkId
}

export interface DeleteMagicLinkRedirectUriPathParams {
  clientId: TypedUuidForMagicLinkId
  redirectUriId: TypedUuidForMagicLinkRedirectUriId
}

export interface CreateMagicLinkSecretPathParams {
  clientId: TypedUuidForMagicLinkId
}

export interface DeleteMagicLinkSecretPathParams {
  clientId: TypedUuidForMagicLinkId
  secretId: TypedUuidForMagicLinkSecretId
}

export interface GetMappersQueryParams {
  includeDepleted?: boolean
}

export interface DeleteMapperPathParams {
  mapperId: TypedUuidForMapperId
}

export interface GetOauthClientPathParams {
  clientId: TypedUuidForOAuthClientId
}

export interface CreateOauthClientRedirectUriPathParams {
  clientId: TypedUuidForOAuthClientId
}

export interface DeleteOauthClientRedirectUriPathParams {
  clientId: TypedUuidForOAuthClientId
  redirectUriId: TypedUuidForOAuthRedirectUriId
}

export interface CreateOauthClientSecretPathParams {
  clientId: TypedUuidForOAuthClientId
}

export interface DeleteOauthClientSecretPathParams {
  clientId: TypedUuidForOAuthClientId
  secretId: TypedUuidForOAuthSecretId
}

export interface ViewRfdMetaPathParams {
  number: string
}

export interface ViewRfdAttrPathParams {
  attr: RfdAttrName
  number: string
}

export interface SetRfdAttrPathParams {
  attr: RfdAttrName
  number: string
}

export interface SetRfdContentPathParams {
  number: string
}

export interface ViewRfdDiscussionPathParams {
  number: string
}

export interface ViewRfdPdfPathParams {
  number: string
}

export interface ViewRfdPathParams {
  number: string
}

export interface SetRfdDocumentPathParams {
  number: string
}

export interface ViewRfdRevisionMetaPathParams {
  number: string
  revision: TypedUuidForRfdRevisionId
}

export interface ViewRfdRevisionAttrPathParams {
  attr: RfdAttrName
  number: string
  revision: TypedUuidForRfdRevisionId
}

export interface ViewRfdRevisionDiscussionPathParams {
  number: string
  revision: TypedUuidForRfdRevisionId
}

export interface ViewRfdRevisionPdfPathParams {
  number: string
  revision: TypedUuidForRfdRevisionId
}

export interface ViewRfdRevisionPathParams {
  number: string
  revision: TypedUuidForRfdRevisionId
}

export interface DiscussRfdPathParams {
  number: string
}

export interface PublishRfdPathParams {
  number: string
}

export interface UpdateRfdVisibilityPathParams {
  number: string
}

export interface SearchRfdsQueryParams {
  attributesToCrop?: string
  highlightPostTag?: string
  highlightPreTag?: string
  limit?: number
  offset?: number
  q: string
}

type EmptyObj = Record<string, never>
export class Api extends HttpClient {
  methods = {
    jwksJson: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<Jwks>({
        path: `/.well-known/jwks.json`,
        method: 'GET',
        ...params,
      })
    },
    openidConfiguration: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<OpenIdConfiguration>({
        path: `/.well-known/openid-configuration`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * List details for users
     */
    listApiUsers: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<GetUserResponse_for_RfdPermission[]>({
        path: `/api-user`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Create a new user
     */
    createApiUser: ({
      body,
    }: { body: ApiUserUpdateParams_for_RfdPermission }, params: FetchParams = {}) => {
      return this.request<ApiUser_for_RfdPermission>({
        path: `/api-user`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * View details for a user
     */
    getApiUser: ({
      path,
    }: { path: GetApiUserPathParams }, params: FetchParams = {}) => {
      return this.request<GetUserResponse_for_RfdPermission>({
        path: `/api-user/${path.userId}`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Update the permissions assigned to a given user
     */
    updateApiUser: ({
      path,
      body,
    }: { path: UpdateApiUserPathParams; body: ApiUserUpdateParams_for_RfdPermission }, params: FetchParams = {}) => {
      return this.request<ApiUser_for_RfdPermission>({
        path: `/api-user/${path.userId}`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Set the contact email for a user
     */
    setApiUserContactEmail: ({
      path,
      body,
    }: { path: SetApiUserContactEmailPathParams; body: ApiUserEmailUpdateParams }, params: FetchParams = {}) => {
      return this.request<ApiUserContactEmail>({
        path: `/api-user/${path.userId}/contact/email`,
        method: 'PUT',
        body,
        ...params,
      })
    },
    /**
     * Add a user to a group
     */
    addApiUserToGroup: ({
      path,
      body,
    }: { path: AddApiUserToGroupPathParams; body: AddGroupBody }, params: FetchParams = {}) => {
      return this.request<ApiUser_for_RfdPermission>({
        path: `/api-user/${path.userId}/group`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Remove a user from a group
     */
    removeApiUserFromGroup: ({
      path,
    }: { path: RemoveApiUserFromGroupPathParams }, params: FetchParams = {}) => {
      return this.request<ApiUser_for_RfdPermission>({
        path: `/api-user/${path.userId}/group/${path.groupId}`,
        method: 'DELETE',
        ...params,
      })
    },
    /**
     * Link an existing login provider to this user
     */
    linkProvider: ({
      path,
      body,
    }: { path: LinkProviderPathParams; body: ApiUserProviderLinkPayload }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/api-user/${path.userId}/link`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * List api keys for a user
     */
    listApiUserTokens: ({
      path,
    }: { path: ListApiUserTokensPathParams }, params: FetchParams = {}) => {
      return this.request<ApiKeyResponse_for_RfdPermission[]>({
        path: `/api-user/${path.userId}/token`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Create a new api key for a user
     */
    createApiUserToken: (
      {
        path,
        body,
      }: { path: CreateApiUserTokenPathParams; body: ApiKeyCreateParams_for_RfdPermission },
      params: FetchParams = {},
    ) => {
      return this.request<InitialApiKeyResponse_for_RfdPermission>({
        path: `/api-user/${path.userId}/token`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * View details of an api key for a user
     */
    getApiUserToken: ({
      path,
    }: { path: GetApiUserTokenPathParams }, params: FetchParams = {}) => {
      return this.request<ApiKeyResponse_for_RfdPermission>({
        path: `/api-user/${path.userId}/token/${path.apiKeyId}`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Revoke an api key for a user
     */
    deleteApiUserToken: ({
      path,
    }: { path: DeleteApiUserTokenPathParams }, params: FetchParams = {}) => {
      return this.request<ApiKeyResponse_for_RfdPermission>({
        path: `/api-user/${path.userId}/token/${path.apiKeyId}`,
        method: 'DELETE',
        ...params,
      })
    },
    /**
     * Create a new link token for linking this provider to a different api user
     */
    createLinkToken: ({
      path,
      body,
    }: { path: CreateLinkTokenPathParams; body: ApiUserLinkRequestPayload }, params: FetchParams = {}) => {
      return this.request<ApiUserLinkRequestResponse>({
        path: `/api-user-provider/${path.providerId}/link-token`,
        method: 'POST',
        body,
        ...params,
      })
    },
    githubWebhook: ({
      body,
    }: { body: GitHubCommitPayload }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/github`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * List all groups
     */
    getGroups: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<AccessGroup_for_RfdPermission[]>({
        path: `/group`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Create a group
     */
    createGroup: ({
      body,
    }: { body: AccessGroupUpdateParams_for_RfdPermission }, params: FetchParams = {}) => {
      return this.request<AccessGroup_for_RfdPermission>({
        path: `/group`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Update a group
     */
    updateGroup: ({
      path,
      body,
    }: { path: UpdateGroupPathParams; body: AccessGroupUpdateParams_for_RfdPermission }, params: FetchParams = {}) => {
      return this.request<AccessGroup_for_RfdPermission>({
        path: `/group/${path.groupId}`,
        method: 'PUT',
        body,
        ...params,
      })
    },
    /**
     * Delete a group
     */
    deleteGroup: ({
      path,
    }: { path: DeleteGroupPathParams }, params: FetchParams = {}) => {
      return this.request<AccessGroup_for_RfdPermission>({
        path: `/group/${path.groupId}`,
        method: 'DELETE',
        ...params,
      })
    },
    /**
     * List all jobs for a RFD
     */
    listJobs: ({
      query,
    }: { query: ListJobsQueryParams }, params: FetchParams = {}) => {
      return this.request<Job[]>({
        path: `/job`,
        method: 'GET',
        query,
        ...params,
      })
    },
    /**
     * Exchange a magic link access code for an access token
     */
    magicLinkExchange: ({
      path,
      body,
    }: { path: MagicLinkExchangePathParams; body: MagicLinkExchangeRequest }, params: FetchParams = {}) => {
      return this.request<MagicLinkExchangeResponse>({
        path: `/login/magic/${path.channel}/exchange`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Send a new magic link authentication link
     */
    magicLinkSend: ({
      path,
      body,
    }: { path: MagicLinkSendPathParams; body: MagicLinkSendRequest }, params: FetchParams = {}) => {
      return this.request<MagicLinkSendResponse>({
        path: `/login/magic/${path.channel}/send`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Generate the remote provider login url and redirect the user
     */
    authzCodeRedirect: ({
      path,
      query,
    }: { path: AuthzCodeRedirectPathParams; query: AuthzCodeRedirectQueryParams }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/login/oauth/${path.provider}/code/authorize`,
        method: 'GET',
        query,
        ...params,
      })
    },
    /**
     * Handle return calls from a remote OAuth provider
     */
    authzCodeCallback: ({
      path,
      query = {},
    }: { path: AuthzCodeCallbackPathParams; query?: AuthzCodeCallbackQueryParams }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/login/oauth/${path.provider}/code/callback`,
        method: 'GET',
        query,
        ...params,
      })
    },
    /**
     * Exchange an authorization code for an access token
     */
    authzCodeExchange: ({
      path,
    }: { path: AuthzCodeExchangePathParams }, params: FetchParams = {}) => {
      return this.request<OAuthAuthzCodeExchangeResponse>({
        path: `/login/oauth/${path.provider}/code/token`,
        method: 'POST',
        ...params,
      })
    },
    /**
     * Retrieve the metadata about an OAuth provider
     */
    getDeviceProvider: ({
      path,
    }: { path: GetDeviceProviderPathParams }, params: FetchParams = {}) => {
      return this.request<OAuthProviderInfo>({
        path: `/login/oauth/${path.provider}/device`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Exchange an OAuth device code request for an access token
     */
    exchangeDeviceToken: ({
      path,
    }: { path: ExchangeDeviceTokenPathParams }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/login/oauth/${path.provider}/device/exchange`,
        method: 'POST',
        ...params,
      })
    },
    /**
     * List Magic Link clients
     */
    listMagicLinks: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<MagicLink[]>({
        path: `/magic/client`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Create a new Magic Link Client
     */
    createMagicLink: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<MagicLink>({
        path: `/magic/client`,
        method: 'POST',
        ...params,
      })
    },
    /**
     * Get a Magic Link Client
     */
    getMagicLink: ({
      path,
    }: { path: GetMagicLinkPathParams }, params: FetchParams = {}) => {
      return this.request<MagicLink>({
        path: `/magic/client/${path.clientId}`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Add a Magic Link client redirect uri
     */
    createMagicLinkRedirectUri: ({
      path,
      body,
    }: { path: CreateMagicLinkRedirectUriPathParams; body: AddMagicLinkRedirectBody }, params: FetchParams = {}) => {
      return this.request<MagicLinkRedirectUri>({
        path: `/magic/client/${path.clientId}/redirect_uri`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Delete a Magic Link client redirect uri
     */
    deleteMagicLinkRedirectUri: ({
      path,
    }: { path: DeleteMagicLinkRedirectUriPathParams }, params: FetchParams = {}) => {
      return this.request<MagicLinkRedirectUri>({
        path: `/magic/client/${path.clientId}/redirect_uri/${path.redirectUriId}`,
        method: 'DELETE',
        ...params,
      })
    },
    /**
     * Add a Magic Link client secret
     */
    createMagicLinkSecret: ({
      path,
    }: { path: CreateMagicLinkSecretPathParams }, params: FetchParams = {}) => {
      return this.request<InitialMagicLinkSecretResponse>({
        path: `/magic/client/${path.clientId}/secret`,
        method: 'POST',
        ...params,
      })
    },
    /**
     * Delete a Magic Link client secret
     */
    deleteMagicLinkSecret: ({
      path,
    }: { path: DeleteMagicLinkSecretPathParams }, params: FetchParams = {}) => {
      return this.request<MagicLinkSecret>({
        path: `/magic/client/${path.clientId}/secret/${path.secretId}`,
        method: 'DELETE',
        ...params,
      })
    },
    /**
     * List all mappers
     */
    getMappers: ({
      query = {},
    }: { query?: GetMappersQueryParams }, params: FetchParams = {}) => {
      return this.request<Mapper[]>({
        path: `/mapper`,
        method: 'GET',
        query,
        ...params,
      })
    },
    /**
     * Create a mapper
     */
    createMapper: ({
      body,
    }: { body: CreateMapper }, params: FetchParams = {}) => {
      return this.request<Mapper>({
        path: `/mapper`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Delete a mapper
     */
    deleteMapper: ({
      path,
    }: { path: DeleteMapperPathParams }, params: FetchParams = {}) => {
      return this.request<Mapper>({
        path: `/mapper/${path.mapperId}`,
        method: 'DELETE',
        ...params,
      })
    },
    /**
     * List OAuth clients
     */
    listOauthClients: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<OAuthClient[]>({
        path: `/oauth/client`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Create a new OAuth Client
     */
    createOauthClient: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<OAuthClient>({
        path: `/oauth/client`,
        method: 'POST',
        ...params,
      })
    },
    /**
     * Get an new OAuth Client
     */
    getOauthClient: ({
      path,
    }: { path: GetOauthClientPathParams }, params: FetchParams = {}) => {
      return this.request<OAuthClient>({
        path: `/oauth/client/${path.clientId}`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Add an OAuth client redirect uri
     */
    createOauthClientRedirectUri: (
      {
        path,
        body,
      }: { path: CreateOauthClientRedirectUriPathParams; body: AddOAuthClientRedirectBody },
      params: FetchParams = {},
    ) => {
      return this.request<OAuthClientRedirectUri>({
        path: `/oauth/client/${path.clientId}/redirect_uri`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Delete an OAuth client redirect uri
     */
    deleteOauthClientRedirectUri: ({
      path,
    }: { path: DeleteOauthClientRedirectUriPathParams }, params: FetchParams = {}) => {
      return this.request<OAuthClientRedirectUri>({
        path: `/oauth/client/${path.clientId}/redirect_uri/${path.redirectUriId}`,
        method: 'DELETE',
        ...params,
      })
    },
    /**
     * Add an OAuth client secret
     */
    createOauthClientSecret: ({
      path,
    }: { path: CreateOauthClientSecretPathParams }, params: FetchParams = {}) => {
      return this.request<InitialOAuthClientSecretResponse>({
        path: `/oauth/client/${path.clientId}/secret`,
        method: 'POST',
        ...params,
      })
    },
    /**
     * Delete an OAuth client secret
     */
    deleteOauthClientSecret: ({
      path,
    }: { path: DeleteOauthClientSecretPathParams }, params: FetchParams = {}) => {
      return this.request<OAuthClientSecret>({
        path: `/oauth/client/${path.clientId}/secret/${path.secretId}`,
        method: 'DELETE',
        ...params,
      })
    },
    /**
     * List all available RFDs
     */
    listRfds: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<RfdWithoutContent[]>({
        path: `/rfd`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Create a new RFD
     */
    reserveRfd: ({
      body,
    }: { body: ReserveRfdBody }, params: FetchParams = {}) => {
      return this.request<ReserveRfdResponse>({
        path: `/rfd`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Get the latest representation of an RFD's metadata
     */
    viewRfdMeta: ({
      path,
    }: { path: ViewRfdMetaPathParams }, params: FetchParams = {}) => {
      return this.request<RfdWithoutContent>({
        path: `/rfd/${path.number}`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Get the an attribute of the latest revision of a RFD
     */
    viewRfdAttr: ({
      path,
    }: { path: ViewRfdAttrPathParams }, params: FetchParams = {}) => {
      return this.request<RfdAttr>({
        path: `/rfd/${path.number}/attr/${path.attr}`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Set an attribute of a RFD
     */
    setRfdAttr: ({
      path,
      body,
    }: { path: SetRfdAttrPathParams; body: RfdAttrValue }, params: FetchParams = {}) => {
      return this.request<RfdAttr>({
        path: `/rfd/${path.number}/attr/${path.attr}`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Replace the contents of a RFD
     */
    setRfdContent: ({
      path,
      body,
    }: { path: SetRfdContentPathParams; body: RfdUpdateContentBody }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/rfd/${path.number}/content`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Get the comments related to the latest revision of a RFD
     */
    viewRfdDiscussion: ({
      path,
    }: { path: ViewRfdDiscussionPathParams }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/rfd/${path.number}/discussion`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Get the PDF locations of the latest revision of a RFD
     */
    viewRfdPdf: ({
      path,
    }: { path: ViewRfdPdfPathParams }, params: FetchParams = {}) => {
      return this.request<RfdWithPdf>({
        path: `/rfd/${path.number}/pdf`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Get the raw contents of the latest revision of a RFD
     */
    viewRfd: ({
      path,
    }: { path: ViewRfdPathParams }, params: FetchParams = {}) => {
      return this.request<RfdWithRaw>({
        path: `/rfd/${path.number}/raw`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Replace the full document of a RFD
     */
    setRfdDocument: ({
      path,
      body,
    }: { path: SetRfdDocumentPathParams; body: RfdUpdateBody }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/rfd/${path.number}/raw`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Get an RFD revision's metadata
     */
    viewRfdRevisionMeta: ({
      path,
    }: { path: ViewRfdRevisionMetaPathParams }, params: FetchParams = {}) => {
      return this.request<RfdWithoutContent>({
        path: `/rfd/${path.number}/revision/${path.revision}`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Get the an attribute of a revision of a RFD
     */
    viewRfdRevisionAttr: ({
      path,
    }: { path: ViewRfdRevisionAttrPathParams }, params: FetchParams = {}) => {
      return this.request<RfdAttr>({
        path: `/rfd/${path.number}/revision/${path.revision}/attr/${path.attr}`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Get the comments related to a revision of a RFD
     */
    viewRfdRevisionDiscussion: ({
      path,
    }: { path: ViewRfdRevisionDiscussionPathParams }, params: FetchParams = {}) => {
      return this.request<void>({
        path: `/rfd/${path.number}/revision/${path.revision}/discussion`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Get the PDF locations of a revision of a RFD
     */
    viewRfdRevisionPdf: ({
      path,
    }: { path: ViewRfdRevisionPdfPathParams }, params: FetchParams = {}) => {
      return this.request<RfdWithPdf>({
        path: `/rfd/${path.number}/revision/${path.revision}/pdf`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Get the raw contents of a revision of a RFD
     */
    viewRfdRevision: ({
      path,
    }: { path: ViewRfdRevisionPathParams }, params: FetchParams = {}) => {
      return this.request<RfdWithRaw>({
        path: `/rfd/${path.number}/revision/${path.revision}/raw`,
        method: 'GET',
        ...params,
      })
    },
    /**
     * Open a RFD for discussion
     */
    discussRfd: ({
      path,
    }: { path: DiscussRfdPathParams }, params: FetchParams = {}) => {
      return this.request<RfdAttr>({
        path: `/rfd/${path.number}/state/discuss`,
        method: 'POST',
        ...params,
      })
    },
    /**
     * Publish a RFD
     */
    publishRfd: ({
      path,
    }: { path: PublishRfdPathParams }, params: FetchParams = {}) => {
      return this.request<RfdAttr>({
        path: `/rfd/${path.number}/state/publish`,
        method: 'POST',
        ...params,
      })
    },
    /**
     * Modify the visibility of a RFD
     */
    updateRfdVisibility: ({
      path,
      body,
    }: { path: UpdateRfdVisibilityPathParams; body: RfdVisibility }, params: FetchParams = {}) => {
      return this.request<Rfd>({
        path: `/rfd/${path.number}/visibility`,
        method: 'POST',
        body,
        ...params,
      })
    },
    /**
     * Search the RFD index and get a list of results
     */
    searchRfds: ({
      query,
    }: { query: SearchRfdsQueryParams }, params: FetchParams = {}) => {
      return this.request<SearchResults>({
        path: `/rfd-search`,
        method: 'GET',
        query,
        ...params,
      })
    },
    /**
     * View details for the calling user
     */
    getSelf: (_: EmptyObj, params: FetchParams = {}) => {
      return this.request<GetUserResponse_for_RfdPermission>({
        path: `/self`,
        method: 'GET',
        ...params,
      })
    },
  }
  ws = {}
}

export default Api
