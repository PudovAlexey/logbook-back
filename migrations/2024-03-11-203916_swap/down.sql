ALTER TABLE loginfo DROP CONSTRAINT loginfo_user_id_fkey;
ALTER TABLE loginfo DROP COLUMN user_id;

-- Вернуть старый тип столбца id в таблице users
ALTER TABLE users DROP COLUMN id;
ALTER TABLE users ADD COLUMN id SERIAL PRIMARY KEY;

-- Удалить временный столбец id_new из таблицы users
ALTER TABLE users DROP COLUMN id_new;

-- Удалить расширение uuid-ossp, если оно было добавлено
DROP EXTENSION IF EXISTS "uuid-ossp";