-- Your SQL goes here

CREATE TABLE `vehicles`(
	`id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	`brand` TEXT NOT NULL,
	`model` TEXT NOT NULL,
	`odometer` INTEGER NOT NULL,
	`odometer_updated_at` TIMESTAMP NOT NULL,
	`registration` TEXT NOT NULL,
	`registration_year` INTEGER NOT NULL,
	`serial_number` TEXT,
	`description` TEXT,
	`profile_id` INTEGER NOT NULL,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP NOT NULL,
	FOREIGN KEY (`profile_id`) REFERENCES `profiles`(`id`) ON DELETE CASCADE 
);

