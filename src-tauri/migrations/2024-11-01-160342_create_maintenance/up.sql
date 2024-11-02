-- Your SQL goes here



CREATE TABLE `maintenance`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`vehicle_id` INTEGER NOT NULL,
	`type` TEXT NOT NULL,
	`description` TEXT,
    `odometer` INTEGER NOT NULL,
	`performed_at` TIMESTAMP NOT NULL,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP NOT NULL,
	FOREIGN KEY (`vehicle_id`) REFERENCES `vehicles`(`id`) ON DELETE CASCADE
);

