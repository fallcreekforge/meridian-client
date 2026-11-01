-- Disable the enforcement of foreign-keys constraints
PRAGMA foreign_keys = off;
-- Create "new_test_parent" table
CREATE TABLE `new_test_parent` (`id` integer NULL, `name` text NOT NULL, PRIMARY KEY (`id`), CHECK (length(name) > 0));
-- Copy rows from old table "test_parent" to new temporary table "new_test_parent"
INSERT INTO `new_test_parent` (`id`, `name`) SELECT `id`, `name` FROM `test_parent`;
-- Drop "test_parent" table after copying rows
DROP TABLE `test_parent`;
-- Rename temporary table "new_test_parent" to "test_parent"
ALTER TABLE `new_test_parent` RENAME TO `test_parent`;
-- Create index "test_parent_name" to table: "test_parent"
CREATE UNIQUE INDEX `test_parent_name` ON `test_parent` (`name`);
-- Enable back the enforcement of foreign-keys constraints
PRAGMA foreign_keys = on;
