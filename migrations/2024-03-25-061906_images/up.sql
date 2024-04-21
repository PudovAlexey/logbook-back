CREATE TABLE image (
    id SERIAL PRIMARY KEY,
    path TEXT NOT NULL,
    filename VARCHAR(100),
    created_at TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE avatar (
    id SERIAL PRIMARY KEY,
    image_id INTEGER NOT NULL REFERENCES image (id),
    user_id UUID NOT NULL REFERENCES users (id),
    UNIQUE (user_id, image_id)
)