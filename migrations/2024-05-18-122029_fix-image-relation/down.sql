-- This file should undo anything in `up.sql`
CREATE UNIQUE INDEX idx_image_id ON log_image (image_id);

ALTER TABLE loginfo
ADD COLUMN image_id INTEGER REFERENCES log_image (image_id),
ADD CONSTRAINT fk_image_id
FOREIGN KEY (image_id) REFERENCES log_image (id);