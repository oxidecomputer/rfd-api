import Api from './Api'
import { ApiConfig, ApiResult, FullParams, handleResponse, mergeParams, toQueryString } from './http-client'
import { snakeify } from './util'

export type RetryHandler = (err: any) => boolean
export type RetryHandlerFactory = (url: RequestInfo | URL, init: RequestInit) => RetryHandler

export type ApiConfigWithRetry = ApiConfig & { retryHandler?: RetryHandlerFactory }

export class ApiWithRetry extends Api {
  retryHandler: RetryHandlerFactory

  constructor({ host = '', baseParams = {}, token, retryHandler }: ApiConfigWithRetry = {}) {
    super({ host, baseParams, token })
    this.retryHandler = retryHandler ? retryHandler : () => () => false
  }

  public async request<Data>({
    body,
    path,
    query,
    host,
    ...fetchParams
  }: FullParams): Promise<ApiResult<Data>> {
    const url = (host || this.host) + path + toQueryString(query)
    const init = {
      ...mergeParams(this.baseParams, fetchParams),
      body: JSON.stringify(snakeify(body), replacer),
    }
    return fetchWithRetry(fetch, url, init, this.retryHandler(url, init))
  }
}

export async function fetchWithRetry<Data>(
  fetch: any,
  url: string,
  init: RequestInit,
  retry: RetryHandler,
): Promise<ApiResult<Data>> {
  try {
    return handleResponse(await fetch(url, init))
  } catch (err) {
    if (retry(err)) {
      return await fetchWithRetry(fetch, url, init, retry)
    }

    throw err
  }
}

/**
 * Convert `Date` to ISO string. Leave other values alone. Used for both request
 * body and query params.
 */
// eslint-disable-next-line @typescript-eslint/no-explicit-any
function replacer(_key: string, value: any) {
  if (value instanceof Date) {
    return value.toISOString()
  }
  return value
}
