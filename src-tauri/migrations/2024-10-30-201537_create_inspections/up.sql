-- Your SQL goes here


CREATE TABLE `inspections`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`vehicle_id` INTEGER NOT NULL,
	`result` INTEGER NOT NULL,
	`performed_at` TIMESTAMP NOT NULL,
	`next_at` TIMESTAMP NOT NULL,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP NOT NULL,
	FOREIGN KEY (`vehicle_id`) REFERENCES `vehicles`(`id`) ON DELETE CASCADE
);

