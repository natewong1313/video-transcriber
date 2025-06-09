-- +goose Up
-- +goose StatementBegin

CREATE TABLE IF NOT EXISTS tasks (
  id INT,
  url TEXT,
  status TEXT
);
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
DROP TABLE tasks;
-- +goose StatementEnd
