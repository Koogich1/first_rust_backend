-- Откат: удаляем автоинкремент и последовательность
DROP SEQUENCE IF EXISTS users_id_seq;

-- Устанавливаем значение по умолчанию как NULL (что потребует ручного указания id)
ALTER TABLE users ALTER COLUMN id DROP DEFAULT;