CREATE TABLE api_user_access_token (
  id UUID PRIMARY KEY,
  api_user_id UUID REFERENCES api_user (id) NOT NULL,
  revoked_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
)