-- Add DEFAULT NOW() to created_at column
ALTER TABLE users ALTER COLUMN created_at SET DEFAULT NOW();
