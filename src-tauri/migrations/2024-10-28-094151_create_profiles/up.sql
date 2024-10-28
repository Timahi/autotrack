-- Your SQL goes here
CREATE TABLE `profiles`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` TEXT NOT NULL UNIQUE,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP NOT NULL
);

