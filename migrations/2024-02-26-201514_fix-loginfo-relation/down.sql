ALTER TABLE loginfo DROP COLUMN user_id;

ALTER TABLE loginfo ADD COLUMN user_id INT REFERENCES loginfo(id) NOT NULL;