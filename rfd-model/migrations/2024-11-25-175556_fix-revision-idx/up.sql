DROP INDEX rfd_revision_commit_sha_idx;
CREATE UNIQUE INDEX rfd_revision_commit_sha_idx ON rfd_revision (rfd_id, commit_sha) WHERE deleted_at IS NULL;
