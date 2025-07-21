import Api, { ApiResult } from '@oxide/rfd.ts/client'
import { ApiWithRetry } from '@oxide/rfd.ts/client-retry'

const retryOnReset = () => {
  const limit = 1
  let retries = 0
  return (err: any) => {
    if (retries < limit && err.type === 'system' && err.errno === 'ECONNRESET' && err.code === 'ECONNRESET') {
      retries += 1
      return true
    } else {
      return false
    }
  }
}

export function client(host: string, token?: string): Api {
  return new ApiWithRetry({ host, token, retryHandler: retryOnReset })
}

export function handleApiResponse<T>(response: ApiResult<T>): T {
  if (response.type === 'success') {
    return response.data
  } else if (response.type === 'client_error') {
    console.error('Failed attempting to send request to RFD server', response)
    throw response.error as Error
  } else {
    console.error('Failed attempting to send request to RFD server', response)
    throw new Error(response.data.message)
  }
}
