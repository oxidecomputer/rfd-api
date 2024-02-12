CREATE TABLE api_user (
  id UUID PRIMARY KEY,
  permissions JSONB NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE TABLE api_key (
  id UUID PRIMARY KEY,
  api_user_id UUID REFERENCES api_user (id) NOT NULL,
  key_signature TEXT NOT NULL UNIQUE,
  permissions JSONB NOT NULL,
  expires_at TIMESTAMPTZ NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE TABLE api_user_provider (
  id UUID PRIMARY KEY,
  api_user_id UUID REFERENCES api_user (id) NOT NULL,
  provider VARCHAR NOT NULL,
  provider_id VARCHAR NOT NULL,
  emails TEXT[] NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX api_user_provider_idx ON api_user_provider (provider, provider_id);
