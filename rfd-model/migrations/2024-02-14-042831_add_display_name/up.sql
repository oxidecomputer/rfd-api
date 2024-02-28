ALTER TABLE api_user_provider ADD COLUMN display_names TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[];
