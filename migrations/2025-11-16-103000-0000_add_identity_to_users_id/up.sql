-- Создаем последовательность для автоинкремента
CREATE SEQUENCE IF NOT EXISTS users_id_seq OWNED BY users.id;

-- Устанавливаем текущее значение последовательности на максимальный id в таблице
SELECT setval('users_id_seq', COALESCE((SELECT MAX(id) FROM users), 1), true);

-- Добавляем автоинкремент к полю id в таблице users
ALTER TABLE users ALTER COLUMN id SET DEFAULT nextval('users_id_seq'::regclass);