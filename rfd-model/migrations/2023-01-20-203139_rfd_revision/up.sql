CREATE TYPE RFD_CONTENT_FORMAT as ENUM('asciidoc', 'markdown');

CREATE TABLE rfd_revision (
  id UUID PRIMARY KEY,
  rfd_id UUID REFERENCES rfd (id) NOT NULL,
  title VARCHAR NOT NULL,
  state VARCHAR,
  discussion VARCHAR,
  authors VARCHAR,
  content VARCHAR NOT NULL,
  content_format RFD_CONTENT_FORMAT NOT NULL,
  sha VARCHAR NOT NULL,
  commit_sha VARCHAR NOT NULL,
  committed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE UNIQUE INDEX rfd_revision_sha_idx ON rfd_revision (rfd_id, sha);
