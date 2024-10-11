-- Your SQL goes here

ALTER TABLE message
ADD COLUMN user_id UUID;

ALTER TABLE message
ADD CONSTRAINT fk_user_id FOREIGN KEY (user_id) REFERENCES users(id);
