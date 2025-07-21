// /*
//  * This Source Code Form is subject to the terms of the Mozilla Public
//  * License, v. 2.0. If a copy of the MPL was not distributed with this
//  * file, you can obtain one at https://mozilla.org/MPL/2.0/.
//  *
//  * Copyright Oxide Computer Company
//  */

import { Api, ApiResult, GetUserResponse_for_RfdPermission } from '@oxide/rfd.ts/client'
import type { SessionStorage } from '@remix-run/server-runtime'
import { redirect } from '@remix-run/server-runtime'
import { Strategy } from 'remix-auth/strategy'

import { RfdScope } from '.'
import { client } from './util'

export class InvalidMethod extends Error {}
export class MissingRequiredField extends Error {}
export class RemoteError extends Error {}

export type TurnstileMagicLinkStrategyOptions = {
  storage: SessionStorage
  host: string
  clientSecret: string
  channel: string
  linkExpirationTime: number
  pendingPath: string
  returnPath: string
  /**
   * @default "user:info:r"
   */
  scope?: RfdScope[]
}

export type TurnstileMagicLinkVerifyParams = {
  attemptId: string
  email: string
  user: GetUserResponse_for_RfdPermission
  token: string
}

export class TurnstileMagicLinkStrategy<User> extends Strategy<User, TurnstileMagicLinkVerifyParams> {
  public name = 'rfd-magic-link'

  // Session based storage to use for storing the client side authentication materials
  // for tracking the authentication flow
  private readonly storage: SessionStorage

  // Turnstile server to perform authentication against
  private readonly host: string

  // Client secret that will be used to exchange magic link codes for user information
  private readonly clientSecret: string

  // Channel the send the magic link over to
  private readonly channel: string

  // Scopes to request for the user during authentication
  private readonly scope: RfdScope[]

  // Duration for which the magic link will be valid
  private readonly linkExpirationTime: number

  // Path to redirect the user to upon issuing a magic link send
  private readonly pendingPath: string

  // Path that the user should reuturn to via the sent magic link
  private readonly returnPath: string

  private readonly emailField: string = 'email'
  private readonly authnCodeSearchParam: string = 'code'

  private readonly sessionAttemptKey: string = 'auth:v-ml:attempt'
  private readonly sessionEmailKey: string = 'auth:v-ml:email'

  protected verify: Strategy.VerifyFunction<User, TurnstileMagicLinkVerifyParams>

  constructor(
    options: TurnstileMagicLinkStrategyOptions,
    verify: Strategy.VerifyFunction<User, TurnstileMagicLinkVerifyParams>,
  ) {
    super(verify)
    this.verify = verify

    this.storage = options.storage
    this.host = options.host
    this.pendingPath = options.pendingPath
    this.returnPath = options.returnPath
    this.clientSecret = options.clientSecret
    this.channel = options.channel
    this.scope = options.scope ?? ['user:info:r']
    this.linkExpirationTime = options.linkExpirationTime
  }

  public async authenticate(
    request: Request,
  ): Promise<User> {
    if (request.method === 'GET') {
      return await this.handleReturnRequest(request)
    } else if (request.method === 'POST') {
      return await this.handleSendRequest(request)
    } else {
      throw new InvalidMethod(request.method)
    }
  }

  private async handleSendRequest(
    request: Request,
  ): Promise<User> {
    const session = await sessionStorage.getSession(
      request.headers.get('Cookie'),
    )

    // Verify pre-conditions:
    //   1. This must be a POST
    //   2. Request body should contain the FormData including an email field
    //   3. Validate options
    //      a. successRedirect is non-empty
    if (request.method !== 'POST') {
      throw new InvalidMethod(request.method)
    }

    const form = new URLSearchParams(await request.text())
    const email = form.get(this.emailField)

    if (!email) {
      throw new MissingRequiredField('email')
    }

    try {
      // Request email send via rfd API
      const rfd = client(this.host)

      const redirectUri = this.getDomainUrl(request)
      redirectUri.pathname = this.returnPath

      const response = await rfd.methods.magicLinkSend({
        path: { channel: this.channel },
        body: {
          expiresIn: this.linkExpirationTime,
          medium: 'email',
          recipient: email,
          redirectUri: redirectUri.toString(),
          secret: this.clientSecret,
          scope: this.scope.join(' '),
        },
      })

      const attemptId = (await this.handleApiResponse(response)).attemptId

      session.set(this.sessionAttemptKey, attemptId)
      session.set(this.sessionEmailKey, email)
    } catch (err) {
      console.error('Turnstile server failed to send magic link email', err)
      throw new RemoteError('Failed to send magic link email')
    }

    const cookies = await sessionStorage.commitSession(session)
    throw redirect(this.returnPath, {
      headers: {
        'Set-Cookie': cookies,
      },
    })
  }

  private async handleReturnRequest(
    request: Request,
  ): Promise<User> {
    const session = await sessionStorage.getSession(
      request.headers.get('Cookie'),
    )

    // Verify pre-conditions:
    //   1. This must be a GET
    //   2. Request query should contain and authn code
    //   3. Session key is set
    //   4. Session attempt exists
    //   5. Session email exists
    if (request.method !== 'GET') {
      throw new InvalidMethod(request.method)
    }

    const code = new URL(request.url).searchParams.get(this.authnCodeSearchParam)
    if (!code) {
      throw new Error('Missing code parameter')
    }

    const attemptId = session.get(this.sessionAttemptKey)
    if (!attemptId) {
      throw new Error(
        'Missing attemptId in session. This link may not have been intended for the current browser',
      )
    }

    const email = session.get(this.sessionEmailKey)
    if (!email) {
      throw new Error('Missing email in session. This link may not have been intended for the current browser')
    }

    let token
    let expiresIn
    try {
      // Send exchange request to rfd with authn code and secret to retrieve
      // an access token
      const rfd = client(this.host)
      const response = await this.handleApiResponse(
        await rfd.methods.magicLinkExchange({
          path: { channel: this.channel },
          body: {
            attemptId,
            recipient: email,
            secret: code,
          },
        }),
      )
      token = response.accessToken
      expiresIn = response.expiresIn

      // Once the exchange has completed, clear the attempt key
      session.unset(this.sessionAttemptKey)
      session.unset(this.sessionEmailKey)
    } catch (err) {
      console.error('Failed to exchange authentication code of user credentials', err)
      throw new RemoteError('Failed to exchange authentication code of user credentials')
    }

    let user
    try {
      // Crete a new client with the token so that authenticated calls can be made
      const rfd = client(this.host, token)
      const apiUser = await this.handleApiResponse(await rfd.methods.getSelf({}, {}))

      user = await this.verify({ attemptId, email, user: apiUser, token })
    } catch (err) {
      console.error('Failed to retrieve user data', err)
      throw new RemoteError('Failed to retrieve user data')
    }

    return user
  }

  private getDomainUrl(request: Request): URL {
    const host = request.headers.get('X-Forwarded-Host') ?? request.headers.get('host')

    if (!host) {
      throw new Error('Could not determine domain URL.')
    }

    const protocol = host.includes('localhost') || host.includes('127.0.0.1')
      ? 'http'
      : request.headers.get('X-Forwarded-Proto') ?? 'https'

    return new URL(`${protocol}://${host}`)
  }

  private async handleApiResponse<T>(response: ApiResult<T>): Promise<T> {
    if (response.type === 'success') {
      return response.data
    } else if (response.type === 'client_error') {
      console.error('Failed attempting to send request to rfd server', response)
      throw response.error as Error
    } else {
      console.error('Failed attempting to send request to rfd server', response)
      throw new Error(response.data.message)
    }
  }
}
