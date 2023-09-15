CREATE TABLE oauth_client(
  id UUID PRIMARY KEY,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE TABLE oauth_client_secret(
  id UUID PRIMARY KEY,
  oauth_client_id UUID REFERENCES oauth_client (id) NOT NULL,
  secret_signature VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE TABLE oauth_client_redirect_uri(
  id UUID PRIMARY KEY,
  oauth_client_id UUID REFERENCES oauth_client (id) NOT NULL,
  redirect_uri VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);