-- +goose Up
-- +goose StatementBegin
-- +goose StatementEnd
-- thank you claude
BEGIN;

-- Step 1: Add new UUID column
ALTER TABLE tasks ADD COLUMN uuid_id UUID;

-- Step 2: Populate new column with UUIDs
UPDATE tasks SET uuid_id = gen_random_uuid();

-- Step 4: Drop the old integer id column
ALTER TABLE tasks DROP COLUMN id;

-- Step 5: Rename uuid_id to id
ALTER TABLE tasks RENAME COLUMN uuid_id TO id;

-- Step 6: Set new id as PRIMARY KEY
ALTER TABLE tasks ADD PRIMARY KEY (id);

COMMIT;

-- +goose Down
-- +goose StatementBegin
-- Step 1: Add old id column back
ALTER TABLE tasks ADD COLUMN old_id SERIAL;

-- Step 2: Assign temporary ids (you can reassign original integers if they were saved elsewhere)
-- Just fill in incremental IDs
WITH numbered AS (
  SELECT id, ROW_NUMBER() OVER () AS rn
  FROM tasks
)
UPDATE tasks
SET old_id = numbered.rn
FROM numbered
WHERE tasks.id = numbered.id;

-- Step 3: Drop current UUID primary key
ALTER TABLE tasks DROP CONSTRAINT tasks_pkey;

-- Step 4: Drop UUID id column
ALTER TABLE tasks DROP COLUMN id;

-- Step 5: Rename old_id back to id
ALTER TABLE tasks RENAME COLUMN old_id TO id;

-- Step 6: Add primary key constraint
ALTER TABLE tasks ADD PRIMARY KEY (id);

COMMIT;
-- +goose StatementEnd
