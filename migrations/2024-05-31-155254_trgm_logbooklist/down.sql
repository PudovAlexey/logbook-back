-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS trgm_loginfo_title_idx;
DROP INDEX IF EXISTS trgm_loginfo_description_idx;

-- Отмена создания расширения
DROP EXTENSION IF EXISTS pg_trgm;