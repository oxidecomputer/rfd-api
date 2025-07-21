/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, you can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright Oxide Computer Company
 */

export type RfdScope =
  | 'user:info:r'
  | 'user:info:w'
  | 'user:provider:w'
  | 'user:token:r'
  | 'user:token:w'
  | 'group:info:r'
  | 'mapper:r'
  | 'mapper:w'
  | 'rfd:content:r'
  | 'rfd:discussion:r'
  | 'search'

export type RfdApiProvider = 'email' | 'google' | 'github'

export type RfdAccessToken = {
  iss: string
  aud: string
  sub: string
  prv: string
  scp: string[]
  exp: number
  nbf: number
  jti: string
}

export * from './magic-link'
export * from './oauth'
