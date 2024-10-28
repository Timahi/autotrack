-- Your SQL goes here
CREATE TABLE `profiles`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`name` VARCHAR NOT NULL,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP
);

