-- +goose Up
-- +goose StatementBegin
CREATE SEQUENCE IF NOT EXISTS tasks_id_seq;
SELECT setval('tasks_id_seq', COALESCE((SELECT MAX(id) FROM tasks), 0) + 1, false);
ALTER TABLE tasks ALTER COLUMN id SET DEFAULT nextval('tasks_id_seq');
ALTER SEQUENCE tasks_id_seq OWNED BY tasks.id;
ALTER TABLE tasks ALTER COLUMN id SET NOT NULL;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE tasks ALTER COLUMN id DROP DEFAULT;
DROP SEQUENCE IF EXISTS tasks_id_seq;
-- +goose StatementEnd
