CREATE TABLE log_image (
    id SERIAL PRIMARY KEY,
    image_id INTEGER NOT NULL REFERENCES image (id),
    logbook_id INTEGER NOT NULL REFERENCES loginfo (id),
    UNIQUE (logbook_id, image_id)
);

-- ALTER TABLE log_image
CREATE UNIQUE INDEX idx_image_id ON log_image (image_id);

ALTER TABLE loginfo
ADD COLUMN image_id INTEGER REFERENCES log_image (image_id),
ADD CONSTRAINT fk_image_id
FOREIGN KEY (image_id) REFERENCES log_image (id);