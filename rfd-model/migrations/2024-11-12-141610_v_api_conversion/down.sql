DROP TRIGGER set_updated_at ON mapper;
ALTER TABLE mapper DROP COLUMN updated_at;

DROP TRIGGER set_updated_at ON login_attempt;
DROP TRIGGER set_updated_at ON api_user_access_token;
DROP TRIGGER set_updated_at ON api_user_provider;
DROP TRIGGER set_updated_at ON api_key;
DROP TRIGGER set_updated_at ON api_user;
DROP TRIGGER set_updated_at ON access_groups;
