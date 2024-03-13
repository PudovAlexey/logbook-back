CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Изменяем тип столбца на UUID с помощью нового временного столбца

ALTER TABLE loginfo DROP COLUMN user_id;
ALTER TABLE users
ADD COLUMN id_new UUID;
UPDATE users SET id_new = uuid_generate_v4();
ALTER TABLE users
DROP COLUMN id,
ADD COLUMN id UUID DEFAULT uuid_generate_v4();

-- Добавляем PRIMARY KEY
ALTER TABLE users
ADD PRIMARY KEY (id);

ALTER TABLE loginfo ADD COLUMN user_id UUID REFERENCES users(id) NOT NULL;
DROP COLUMN id_new;