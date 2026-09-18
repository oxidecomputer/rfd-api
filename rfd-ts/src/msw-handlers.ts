import { http, type HttpHandler, HttpResponse, type PathParams, type StrictResponse } from 'msw'
import type { Promisable, SnakeCasedPropertiesDeep as Snakify } from 'type-fest'
import { type ZodType } from 'zod/v4'
import type * as Api from './Api'
import { snakeify } from './util'
import * as schema from './validate'

type HandlerResult<T> = Json<T> | StrictResponse<Json<T>>
type StatusCode = number

// these are used for turning our nice JS-ified API types back into the original
// API JSON types (snake cased and dates as strings) for use in our mock API

type StringifyDates<T> = T extends Date ? string
  : {
    [K in keyof T]: T[K] extends Array<infer U> ? Array<StringifyDates<U>>
      : StringifyDates<T[K]>
  }

/**
 * Snake case fields and convert dates to strings. Not intended to be a general
 * purpose JSON type!
 */
export type Json<B> = Snakify<StringifyDates<B>>
export const json = HttpResponse.json

// Shortcut to reduce number of imports required in consumers
export { HttpResponse }

export interface MSWHandlers {
  /** `GET /.well-known/jwks.json` */
  jwksJson: (params: { req: Request; cookies: Record<string, string> }) => Promisable<HandlerResult<Api.Jwks>>
  /** `GET /.well-known/openid-configuration` */
  openidConfiguration: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.OpenIdConfiguration>>
  /** `GET /api-user` */
  listApiUsers: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission[]>>
  /** `POST /api-user` */
  createApiUser: (
    params: { body: Json<Api.ApiUserUpdateParams_for_RfdPermission>; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission>>
  /** `GET /api-user/:userId` */
  getApiUser: (
    params: { path: Api.GetApiUserPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission>>
  /** `POST /api-user/:userId` */
  updateApiUser: (
    params: {
      path: Api.UpdateApiUserPathParams
      body: Json<Api.ApiUserUpdateParams_for_RfdPermission>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission>>
  /** `PUT /api-user/:userId/contact/email` */
  setApiUserContactEmail: (
    params: {
      path: Api.SetApiUserContactEmailPathParams
      body: Json<Api.ApiUserEmailUpdateParams>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.ApiUserContactEmail>>
  /** `POST /api-user/:userId/group` */
  addApiUserToGroup: (
    params: {
      path: Api.AddApiUserToGroupPathParams
      body: Json<Api.AddGroupBody>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission>>
  /** `DELETE /api-user/:userId/group/:groupId` */
  removeApiUserFromGroup: (
    params: { path: Api.RemoveApiUserFromGroupPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission>>
  /** `POST /api-user/:userId/link` */
  linkProvider: (
    params: {
      path: Api.LinkProviderPathParams
      body: Json<Api.ApiUserProviderLinkPayload>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<StatusCode>
  /** `POST /api-user/:userId/permission` */
  addApiUserPermission: (
    params: {
      path: Api.AddApiUserPermissionPathParams
      body: Json<Api.ApiUserPermissionParams_for_RfdPermission>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission>>
  /** `DELETE /api-user/:userId/permission` */
  removeApiUserPermission: (
    params: { path: Api.RemoveApiUserPermissionPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission>>
  /** `GET /api-user/:userId/token` */
  listApiUserTokens: (
    params: { path: Api.ListApiUserTokensPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.ApiKeyResponse_for_RfdPermission[]>>
  /** `POST /api-user/:userId/token` */
  createApiUserToken: (
    params: {
      path: Api.CreateApiUserTokenPathParams
      body: Json<Api.ApiKeyCreateParams_for_RfdPermission>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.InitialApiKeyResponse_for_RfdPermission>>
  /** `GET /api-user/:userId/token/:apiKeyId` */
  getApiUserToken: (
    params: { path: Api.GetApiUserTokenPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.ApiKeyResponse_for_RfdPermission>>
  /** `DELETE /api-user/:userId/token/:apiKeyId` */
  deleteApiUserToken: (
    params: { path: Api.DeleteApiUserTokenPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.ApiKeyResponse_for_RfdPermission>>
  /** `POST /api-user-provider/:providerId/link-token` */
  createLinkToken: (
    params: {
      path: Api.CreateLinkTokenPathParams
      body: Json<Api.ApiUserLinkRequestPayload>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.ApiUserLinkRequestResponse>>
  /** `POST /github` */
  githubWebhook: (
    params: { body: Json<Api.GitHubCommitPayload>; req: Request; cookies: Record<string, string> },
  ) => Promisable<StatusCode>
  /** `GET /group` */
  getGroups: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.AccessGroup_for_RfdPermission[]>>
  /** `POST /group` */
  createGroup: (
    params: {
      body: Json<Api.AccessGroupUpdateParams_for_RfdPermission>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.AccessGroup_for_RfdPermission>>
  /** `PUT /group/:groupId` */
  updateGroup: (
    params: {
      path: Api.UpdateGroupPathParams
      body: Json<Api.AccessGroupUpdateParams_for_RfdPermission>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.AccessGroup_for_RfdPermission>>
  /** `DELETE /group/:groupId` */
  deleteGroup: (
    params: { path: Api.DeleteGroupPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.AccessGroup_for_RfdPermission>>
  /** `GET /group-membership/:groupId` */
  getGroupMembers: (
    params: { path: Api.GetGroupMembersPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission[]>>
  /** `GET /job` */
  listJobs: (
    params: { query: Api.ListJobsQueryParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.Job[]>>
  /** `POST /login/magic/:channel/exchange` */
  magicLinkExchange: (
    params: {
      path: Api.MagicLinkExchangePathParams
      body: Json<Api.MagicLinkExchangeRequest>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.MagicLinkExchangeResponse>>
  /** `POST /login/magic/:channel/send` */
  magicLinkSend: (
    params: {
      path: Api.MagicLinkSendPathParams
      body: Json<Api.MagicLinkSendRequest>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.MagicLinkSendResponse>>
  /** `GET /login/oauth/:provider/code/authorize` */
  authzCodeRedirect: (
    params: {
      path: Api.AuthzCodeRedirectPathParams
      query: Api.AuthzCodeRedirectQueryParams
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<StatusCode>
  /** `GET /login/oauth/:provider/code/callback` */
  authzCodeCallback: (
    params: {
      path: Api.AuthzCodeCallbackPathParams
      query: Api.AuthzCodeCallbackQueryParams
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<StatusCode>
  /** `POST /login/oauth/:provider/code/token` */
  authzCodeExchange: (
    params: {
      path: Api.AuthzCodeExchangePathParams
      query: Api.AuthzCodeExchangeQueryParams
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.OAuthAuthzCodeExchangeResponse>>
  /** `GET /login/oauth/:provider/device` */
  getDeviceProvider: (
    params: { path: Api.GetDeviceProviderPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.OAuthProviderDeviceInfo>>
  /** `POST /login/oauth/:provider/device` */
  deviceAuthz: (
    params: {
      path: Api.DeviceAuthzPathParams
      body: Json<Api.DeviceAuthorizationRequest>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<StatusCode>
  /** `POST /login/oauth/:provider/device/exchange` */
  exchangeDeviceToken: (
    params: { path: Api.ExchangeDeviceTokenPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<StatusCode>
  /** `GET /login/oauth/:provider/public-pkce` */
  getWebPkceProvider: (
    params: { path: Api.GetWebPkceProviderPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.OAuthProviderAuthorizationCodePkceInfo>>
  /** `GET /magic/client` */
  listMagicLinks: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.MagicLink[]>>
  /** `POST /magic/client` */
  createMagicLink: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.MagicLink>>
  /** `GET /magic/client/:clientId` */
  getMagicLink: (
    params: { path: Api.GetMagicLinkPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.MagicLink>>
  /** `POST /magic/client/:clientId/redirect_uri` */
  createMagicLinkRedirectUri: (
    params: {
      path: Api.CreateMagicLinkRedirectUriPathParams
      body: Json<Api.AddMagicLinkRedirectBody>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.MagicLinkRedirectUri>>
  /** `DELETE /magic/client/:clientId/redirect_uri/:redirectUriId` */
  deleteMagicLinkRedirectUri: (
    params: { path: Api.DeleteMagicLinkRedirectUriPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.MagicLinkRedirectUri>>
  /** `POST /magic/client/:clientId/secret` */
  createMagicLinkSecret: (
    params: { path: Api.CreateMagicLinkSecretPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.InitialMagicLinkSecretResponse>>
  /** `DELETE /magic/client/:clientId/secret/:secretId` */
  deleteMagicLinkSecret: (
    params: { path: Api.DeleteMagicLinkSecretPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.MagicLinkSecret>>
  /** `GET /mapper` */
  getMappers: (
    params: { query: Api.GetMappersQueryParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.Mapper[]>>
  /** `POST /mapper` */
  createMapper: (
    params: { body: Json<Api.CreateMapper>; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.Mapper>>
  /** `DELETE /mapper/:mapperId` */
  deleteMapper: (
    params: { path: Api.DeleteMapperPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.Mapper>>
  /** `GET /oauth/client` */
  listOauthClients: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.OAuthClient[]>>
  /** `POST /oauth/client` */
  createOauthClient: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.OAuthClient>>
  /** `GET /oauth/client/:clientId` */
  getOauthClient: (
    params: { path: Api.GetOauthClientPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.OAuthClient>>
  /** `POST /oauth/client/:clientId/redirect_uri` */
  createOauthClientRedirectUri: (
    params: {
      path: Api.CreateOauthClientRedirectUriPathParams
      body: Json<Api.AddOAuthClientRedirectBody>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.OAuthClientRedirectUri>>
  /** `DELETE /oauth/client/:clientId/redirect_uri/:redirectUriId` */
  deleteOauthClientRedirectUri: (
    params: { path: Api.DeleteOauthClientRedirectUriPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.OAuthClientRedirectUri>>
  /** `POST /oauth/client/:clientId/secret` */
  createOauthClientSecret: (
    params: { path: Api.CreateOauthClientSecretPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.InitialOAuthClientSecretResponse>>
  /** `DELETE /oauth/client/:clientId/secret/:secretId` */
  deleteOauthClientSecret: (
    params: { path: Api.DeleteOauthClientSecretPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.OAuthClientSecret>>
  /** `GET /rfd` */
  listRfds: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdWithoutContent[]>>
  /** `POST /rfd` */
  reserveRfd: (
    params: { body: Json<Api.ReserveRfdBody>; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.ReserveRfdResponse>>
  /** `GET /rfd/:number` */
  viewRfdMeta: (
    params: { path: Api.ViewRfdMetaPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdWithoutContent>>
  /** `GET /rfd/:number/attr/:attr` */
  viewRfdAttr: (
    params: { path: Api.ViewRfdAttrPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdAttr>>
  /** `POST /rfd/:number/attr/:attr` */
  setRfdAttr: (
    params: {
      path: Api.SetRfdAttrPathParams
      body: Json<Api.RfdAttrValue>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.RfdAttr>>
  /** `POST /rfd/:number/content` */
  setRfdContent: (
    params: {
      path: Api.SetRfdContentPathParams
      body: Json<Api.RfdUpdateContentBody>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<StatusCode>
  /** `GET /rfd/:number/discussion` */
  viewRfdDiscussion: (
    params: { path: Api.ViewRfdDiscussionPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<StatusCode>
  /** `GET /rfd/:number/pdf` */
  viewRfdPdf: (
    params: { path: Api.ViewRfdPdfPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdWithPdf>>
  /** `GET /rfd/:number/raw` */
  viewRfd: (
    params: { path: Api.ViewRfdPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdWithRaw>>
  /** `POST /rfd/:number/raw` */
  setRfdDocument: (
    params: {
      path: Api.SetRfdDocumentPathParams
      body: Json<Api.RfdUpdateBody>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<StatusCode>
  /** `GET /rfd/:number/revision` */
  listRfdRevisions: (
    params: {
      path: Api.ListRfdRevisionsPathParams
      query: Api.ListRfdRevisionsQueryParams
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.RfdRevisionMeta[]>>
  /** `GET /rfd/:number/revision/:revision` */
  viewRfdRevisionMeta: (
    params: { path: Api.ViewRfdRevisionMetaPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdWithoutContent>>
  /** `PATCH /rfd/:number/revision/:revision` */
  updateRfdRevision: (
    params: { path: Api.UpdateRfdRevisionPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdRevisionMeta>>
  /** `GET /rfd/:number/revision/:revision/attr/:attr` */
  viewRfdRevisionAttr: (
    params: { path: Api.ViewRfdRevisionAttrPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdAttr>>
  /** `GET /rfd/:number/revision/:revision/discussion` */
  viewRfdRevisionDiscussion: (
    params: { path: Api.ViewRfdRevisionDiscussionPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<StatusCode>
  /** `GET /rfd/:number/revision/:revision/pdf` */
  viewRfdRevisionPdf: (
    params: { path: Api.ViewRfdRevisionPdfPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdWithPdf>>
  /** `GET /rfd/:number/revision/:revision/raw` */
  viewRfdRevision: (
    params: { path: Api.ViewRfdRevisionPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdWithRaw>>
  /** `POST /rfd/:number/state/discuss` */
  discussRfd: (
    params: { path: Api.DiscussRfdPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdAttr>>
  /** `POST /rfd/:number/state/publish` */
  publishRfd: (
    params: { path: Api.PublishRfdPathParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.RfdAttr>>
  /** `POST /rfd/:number/visibility` */
  updateRfdVisibility: (
    params: {
      path: Api.UpdateRfdVisibilityPathParams
      body: Json<Api.RfdVisibility>
      req: Request
      cookies: Record<string, string>
    },
  ) => Promisable<HandlerResult<Api.Rfd>>
  /** `GET /rfd-search` */
  searchRfds: (
    params: { query: Api.SearchRfdsQueryParams; req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.SearchResults>>
  /** `GET /self` */
  getSelf: (
    params: { req: Request; cookies: Record<string, string> },
  ) => Promisable<HandlerResult<Api.GetUserResponse_for_RfdPermission>>
}

function validateParams<S extends ZodType>(schema: S, req: Request, pathParams: PathParams) {
  const rawParams = new URLSearchParams(new URL(req.url).search)
  const params: [string, unknown][] = []

  // Ensure numeric params like `limit` are parsed as numbers
  for (const [name, value] of rawParams) {
    params.push([name, isNaN(Number(value)) ? value : Number(value)])
  }

  const result = schema.safeParse({
    path: pathParams,
    query: Object.fromEntries(params),
  })

  if (result.success) {
    return { params: result.data }
  }

  // if any of the errors come from path params, just 404 — the resource cannot
  // exist if there's no valid name
  const status = result.error.issues.some((e) => e.path[0] === 'path') ? 404 : 400
  const error_code = status === 404 ? 'NotFound' : 'InvalidRequest'
  const message = 'Zod error for params: ' + JSON.stringify(result.error)
  return { paramsErr: json({ error_code, message }, { status }) }
}

const handler = (
  handler: MSWHandlers[keyof MSWHandlers],
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  paramSchema: ZodType<any> | null,
  bodySchema: ZodType | null,
) =>
async ({
  request: req,
  params: pathParams,
  cookies,
}: {
  request: Request
  params: PathParams
  cookies: Record<string, string | string[]>
}) => {
  const { params, paramsErr } = paramSchema
    ? validateParams(paramSchema, req, pathParams)
    : { params: {}, paramsErr: undefined }
  if (paramsErr) { return paramsErr }

  const { path, query } = params

  let body = undefined
  if (bodySchema) {
    const rawBody = await req.json()
    const result = bodySchema.transform(snakeify).safeParse(rawBody)
    if (!result.success) {
      const message = 'Zod error for body: ' + JSON.stringify(result.error)
      return json({ error_code: 'InvalidRequest', message }, { status: 400 })
    }
    body = result.data
  }

  try {
    // TypeScript can't narrow the handler down because there's not an explicit relationship between the schema
    // being present and the shape of the handler API. The type of this function could be resolved such that the
    // relevant schema is required if and only if the handler has a type that matches the inferred schema
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const result = await (handler as any).apply(null, [{ path, query, body, req, cookies }])
    if (typeof result === 'number') {
      return new HttpResponse(null, { status: result })
    }
    if (result instanceof Response) {
      return result
    }
    return json(result)
  } catch (thrown) {
    if (typeof thrown === 'number') {
      return new HttpResponse(null, { status: thrown })
    }
    if (typeof thrown === 'string') {
      return json({ message: thrown }, { status: 400 })
    }
    if (thrown instanceof Response) {
      return thrown
    }

    // if it's not one of those, then we don't know what to do with it
    console.error('Unexpected mock error', thrown)
    if (typeof thrown === 'function') {
      console.error(
        "It looks like you've accidentally thrown an error constructor function from a mock handler without calling it!",
      )
    }
    // rethrow so everything breaks because this isn't supposed to happen
    throw thrown
  }
}

export function makeHandlers(
  handlers: MSWHandlers,
): HttpHandler[] {
  return [
    http.get('/.well-known/jwks.json', handler(handlers['jwksJson'], null, null)),
    http.get('/.well-known/openid-configuration', handler(handlers['openidConfiguration'], null, null)),
    http.get('/api-user', handler(handlers['listApiUsers'], null, null)),
    http.post('/api-user', handler(handlers['createApiUser'], null, schema.ApiUserUpdateParams_for_RfdPermission)),
    http.get('/api-user/:userId', handler(handlers['getApiUser'], schema.GetApiUserParams, null)),
    http.post(
      '/api-user/:userId',
      handler(handlers['updateApiUser'], schema.UpdateApiUserParams, schema.ApiUserUpdateParams_for_RfdPermission),
    ),
    http.put(
      '/api-user/:userId/contact/email',
      handler(handlers['setApiUserContactEmail'], schema.SetApiUserContactEmailParams, schema.ApiUserEmailUpdateParams),
    ),
    http.post(
      '/api-user/:userId/group',
      handler(handlers['addApiUserToGroup'], schema.AddApiUserToGroupParams, schema.AddGroupBody),
    ),
    http.delete(
      '/api-user/:userId/group/:groupId',
      handler(handlers['removeApiUserFromGroup'], schema.RemoveApiUserFromGroupParams, null),
    ),
    http.post(
      '/api-user/:userId/link',
      handler(handlers['linkProvider'], schema.LinkProviderParams, schema.ApiUserProviderLinkPayload),
    ),
    http.post(
      '/api-user/:userId/permission',
      handler(
        handlers['addApiUserPermission'],
        schema.AddApiUserPermissionParams,
        schema.ApiUserPermissionParams_for_RfdPermission,
      ),
    ),
    http.delete(
      '/api-user/:userId/permission',
      handler(handlers['removeApiUserPermission'], schema.RemoveApiUserPermissionParams, null),
    ),
    http.get('/api-user/:userId/token', handler(handlers['listApiUserTokens'], schema.ListApiUserTokensParams, null)),
    http.post(
      '/api-user/:userId/token',
      handler(
        handlers['createApiUserToken'],
        schema.CreateApiUserTokenParams,
        schema.ApiKeyCreateParams_for_RfdPermission,
      ),
    ),
    http.get(
      '/api-user/:userId/token/:apiKeyId',
      handler(handlers['getApiUserToken'], schema.GetApiUserTokenParams, null),
    ),
    http.delete(
      '/api-user/:userId/token/:apiKeyId',
      handler(handlers['deleteApiUserToken'], schema.DeleteApiUserTokenParams, null),
    ),
    http.post(
      '/api-user-provider/:providerId/link-token',
      handler(handlers['createLinkToken'], schema.CreateLinkTokenParams, schema.ApiUserLinkRequestPayload),
    ),
    http.post('/github', handler(handlers['githubWebhook'], null, schema.GitHubCommitPayload)),
    http.get('/group', handler(handlers['getGroups'], null, null)),
    http.post('/group', handler(handlers['createGroup'], null, schema.AccessGroupUpdateParams_for_RfdPermission)),
    http.put(
      '/group/:groupId',
      handler(handlers['updateGroup'], schema.UpdateGroupParams, schema.AccessGroupUpdateParams_for_RfdPermission),
    ),
    http.delete('/group/:groupId', handler(handlers['deleteGroup'], schema.DeleteGroupParams, null)),
    http.get('/group-membership/:groupId', handler(handlers['getGroupMembers'], schema.GetGroupMembersParams, null)),
    http.get('/job', handler(handlers['listJobs'], schema.ListJobsParams, null)),
    http.post(
      '/login/magic/:channel/exchange',
      handler(handlers['magicLinkExchange'], schema.MagicLinkExchangeParams, schema.MagicLinkExchangeRequest),
    ),
    http.post(
      '/login/magic/:channel/send',
      handler(handlers['magicLinkSend'], schema.MagicLinkSendParams, schema.MagicLinkSendRequest),
    ),
    http.get(
      '/login/oauth/:provider/code/authorize',
      handler(handlers['authzCodeRedirect'], schema.AuthzCodeRedirectParams, null),
    ),
    http.get(
      '/login/oauth/:provider/code/callback',
      handler(handlers['authzCodeCallback'], schema.AuthzCodeCallbackParams, null),
    ),
    http.post(
      '/login/oauth/:provider/code/token',
      handler(handlers['authzCodeExchange'], schema.AuthzCodeExchangeParams, null),
    ),
    http.get(
      '/login/oauth/:provider/device',
      handler(handlers['getDeviceProvider'], schema.GetDeviceProviderParams, null),
    ),
    http.post(
      '/login/oauth/:provider/device',
      handler(handlers['deviceAuthz'], schema.DeviceAuthzParams, schema.DeviceAuthorizationRequest),
    ),
    http.post(
      '/login/oauth/:provider/device/exchange',
      handler(handlers['exchangeDeviceToken'], schema.ExchangeDeviceTokenParams, null),
    ),
    http.get(
      '/login/oauth/:provider/public-pkce',
      handler(handlers['getWebPkceProvider'], schema.GetWebPkceProviderParams, null),
    ),
    http.get('/magic/client', handler(handlers['listMagicLinks'], null, null)),
    http.post('/magic/client', handler(handlers['createMagicLink'], null, null)),
    http.get('/magic/client/:clientId', handler(handlers['getMagicLink'], schema.GetMagicLinkParams, null)),
    http.post(
      '/magic/client/:clientId/redirect_uri',
      handler(
        handlers['createMagicLinkRedirectUri'],
        schema.CreateMagicLinkRedirectUriParams,
        schema.AddMagicLinkRedirectBody,
      ),
    ),
    http.delete(
      '/magic/client/:clientId/redirect_uri/:redirectUriId',
      handler(handlers['deleteMagicLinkRedirectUri'], schema.DeleteMagicLinkRedirectUriParams, null),
    ),
    http.post(
      '/magic/client/:clientId/secret',
      handler(handlers['createMagicLinkSecret'], schema.CreateMagicLinkSecretParams, null),
    ),
    http.delete(
      '/magic/client/:clientId/secret/:secretId',
      handler(handlers['deleteMagicLinkSecret'], schema.DeleteMagicLinkSecretParams, null),
    ),
    http.get('/mapper', handler(handlers['getMappers'], schema.GetMappersParams, null)),
    http.post('/mapper', handler(handlers['createMapper'], null, schema.CreateMapper)),
    http.delete('/mapper/:mapperId', handler(handlers['deleteMapper'], schema.DeleteMapperParams, null)),
    http.get('/oauth/client', handler(handlers['listOauthClients'], null, null)),
    http.post('/oauth/client', handler(handlers['createOauthClient'], null, null)),
    http.get('/oauth/client/:clientId', handler(handlers['getOauthClient'], schema.GetOauthClientParams, null)),
    http.post(
      '/oauth/client/:clientId/redirect_uri',
      handler(
        handlers['createOauthClientRedirectUri'],
        schema.CreateOauthClientRedirectUriParams,
        schema.AddOAuthClientRedirectBody,
      ),
    ),
    http.delete(
      '/oauth/client/:clientId/redirect_uri/:redirectUriId',
      handler(handlers['deleteOauthClientRedirectUri'], schema.DeleteOauthClientRedirectUriParams, null),
    ),
    http.post(
      '/oauth/client/:clientId/secret',
      handler(handlers['createOauthClientSecret'], schema.CreateOauthClientSecretParams, null),
    ),
    http.delete(
      '/oauth/client/:clientId/secret/:secretId',
      handler(handlers['deleteOauthClientSecret'], schema.DeleteOauthClientSecretParams, null),
    ),
    http.get('/rfd', handler(handlers['listRfds'], null, null)),
    http.post('/rfd', handler(handlers['reserveRfd'], null, schema.ReserveRfdBody)),
    http.get('/rfd/:number', handler(handlers['viewRfdMeta'], schema.ViewRfdMetaParams, null)),
    http.get('/rfd/:number/attr/:attr', handler(handlers['viewRfdAttr'], schema.ViewRfdAttrParams, null)),
    http.post('/rfd/:number/attr/:attr', handler(handlers['setRfdAttr'], schema.SetRfdAttrParams, schema.RfdAttrValue)),
    http.post(
      '/rfd/:number/content',
      handler(handlers['setRfdContent'], schema.SetRfdContentParams, schema.RfdUpdateContentBody),
    ),
    http.get('/rfd/:number/discussion', handler(handlers['viewRfdDiscussion'], schema.ViewRfdDiscussionParams, null)),
    http.get('/rfd/:number/pdf', handler(handlers['viewRfdPdf'], schema.ViewRfdPdfParams, null)),
    http.get('/rfd/:number/raw', handler(handlers['viewRfd'], schema.ViewRfdParams, null)),
    http.post(
      '/rfd/:number/raw',
      handler(handlers['setRfdDocument'], schema.SetRfdDocumentParams, schema.RfdUpdateBody),
    ),
    http.get('/rfd/:number/revision', handler(handlers['listRfdRevisions'], schema.ListRfdRevisionsParams, null)),
    http.get(
      '/rfd/:number/revision/:revision',
      handler(handlers['viewRfdRevisionMeta'], schema.ViewRfdRevisionMetaParams, null),
    ),
    http.patch(
      '/rfd/:number/revision/:revision',
      handler(handlers['updateRfdRevision'], schema.UpdateRfdRevisionParams, null),
    ),
    http.get(
      '/rfd/:number/revision/:revision/attr/:attr',
      handler(handlers['viewRfdRevisionAttr'], schema.ViewRfdRevisionAttrParams, null),
    ),
    http.get(
      '/rfd/:number/revision/:revision/discussion',
      handler(handlers['viewRfdRevisionDiscussion'], schema.ViewRfdRevisionDiscussionParams, null),
    ),
    http.get(
      '/rfd/:number/revision/:revision/pdf',
      handler(handlers['viewRfdRevisionPdf'], schema.ViewRfdRevisionPdfParams, null),
    ),
    http.get(
      '/rfd/:number/revision/:revision/raw',
      handler(handlers['viewRfdRevision'], schema.ViewRfdRevisionParams, null),
    ),
    http.post('/rfd/:number/state/discuss', handler(handlers['discussRfd'], schema.DiscussRfdParams, null)),
    http.post('/rfd/:number/state/publish', handler(handlers['publishRfd'], schema.PublishRfdParams, null)),
    http.post(
      '/rfd/:number/visibility',
      handler(handlers['updateRfdVisibility'], schema.UpdateRfdVisibilityParams, schema.RfdVisibility),
    ),
    http.get('/rfd-search', handler(handlers['searchRfds'], schema.SearchRfdsParams, null)),
    http.get('/self', handler(handlers['getSelf'], null, null)),
  ]
}
