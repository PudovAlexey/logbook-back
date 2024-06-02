-- Your SQL goes here
-- ALTER TABLE users
-- ADD COLUMN avatar_id INTEGER,
-- ADD CONSTRAINT fk_avatar_id
-- FOREIGN KEY (avatar_id) REFERENCES avatar (id);

ALTER TABLE loginfo DROP COLUMN image_id;

DROP INDEX idx_image_id;

ALTER TABLE loginfo
ADD COLUMN image_id INTEGER,
ADD CONSTRAINT fk_image_id
FOREIGN KEY (image_id) REFERENCES log_image (id);


