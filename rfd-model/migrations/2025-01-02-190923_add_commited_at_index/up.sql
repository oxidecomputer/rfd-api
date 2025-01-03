CREATE INDEX rfd_revision_sort_no_content_idx ON rfd_revision (rfd_id, committed_at desc) include(id, title, state, discussion, authors, content_format, sha, commit_sha, created_at, updated_at, deleted_at, labels);
CREATE INDEX rfd_number_idx ON rfd (rfd_number);
