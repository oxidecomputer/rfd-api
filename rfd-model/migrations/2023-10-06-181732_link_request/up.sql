CREATE TABLE link_request (
  id UUID PRIMARY KEY,
  source_provider_id UUID NOT NULL,
  source_api_user_id UUID NOT NULL,
  target_api_user_id UUID NOT NULL,
  secret_signature VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  expires_at TIMESTAMPTZ NOT NULL,
  completed_at TIMESTAMPTZ
);