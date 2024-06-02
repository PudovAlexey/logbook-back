ALTER TABLE loginfo
DROP CONSTRAINT fk_image_id;

ALTER TABLE loginfo
DROP COLUMN image_id;

DROP TABLE log_images;