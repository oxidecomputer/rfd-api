CREATE TABLE mapper (
  id UUID PRIMARY KEY,
  name VARCHAR NOT NULL UNIQUE,
  rule JSONB NOT NULL,
  activations INTEGER,
  max_activations INTEGER,
  depleted_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,
  CHECK (activations <= max_activations)
);