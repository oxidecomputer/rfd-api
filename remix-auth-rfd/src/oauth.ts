/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, you can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright Oxide Computer Company
 */

import Api, { GetUserResponse_for_RfdPermission } from '@oxide/rfd.ts/client'
import { redirect, type SessionData, type SessionStorage } from 'react-router'
import { OAuth2Strategy } from 'remix-auth-oauth2'
import { Strategy } from 'remix-auth/strategy'
import { RfdApiProvider, RfdScope } from '.'
import { client, handleApiResponse } from './util'

export type RfdOAuthStrategyOptions = {
  host: string
  clientId: string
  clientSecret: string
  redirectURI: string
  remoteProvider: RfdApiProvider
  /**
   * @default "user:info:r"
   */
  scopes?: RfdScope[]
  /**
   * Custom host for the authorization endpoint. Overrides `host` when
   * constructing the authorization URL.
   * @default host
   */
  authorizationHost?: string
  /**
   * Custom host for the token endpoint. Overrides `host` when
   * constructing the token URL.
   * @default host
   */
  tokenHost?: string
}

export type ExpiringUser = {
  expiresAt: Date
}

export type RfdVerifyCallback<T> = Strategy.VerifyFunction<T, OAuth2Strategy.VerifyOptions>

export class RfdOAuthStrategy<User extends ExpiringUser> extends OAuth2Strategy<
  User
> {
  public name = `rfd`
  protected readonly userInfoUrl
  protected readonly host

  constructor(
    {
      host,
      clientId,
      clientSecret,
      redirectURI,
      remoteProvider,
      scopes,
      authorizationHost,
      tokenHost,
    }: RfdOAuthStrategyOptions,
    verify: RfdVerifyCallback<User>,
  ) {
    super(
      {
        clientId,
        clientSecret,
        redirectURI,
        authorizationEndpoint: `${authorizationHost ?? host}/login/oauth/${remoteProvider}/code/authorize`,
        tokenEndpoint: `${tokenHost ?? host}/login/oauth/${remoteProvider}/code/token`,
        scopes: scopes ?? ['user:info:r'],
      },
      verify,
    )
    this.name = `${this.name}-${remoteProvider}`
    this.host = host
    this.userInfoUrl = `${host}/self`
  }
}
