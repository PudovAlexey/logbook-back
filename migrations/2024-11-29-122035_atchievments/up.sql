CREATE TABLE achievement (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description VARCHAR(300) NOT NULL,
    image_id INTEGER NOT NULL REFERENCES image(id)
);

CREATE TABLE task (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    description VARCHAR(300) NOT NULL,
    image_id INTEGER NOT NULL REFERENCES image(id)
);

CREATE TABLE user_achievement (
    id SERIAL PRIMARY KEY,
    achievement_id INTEGER NOT NULL REFERENCES achievement(id),
    user_id UUID NOT NULL REFERENCES users(id),
    task_id INTEGER NOT NULL REFERENCES task(id)
);