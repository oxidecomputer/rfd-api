CREATE TABLE rfd_comment_user (
  id UUID PRIMARY KEY,
  external_id INTEGER NOT NULL,
  node_id VARCHAR NOT NULL,

  user_username VARCHAR,
  user_avatar_url VARCHAR,
  user_type VARCHAR NOT NULL,

  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,

  UNIQUE(external_id),
  UNIQUE(node_id)
);

CREATE TABLE rfd_review (
    id UUID PRIMARY KEY,
    rfd_id UUID REFERENCES rfd (id) NOT NULL,
    comment_user_id UUID REFERENCES rfd_comment_user(id) NOT NULL,
    external_id INTEGER NOT NULL,
    node_id VARCHAR NOT NULL,

    body VARCHAR NOT NULL,
    state VARCHAR NOT NULL,
    commit_id VARCHAR NOT NULL,

    review_created_at TIMESTAMPTZ NOT NULL,

    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    deleted_at TIMESTAMPTZ,

    UNIQUE(external_id),
    UNIQUE(node_id)
);

CREATE TABLE rfd_review_comment (
    id UUID PRIMARY KEY,
    rfd_id UUID REFERENCES rfd (id) NOT NULL,
    comment_user_id UUID REFERENCES rfd_comment_user(id) NOT NULL,
    external_id INTEGER NOT NULL,
    node_id -> VARCHAR NOT NULL,

    review_id UUID REFERENCES rfd_review(id),

    diff_hunk -> VARCHAR NOT NULL,
    path -> VARCHAR NOT NULL,
    body -> VARCHAR NOT NULL,
    commit_id -> VARCHAR NOT NULL,
    original_commit_id -> VARCHAR NOT NULL,
    line INTEGER,
    original_line INTEGER,
    start_line INTEGER,
    original_start_line INTEGER,
    side -> VARCHAR,
    start_side -> VARCHAR,
    subject VARCHAR NOT NULL,
    in_reply_to INTEGER,

    comment_created_at TIMESTAMPTZ NOT NULL,
    comment_updated_at TIMESTAMPTZ NOT NULL,

    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    deleted_at TIMESTAMPTZ,

    UNIQUE(external_id),
    UNIQUE(node_id)
);

CREATE TABLE rfd_comment (
    id UUID PRIMARY KEY,
    rfd_id UUID REFERENCES rfd (id) NOT NULL,
    comment_user_id UUID REFERENCES rfd_comment_user(id) NOT NULL,
    external_id INTEGER NOT NULL,
    node_id VARCHAR NOT NULL,

    body VARCHAR,

    comment_created_at TIMESTAMPTZ NOT NULL,
    comment_updated_at TIMESTAMPTZ NOT NULL,

    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    deleted_at TIMESTAMPTZ,

    UNIQUE(external_id),
    UNIQUE(node_id)
);
