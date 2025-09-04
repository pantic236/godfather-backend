-- Add migration script here
ALTER TABLE users ADD COLUMN lifetime_hours INTEGER DEFAULT 0;