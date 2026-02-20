-- Migration to create the initialization table
-- This table tracks whether the system has been initialized with an initial OAuth client

CREATE TABLE initialization (
    id UUID PRIMARY KEY,
    initialized_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    oauth_client_id UUID NOT NULL
);
