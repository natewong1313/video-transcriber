-- +goose Up
-- +goose StatementBegin
ALTER TABLE tasks ADD COLUMN transcript TEXT;
-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
ALTER TABLE tasks DROP COLUMN transcript;
-- +goose StatementEnd
