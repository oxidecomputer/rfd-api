CREATE TYPE ATTEMPT_STATE as ENUM('new', 'remote_authenticated', 'failed', 'complete');

CREATE TABLE login_attempt(
  id UUID PRIMARY KEY,
  attempt_state ATTEMPT_STATE NOT NULL,

  client_id UUID NOT NULL,
  redirect_uri VARCHAR NOT NULL,
  state VARCHAR,
  pkce_challenge VARCHAR,
  pkce_challenge_method VARCHAR,
  authz_code VARCHAR,
  expires_at TIMESTAMPTZ,

  provider VARCHAR NOT NULL,
  provider_state VARCHAR NOT NULL UNIQUE,
  provider_pkce_verifier VARCHAR NOT NULL,
  provider_authz_code VARCHAR,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)
