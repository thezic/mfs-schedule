-- Add migration script here
ALTER TABLE persons ADD COLUMN comment TEXT NOT NULL DEFAULT '';
