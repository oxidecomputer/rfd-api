CREATE TYPE RFD_PDF_SOURCE as ENUM('github', 'google');

CREATE TABLE rfd_pdf (
  id UUID PRIMARY KEY,
  rfd_revision_id UUID REFERENCES rfd_revision (id) NOT NULL,
  source RFD_PDF_SOURCE NOT NULL,
  link VARCHAR NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,

  CONSTRAINT revision_links_unique UNIQUE (rfd_revision_id, source, link)
);