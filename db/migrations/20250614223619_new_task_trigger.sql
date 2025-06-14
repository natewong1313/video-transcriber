-- +goose Up
-- +goose StatementBegin
CREATE OR REPLACE FUNCTION notify_task_insert()
RETURNS trigger AS $$
BEGIN
  PERFORM pg_notify('task_inserted', row_to_json(NEW)::text);
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER task_insert_trigger
AFTER INSERT ON tasks
FOR EACH ROW
EXECUTE FUNCTION notify_task_insert();
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TRIGGER IF EXISTS task_insert_trigger ON tasks;
DROP FUNCTION IF EXISTS notify_task_insert();
-- +goose StatementEnd
