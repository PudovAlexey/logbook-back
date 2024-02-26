CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(100) UNIQUE NOT NULL,
    name VARCHAR(50) NOT NULL,
    surname VARCHAR(50),
    patronymic VARCHAR(50),
    role VARCHAR(10) NOT NULL,
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    date_of_birth TIMESTAMP
);

ALTER TABLE loginfo
ADD user_id INT REFERENCES loginfo(id) NOT NULL;
