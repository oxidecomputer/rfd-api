CREATE TABLE job (
  id SERIAL PRIMARY KEY,
  owner VARCHAR NOT NULL,
  repository VARCHAR NOT NULL,
  branch VARCHAR NOT NULL,
  sha VARCHAR NOT NULL,
  rfd INTEGER NOT NULL,
  webhook_delivery_id UUID,
  committed_at TIMESTAMPTZ NOT NULL,
  processed BOOLEAN NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  UNIQUE (sha, rfd)
);

CREATE INDEX jobs ON job (id, processed ASC, committed_at ASC, created_at ASC);