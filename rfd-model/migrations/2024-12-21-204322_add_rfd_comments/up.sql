CREATE TABLE rfd_comment_user (
  id UUID PRIMARY KEY,
  github_user_id INTEGER NOT NULL,
  github_user_node_id VARCHAR NOT NULL,
  github_user_username VARCHAR,
  github_user_avatar_url VARCHAR,
  github_user_type VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ,

  UNIQUE(github_user_id),
  UNIQUE(github_user_node_id)
);

CREATE TABLE rfd_comment (
    id UUID PRIMARY KEY,
    rfd_id UUID REFERENCES rfd (id) NOT NULL,
    comment_user_id UUID REFERENCES rfd_comment_user(id) NOT NULL,
    external_id INTEGER NOT NULL,
    node_id -> VARCHAR NOT NULL,
    discussion_number INTEGER,
    diff_hunk -> VARCHAR NOT NULL,
    path -> VARCHAR NOT NULL,
    body -> VARCHAR,
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
    comment_created_at TIMESTAMPTZ,
    comment_updated_at TIMESTAMPTZ,
    created_at -> TIMESTAMPTZ NOT NULL,
    updated_at -> TIMESTAMPTZ NOT NULL,
    deleted_at -> TIMESTAMPTZ,

    UNIQUE(external_id),
    UNIQUE(node_id)
);
