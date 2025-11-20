-- This file should undo anything in `up.sql`

ALTER TABLE users
DROP COLUMN updated_at,
DROP COLUMN is_active,
DROP COLUMN bio,
DROP COLUMN avatar_url,
DROP COLUMN surname; 