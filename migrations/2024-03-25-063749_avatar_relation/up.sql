ALTER TABLE users
ADD COLUMN avatar_id INTEGER,
ADD CONSTRAINT fk_avatar_id
FOREIGN KEY (avatar_id) REFERENCES avatar (id);
