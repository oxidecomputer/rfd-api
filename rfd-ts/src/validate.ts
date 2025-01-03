/* eslint-disable */

/**
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, you can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright Oxide Computer Company
 */

import { z, ZodType } from 'zod'
import { processResponseBody, uniqueItems } from './util'

/**
 * Zod only supports string enums at the moment. A previous issue was opened
 * and closed as stale but it provided a hint on how to implement it.
 *
 * @see https://github.com/colinhacks/zod/issues/1118
 * TODO: PR an update for zod to support other native enum types
 */
const IntEnum = <T extends readonly number[]>(values: T) =>
  z.number().refine((v) => values.includes(v)) as ZodType<T[number]>

/** Helper to ensure booleans provided as strings end up with the correct value */
const SafeBoolean = z.preprocess(v => v === 'false' ? false : v, z.coerce.boolean())

export const TypedUuidForUserId = z.preprocess(processResponseBody, z.string().uuid())

export const TypedUuidForApiKeyId = z.preprocess(processResponseBody, z.string().uuid())

export const TypedUuidForAccessGroupId = z.preprocess(processResponseBody, z.string().uuid())

export const TypedUuidForMapperId = z.preprocess(processResponseBody, z.string().uuid())

export const TypedUuidForOAuthClientId = z.preprocess(processResponseBody, z.string().uuid())

export const TypedUuidForMagicLinkId = z.preprocess(processResponseBody, z.string().uuid())

export const RfdPermission = z.preprocess(
  processResponseBody,
  z.union([
    z.enum([
      'GetRfdsAssigned',
      'GetRfdsAll',
      'CreateRfd',
      'UpdateRfdsAssigned',
      'UpdateRfdsAll',
      'ManageRfdsVisibilityAssigned',
      'ManageRfdsVisibilityAll',
      'GetDiscussionsAssigned',
      'GetDiscussionsAll',
      'SearchRfds',
      'CreateApiUser',
      'GetApiUserSelf',
      'GetApiUsersAssigned',
      'GetApiUsersAll',
      'ManageApiUsersAssigned',
      'ManageApiUsersAll',
      'CreateApiKeySelf',
      'CreateApiKeyAssigned',
      'CreateApiKeyAll',
      'GetApiKeysAssigned',
      'GetApiKeysAll',
      'ManageApiKeysAssigned',
      'ManageApiKeysAll',
      'CreateUserApiProviderLinkToken',
      'CreateGroup',
      'GetGroupsJoined',
      'GetGroupsAll',
      'ManageGroupsAssigned',
      'ManageGroupsAll',
      'ManageGroupMembershipsAssigned',
      'ManageGroupMembershipsAll',
      'CreateMapper',
      'GetMappersAll',
      'ManageMappersAssigned',
      'ManageMappersAll',
      'CreateOAuthClient',
      'GetOAuthClientsAssigned',
      'GetOAuthClientsAll',
      'ManageOAuthClientsAssigned',
      'ManageOAuthClientsAll',
      'CreateMagicLinkClient',
      'GetMagicLinkClientsAssigned',
      'GetMagicLinkClientsAll',
      'ManageMagicLinkClientsAssigned',
      'ManageMagicLinkClientsAll',
      'CreateAccessToken',
    ]),
    z.object({ 'GetRfd': z.number().min(-2147483647).max(2147483647) }),
    z.object({ 'GetRfds': z.number().min(-2147483647).max(2147483647).array().refine(...uniqueItems) }),
    z.object({ 'UpdateRfd': z.number().min(-2147483647).max(2147483647) }),
    z.object({ 'UpdateRfds': z.number().min(-2147483647).max(2147483647).array().refine(...uniqueItems) }),
    z.object({ 'ManageRfdVisibility': z.number().min(-2147483647).max(2147483647) }),
    z.object({ 'ManageRfdsVisibility': z.number().min(-2147483647).max(2147483647).array().refine(...uniqueItems) }),
    z.object({ 'GetDiscussion': z.number().min(-2147483647).max(2147483647) }),
    z.object({ 'GetDiscussions': z.number().min(-2147483647).max(2147483647).array().refine(...uniqueItems) }),
    z.object({ 'GetApiUser': TypedUuidForUserId }),
    z.object({ 'GetApiUsers': TypedUuidForUserId.array().refine(...uniqueItems) }),
    z.object({ 'ManageApiUser': TypedUuidForUserId }),
    z.object({ 'ManageApiUsers': TypedUuidForUserId.array().refine(...uniqueItems) }),
    z.object({ 'CreateApiKey': TypedUuidForUserId }),
    z.object({ 'GetApiKey': TypedUuidForApiKeyId }),
    z.object({ 'GetApiKeys': TypedUuidForApiKeyId.array().refine(...uniqueItems) }),
    z.object({ 'ManageApiKey': TypedUuidForApiKeyId }),
    z.object({ 'ManageApiKeys': TypedUuidForApiKeyId.array().refine(...uniqueItems) }),
    z.object({ 'GetGroup': TypedUuidForAccessGroupId }),
    z.object({ 'ManageGroup': TypedUuidForAccessGroupId }),
    z.object({ 'ManageGroups': TypedUuidForAccessGroupId.array().refine(...uniqueItems) }),
    z.object({ 'ManageGroupMembership': TypedUuidForAccessGroupId }),
    z.object({ 'ManageGroupMemberships': TypedUuidForAccessGroupId.array().refine(...uniqueItems) }),
    z.object({ 'ManageMapper': TypedUuidForMapperId }),
    z.object({ 'ManageMappers': TypedUuidForMapperId.array().refine(...uniqueItems) }),
    z.object({ 'GetOAuthClient': TypedUuidForOAuthClientId }),
    z.object({ 'GetOAuthClients': TypedUuidForOAuthClientId.array().refine(...uniqueItems) }),
    z.object({ 'ManageOAuthClient': TypedUuidForOAuthClientId }),
    z.object({ 'ManageOAuthClients': TypedUuidForOAuthClientId.array().refine(...uniqueItems) }),
    z.object({ 'GetMagicLinkClient': TypedUuidForMagicLinkId }),
    z.object({ 'GetMagicLinkClients': TypedUuidForMagicLinkId.array().refine(...uniqueItems) }),
    z.object({ 'ManageMagicLinkClient': TypedUuidForMagicLinkId }),
    z.object({ 'ManageMagicLinkClients': TypedUuidForMagicLinkId.array().refine(...uniqueItems) }),
    z.object({ 'Unsupported': z.record(z.unknown()) }),
  ]),
)

export const Permissions_for_RfdPermission = z.preprocess(processResponseBody, RfdPermission.array())

export const AccessGroupUpdateParams_for_RfdPermission = z.preprocess(
  processResponseBody,
  z.object({ 'name': z.string(), 'permissions': Permissions_for_RfdPermission }),
)

export const AccessGroup_for_RfdPermission = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'id': TypedUuidForAccessGroupId,
    'name': z.string(),
    'permissions': Permissions_for_RfdPermission,
    'updatedAt': z.coerce.date(),
  }),
)

export const AccessTokenExchangeRequest = z.preprocess(
  processResponseBody,
  z.object({ 'deviceCode': z.string(), 'expiresAt': z.coerce.date().optional(), 'grantType': z.string() }),
)

export const AddGroupBody = z.preprocess(processResponseBody, z.object({ 'groupId': TypedUuidForAccessGroupId }))

export const AddMagicLinkRedirectBody = z.preprocess(processResponseBody, z.object({ 'redirectUri': z.string() }))

export const AddOAuthClientRedirectBody = z.preprocess(processResponseBody, z.object({ 'redirectUri': z.string() }))

export const ApiKeyCreateParams_for_RfdPermission = z.preprocess(
  processResponseBody,
  z.object({ 'expiresAt': z.coerce.date(), 'permissions': Permissions_for_RfdPermission.optional() }),
)

export const ApiKeyResponse_for_RfdPermission = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'id': TypedUuidForApiKeyId,
    'permissions': Permissions_for_RfdPermission.optional(),
  }),
)

export const TypedUuidForUserProviderId = z.preprocess(processResponseBody, z.string().uuid())

export const ApiUserContactEmail = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'email': z.string(),
    'id': TypedUuidForUserProviderId,
    'updatedAt': z.coerce.date(),
    'userId': TypedUuidForUserId,
  }),
)

export const ApiUserEmailUpdateParams = z.preprocess(processResponseBody, z.object({ 'email': z.string() }))

export const ApiUserLinkRequestPayload = z.preprocess(processResponseBody, z.object({ 'userId': TypedUuidForUserId }))

export const SecretString = z.preprocess(processResponseBody, z.string())

export const ApiUserLinkRequestResponse = z.preprocess(processResponseBody, z.object({ 'token': SecretString }))

export const ApiUserProvider = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'displayNames': z.string().array(),
    'emails': z.string().array(),
    'id': TypedUuidForUserProviderId,
    'provider': z.string(),
    'providerId': z.string(),
    'updatedAt': z.coerce.date(),
    'userId': TypedUuidForUserId,
  }),
)

export const ApiUserProviderLinkPayload = z.preprocess(processResponseBody, z.object({ 'token': z.string() }))

export const ApiUserUpdateParams_for_RfdPermission = z.preprocess(
  processResponseBody,
  z.object({
    'groupIds': TypedUuidForAccessGroupId.array().refine(...uniqueItems),
    'permissions': Permissions_for_RfdPermission,
  }),
)

export const ApiUser_for_RfdPermission = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'groups': TypedUuidForAccessGroupId.array().refine(...uniqueItems),
    'id': TypedUuidForUserId,
    'permissions': Permissions_for_RfdPermission,
    'updatedAt': z.coerce.date(),
  }),
)

export const CommitSha = z.preprocess(processResponseBody, z.string())

export const ContentFormat = z.preprocess(processResponseBody, z.enum(['asciidoc', 'markdown']))

export const CreateMapper = z.preprocess(
  processResponseBody,
  z.object({
    'maxActivations': z.number().min(-2147483647).max(2147483647).optional(),
    'name': z.string(),
    'rule': z.record(z.unknown()),
  }),
)

/**
 * Error information from a response.
 */
export const Error = z.preprocess(
  processResponseBody,
  z.object({ 'errorCode': z.string().optional(), 'message': z.string(), 'requestId': z.string() }),
)

export const FileSha = z.preprocess(processResponseBody, z.string())

export const FormattedSearchResultHit = z.preprocess(
  processResponseBody,
  z.object({
    'anchor': z.string().optional(),
    'content': z.string().optional(),
    'hierarchy': z.string().array(),
    'hierarchyRadio': z.string().array(),
    'objectId': z.string(),
    'rfdNumber': z.number().min(0),
    'url': z.string().optional(),
  }),
)

export const GetUserResponse_for_RfdPermission = z.preprocess(
  processResponseBody,
  z.object({ 'info': ApiUser_for_RfdPermission, 'providers': ApiUserProvider.array() }),
)

export const GitHubCommit = z.preprocess(
  processResponseBody,
  z.object({
    'added': z.string().array(),
    'id': z.string(),
    'modified': z.string().array(),
    'removed': z.string().array(),
    'timestamp': z.coerce.date(),
  }),
)

export const GitHubInstallation = z.preprocess(
  processResponseBody,
  z.object({ 'id': z.number().min(0), 'nodeId': z.string() }),
)

export const GitHubRepositoryOwner = z.preprocess(processResponseBody, z.object({ 'login': z.string() }))

export const GitHubRepository = z.preprocess(
  processResponseBody,
  z.object({
    'defaultBranch': z.string(),
    'id': z.number().min(0),
    'name': z.string(),
    'nodeId': z.string(),
    'owner': GitHubRepositoryOwner,
  }),
)

export const GitHubSender = z.preprocess(
  processResponseBody,
  z.object({ 'id': z.number().min(0), 'login': z.string(), 'nodeId': z.string(), 'type': z.string() }),
)

export const GitHubCommitPayload = z.preprocess(
  processResponseBody,
  z.object({
    'commits': GitHubCommit.array(),
    'headCommit': GitHubCommit.optional(),
    'installation': GitHubInstallation,
    'ref': z.string(),
    'repository': GitHubRepository,
    'sender': GitHubSender,
  }),
)

export const InitialApiKeyResponse_for_RfdPermission = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'id': TypedUuidForApiKeyId,
    'key': SecretString,
    'permissions': Permissions_for_RfdPermission.optional(),
  }),
)

export const TypedUuidForMagicLinkSecretId = z.preprocess(processResponseBody, z.string().uuid())

export const InitialMagicLinkSecretResponse = z.preprocess(
  processResponseBody,
  z.object({ 'createdAt': z.coerce.date(), 'id': TypedUuidForMagicLinkSecretId, 'key': SecretString }),
)

export const TypedUuidForOAuthSecretId = z.preprocess(processResponseBody, z.string().uuid())

export const InitialOAuthClientSecretResponse = z.preprocess(
  processResponseBody,
  z.object({ 'createdAt': z.coerce.date(), 'id': TypedUuidForOAuthSecretId, 'key': SecretString }),
)

export const Jwk = z.preprocess(
  processResponseBody,
  z.object({ 'e': z.string(), 'kid': z.string(), 'kty': z.string(), 'n': z.string(), 'use': z.string() }),
)

export const Jwks = z.preprocess(processResponseBody, z.object({ 'keys': Jwk.array() }))

export const TypedUuidForMagicLinkRedirectUriId = z.preprocess(processResponseBody, z.string().uuid())

export const MagicLinkRedirectUri = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'id': TypedUuidForMagicLinkRedirectUriId,
    'magicLinkClientId': TypedUuidForMagicLinkId,
    'redirectUri': z.string(),
  }),
)

export const MagicLinkSecret = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'id': TypedUuidForMagicLinkSecretId,
    'magicLinkClientId': TypedUuidForMagicLinkId,
    'secretSignature': z.string(),
  }),
)

export const MagicLink = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'id': TypedUuidForMagicLinkId,
    'redirectUris': MagicLinkRedirectUri.array(),
    'secrets': MagicLinkSecret.array(),
  }),
)

export const TypedUuidForMagicLinkAttemptId = z.preprocess(processResponseBody, z.string().uuid())

export const MagicLinkExchangeRequest = z.preprocess(
  processResponseBody,
  z.object({ 'attemptId': TypedUuidForMagicLinkAttemptId, 'recipient': z.string(), 'secret': z.string() }),
)

export const MagicLinkExchangeResponse = z.preprocess(
  processResponseBody,
  z.object({ 'accessToken': z.string(), 'expiresIn': z.number(), 'tokenType': z.string() }),
)

export const MagicLinkMedium = z.preprocess(processResponseBody, z.enum(['email']))

export const MagicLinkSendRequest = z.preprocess(
  processResponseBody,
  z.object({
    'expiresIn': z.number(),
    'medium': MagicLinkMedium,
    'recipient': z.string(),
    'redirectUri': z.string(),
    'scope': z.string().optional(),
    'secret': z.string(),
  }),
)

export const MagicLinkSendResponse = z.preprocess(
  processResponseBody,
  z.object({ 'attemptId': TypedUuidForMagicLinkAttemptId }),
)

export const Mapper = z.preprocess(
  processResponseBody,
  z.object({
    'activations': z.number().min(-2147483647).max(2147483647).optional(),
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'depletedAt': z.coerce.date().optional(),
    'id': TypedUuidForMapperId,
    'maxActivations': z.number().min(-2147483647).max(2147483647).optional(),
    'name': z.string(),
    'rule': z.record(z.unknown()),
    'updatedAt': z.coerce.date(),
  }),
)

export const OAuthAuthzCodeExchangeBody = z.preprocess(
  processResponseBody,
  z.object({
    'clientId': TypedUuidForOAuthClientId,
    'clientSecret': SecretString,
    'code': z.string(),
    'grantType': z.string(),
    'pkceVerifier': z.string().optional(),
    'redirectUri': z.string(),
  }),
)

export const OAuthAuthzCodeExchangeResponse = z.preprocess(
  processResponseBody,
  z.object({ 'accessToken': z.string(), 'expiresIn': z.number(), 'tokenType': z.string() }),
)

export const TypedUuidForOAuthRedirectUriId = z.preprocess(processResponseBody, z.string().uuid())

export const OAuthClientRedirectUri = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'id': TypedUuidForOAuthRedirectUriId,
    'oauthClientId': TypedUuidForOAuthClientId,
    'redirectUri': z.string(),
  }),
)

export const OAuthClientSecret = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'id': TypedUuidForOAuthSecretId,
    'oauthClientId': TypedUuidForOAuthClientId,
    'secretSignature': z.string(),
  }),
)

export const OAuthClient = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'id': TypedUuidForOAuthClientId,
    'redirectUris': OAuthClientRedirectUri.array(),
    'secrets': OAuthClientSecret.array(),
  }),
)

export const OAuthProviderName = z.preprocess(processResponseBody, z.enum(['github', 'google']))

export const OAuthProviderInfo = z.preprocess(
  processResponseBody,
  z.object({
    'authUrlEndpoint': z.string(),
    'clientId': z.string(),
    'deviceCodeEndpoint': z.string(),
    'provider': OAuthProviderName,
    'scopes': z.string().array(),
    'tokenEndpoint': z.string(),
  }),
)

export const OpenIdConfiguration = z.preprocess(processResponseBody, z.object({ 'jwksUri': z.string() }))

export const PdfSource = z.preprocess(processResponseBody, z.enum(['github', 'google']))

export const ReserveRfdBody = z.preprocess(
  processResponseBody,
  z.object({ 'content': z.string().optional(), 'title': z.string() }),
)

export const ReserveRfdResponse = z.preprocess(
  processResponseBody,
  z.object({ 'number': z.number().min(-2147483647).max(2147483647) }),
)

export const TypedUuidForRfdRevisionId = z.preprocess(processResponseBody, z.string().uuid())

export const TypedUuidForRfdId = z.preprocess(processResponseBody, z.string().uuid())

export const RfdRevision = z.preprocess(
  processResponseBody,
  z.object({
    'authors': z.string().optional(),
    'commit': CommitSha,
    'committedAt': z.coerce.date(),
    'content': z.string(),
    'contentFormat': ContentFormat,
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'discussion': z.string().optional(),
    'id': TypedUuidForRfdRevisionId,
    'labels': z.string().optional(),
    'rfdId': TypedUuidForRfdId,
    'sha': FileSha,
    'state': z.string().optional(),
    'title': z.string(),
    'updatedAt': z.coerce.date(),
  }),
)

export const Visibility = z.preprocess(processResponseBody, z.enum(['public', 'private']))

export const Rfd = z.preprocess(
  processResponseBody,
  z.object({
    'content': RfdRevision,
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'id': TypedUuidForRfdId,
    'link': z.string().optional(),
    'rfdNumber': z.number().min(-2147483647).max(2147483647),
    'updatedAt': z.coerce.date(),
    'visibility': Visibility,
  }),
)

export const RfdState = z.preprocess(
  processResponseBody,
  z.enum(['abandoned', 'committed', 'discussion', 'ideation', 'prediscussion', 'published']),
)

export const RfdAttr = z.preprocess(
  processResponseBody,
  z.union([
    z.object({ 'discussion': z.string() }),
    z.object({ 'labels': z.string() }),
    z.object({ 'state': RfdState }),
  ]),
)

export const RfdAttrValue = z.preprocess(
  processResponseBody,
  z.object({ 'message': z.string().optional(), 'value': z.string() }),
)

export const TypedUuidForRfdPdfId = z.preprocess(processResponseBody, z.string().uuid())

export const RfdPdf = z.preprocess(
  processResponseBody,
  z.object({
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'externalId': z.string(),
    'id': TypedUuidForRfdPdfId,
    'link': z.string(),
    'rfdId': TypedUuidForRfdId,
    'rfdRevisionId': TypedUuidForRfdRevisionId,
    'source': PdfSource,
    'updatedAt': z.coerce.date(),
  }),
)

export const RfdRevisionPdf = z.preprocess(
  processResponseBody,
  z.object({
    'authors': z.string().optional(),
    'commit': CommitSha,
    'committedAt': z.coerce.date(),
    'content': RfdPdf.array(),
    'contentFormat': ContentFormat,
    'createdAt': z.coerce.date(),
    'deletedAt': z.coerce.date().optional(),
    'discussion': z.string().optional(),
    'id': TypedUuidForRfdRevisionId,
    'labels': z.string().optional(),
    'rfdId': TypedUuidForRfdId,
    'sha': FileSha,
    'state': z.string().optional(),
    'title': z.string(),
    'updatedAt': z.coerce.date(),
  }),
)

export const RfdUpdateBody = z.preprocess(
  processResponseBody,
  z.object({ 'document': z.string(), 'message': z.string().optional() }),
)

export const RfdUpdateContentBody = z.preprocess(
  processResponseBody,
  z.object({ 'content': z.string(), 'message': z.string().optional() }),
)

export const RfdVisibility = z.preprocess(processResponseBody, z.object({ 'visibility': Visibility }))

export const RfdWithContent = z.preprocess(
  processResponseBody,
  z.object({
    'authors': z.string().optional(),
    'commit': CommitSha,
    'committedAt': z.coerce.date(),
    'content': z.string(),
    'discussion': z.string().optional(),
    'format': ContentFormat,
    'id': TypedUuidForRfdId,
    'labels': z.string().optional(),
    'link': z.string().optional(),
    'rfdNumber': z.number().min(-2147483647).max(2147483647),
    'sha': FileSha,
    'state': z.string().optional(),
    'title': z.string(),
    'visibility': Visibility,
  }),
)

export const RfdWithoutContent = z.preprocess(
  processResponseBody,
  z.object({
    'authors': z.string().optional(),
    'commit': CommitSha,
    'committedAt': z.coerce.date(),
    'discussion': z.string().optional(),
    'format': ContentFormat,
    'id': TypedUuidForRfdId,
    'labels': z.string().optional(),
    'link': z.string().optional(),
    'rfdNumber': z.number().min(-2147483647).max(2147483647),
    'sha': FileSha,
    'state': z.string().optional(),
    'title': z.string(),
    'visibility': Visibility,
  }),
)

export const SearchResultHit = z.preprocess(
  processResponseBody,
  z.object({
    'anchor': z.string().optional(),
    'content': z.string(),
    'formatted': FormattedSearchResultHit.optional(),
    'hierarchy': z.string().array(),
    'hierarchyRadio': z.string().array(),
    'objectId': z.string(),
    'rfdNumber': z.number().min(0),
    'url': z.string().optional(),
  }),
)

export const SearchResults = z.preprocess(
  processResponseBody,
  z.object({
    'hits': SearchResultHit.array(),
    'limit': z.number().min(0).optional(),
    'offset': z.number().min(0).optional(),
    'query': z.string(),
  }),
)

export const RfdAttrName = z.preprocess(processResponseBody, z.enum(['discussion', 'labels', 'state']))

export const JwksJsonParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const OpenidConfigurationParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const CreateApiUserParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const GetApiUserParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const UpdateApiUserParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const SetApiUserContactEmailParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const AddApiUserToGroupParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const RemoveApiUserFromGroupParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      groupId: TypedUuidForAccessGroupId,
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const LinkProviderParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const ListApiUserTokensParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const CreateApiUserTokenParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const GetApiUserTokenParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      apiKeyId: TypedUuidForApiKeyId,
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const DeleteApiUserTokenParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      apiKeyId: TypedUuidForApiKeyId,
      userId: TypedUuidForUserId,
    }),
    query: z.object({}),
  }),
)

export const CreateLinkTokenParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      providerId: TypedUuidForUserProviderId,
    }),
    query: z.object({}),
  }),
)

export const GithubWebhookParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const GetGroupsParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const CreateGroupParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const UpdateGroupParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      groupId: TypedUuidForAccessGroupId,
    }),
    query: z.object({}),
  }),
)

export const DeleteGroupParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      groupId: TypedUuidForAccessGroupId,
    }),
    query: z.object({}),
  }),
)

export const MagicLinkExchangeParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      channel: z.string(),
    }),
    query: z.object({}),
  }),
)

export const MagicLinkSendParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      channel: z.string(),
    }),
    query: z.object({}),
  }),
)

export const AuthzCodeRedirectParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      provider: OAuthProviderName,
    }),
    query: z.object({
      clientId: TypedUuidForOAuthClientId,
      redirectUri: z.string(),
      responseType: z.string(),
      scope: z.string().optional(),
      state: z.string(),
    }),
  }),
)

export const AuthzCodeCallbackParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      provider: OAuthProviderName,
    }),
    query: z.object({
      code: z.string().optional(),
      error: z.string().optional(),
      state: z.string().optional(),
    }),
  }),
)

export const AuthzCodeExchangeParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      provider: OAuthProviderName,
    }),
    query: z.object({}),
  }),
)

export const GetDeviceProviderParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      provider: OAuthProviderName,
    }),
    query: z.object({}),
  }),
)

export const ExchangeDeviceTokenParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      provider: OAuthProviderName,
    }),
    query: z.object({}),
  }),
)

export const ListMagicLinksParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const CreateMagicLinkParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const GetMagicLinkParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForMagicLinkId,
    }),
    query: z.object({}),
  }),
)

export const CreateMagicLinkRedirectUriParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForMagicLinkId,
    }),
    query: z.object({}),
  }),
)

export const DeleteMagicLinkRedirectUriParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForMagicLinkId,
      redirectUriId: TypedUuidForMagicLinkRedirectUriId,
    }),
    query: z.object({}),
  }),
)

export const CreateMagicLinkSecretParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForMagicLinkId,
    }),
    query: z.object({}),
  }),
)

export const DeleteMagicLinkSecretParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForMagicLinkId,
      secretId: TypedUuidForMagicLinkSecretId,
    }),
    query: z.object({}),
  }),
)

export const GetMappersParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({
      includeDepleted: SafeBoolean.optional(),
    }),
  }),
)

export const CreateMapperParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const DeleteMapperParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      mapperId: TypedUuidForMapperId,
    }),
    query: z.object({}),
  }),
)

export const ListOauthClientsParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const CreateOauthClientParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const GetOauthClientParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForOAuthClientId,
    }),
    query: z.object({}),
  }),
)

export const CreateOauthClientRedirectUriParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForOAuthClientId,
    }),
    query: z.object({}),
  }),
)

export const DeleteOauthClientRedirectUriParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForOAuthClientId,
      redirectUriId: TypedUuidForOAuthRedirectUriId,
    }),
    query: z.object({}),
  }),
)

export const CreateOauthClientSecretParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForOAuthClientId,
    }),
    query: z.object({}),
  }),
)

export const DeleteOauthClientSecretParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      clientId: TypedUuidForOAuthClientId,
      secretId: TypedUuidForOAuthSecretId,
    }),
    query: z.object({}),
  }),
)

export const ListRfdsParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const ReserveRfdParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)

export const ViewRfdMetaParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const ViewRfdAttrParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      attr: RfdAttrName,
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const SetRfdAttrParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      attr: RfdAttrName,
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const SetRfdContentParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const ViewRfdDiscussionParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const ViewRfdPdfParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const ViewRfdParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const SetRfdDocumentParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const ViewRfdRevisionMetaParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
      revision: TypedUuidForRfdRevisionId,
    }),
    query: z.object({}),
  }),
)

export const ViewRfdRevisionAttrParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      attr: RfdAttrName,
      number: z.string(),
      revision: TypedUuidForRfdRevisionId,
    }),
    query: z.object({}),
  }),
)

export const ViewRfdRevisionDiscussionParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
      revision: TypedUuidForRfdRevisionId,
    }),
    query: z.object({}),
  }),
)

export const ViewRfdRevisionPdfParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
      revision: TypedUuidForRfdRevisionId,
    }),
    query: z.object({}),
  }),
)

export const ViewRfdRevisionParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
      revision: TypedUuidForRfdRevisionId,
    }),
    query: z.object({}),
  }),
)

export const DiscussRfdParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const PublishRfdParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const UpdateRfdVisibilityParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({
      number: z.string(),
    }),
    query: z.object({}),
  }),
)

export const SearchRfdsParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({
      attributesToCrop: z.string().optional(),
      highlightPostTag: z.string().optional(),
      highlightPreTag: z.string().optional(),
      limit: z.number().min(0).max(4294967295).optional(),
      offset: z.number().min(0).max(4294967295).optional(),
      q: z.string(),
    }),
  }),
)

export const GetSelfParams = z.preprocess(
  processResponseBody,
  z.object({
    path: z.object({}),
    query: z.object({}),
  }),
)
