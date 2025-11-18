-- Remove DEFAULT from created_at column
ALTER TABLE users ALTER COLUMN created_at DROP DEFAULT;
