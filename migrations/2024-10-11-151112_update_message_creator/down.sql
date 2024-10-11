-- This file should undo anything in `up.sql`
ALTER TABLE message
DROP CONSTRAINT fk_user_id;

ALTER TABLE message
DROP COLUMN user_id;