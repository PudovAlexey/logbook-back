ALTER TABLE loginfo
ALTER COLUMN depth set NOT NULL,
ALTER COLUMN start_datetime set NOT NULL,
ALTER COLUMN start_pressure set NOT NULL,
ALTER COLUMN end_pressure set NOT NULL;
