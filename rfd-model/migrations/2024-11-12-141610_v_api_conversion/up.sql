SELECT diesel_manage_updated_at('access_groups');
SELECT diesel_manage_updated_at('api_user');
SELECT diesel_manage_updated_at('api_key');
SELECT diesel_manage_updated_at('api_user_provider');
SELECT diesel_manage_updated_at('api_user_access_token');
SELECT diesel_manage_updated_at('login_attempt');

ALTER TABLE mapper ADD COLUMN updated_at TIMESTAMPTZ NOT NULL DEFAULT now();
SELECT diesel_manage_updated_at('mapper');
