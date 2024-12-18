DROP TRIGGER IF EXISTS set_updated_at ON access_groups;
SELECT diesel_manage_updated_at('access_groups');
DROP TRIGGER IF EXISTS set_updated_at ON api_user;
SELECT diesel_manage_updated_at('api_user');
DROP TRIGGER IF EXISTS set_updated_at ON api_key;
SELECT diesel_manage_updated_at('api_key');
DROP TRIGGER IF EXISTS set_updated_at ON api_user_provider;
SELECT diesel_manage_updated_at('api_user_provider');
DROP TRIGGER IF EXISTS set_updated_at ON api_user_access_token;
SELECT diesel_manage_updated_at('api_user_access_token');
DROP TRIGGER IF EXISTS set_updated_at ON login_attempt;
SELECT diesel_manage_updated_at('login_attempt');

ALTER TABLE mapper ADD COLUMN IF NOT EXISTS updated_at TIMESTAMPTZ NOT NULL DEFAULT now();
DROP TRIGGER IF EXISTS set_updated_at ON mapper;
SELECT diesel_manage_updated_at('mapper');

UPDATE api_user SET permissions = '[]' WHERE permissions = '[{"ManageGroupMemberships": []}, {"ManageGroups": []}, {"GetRfds": []}, {"UpdateRfds": []}, {"GetDiscussions": []}, {"GetOAuthClients": []}, {"UpdateOAuthClients": []}, {"DeleteOAuthClients": []}]';
UPDATE api_user SET permissions = '[]' WHERE permissions = '[{"ManageGroupMemberships": []}, {"ManageGroups": []}, {"GetRfds": []}, {"GetDiscussions": []}, {"GetOAuthClients": []}, {"UpdateOAuthClients": []}, {"DeleteOAuthClients": []}]';
