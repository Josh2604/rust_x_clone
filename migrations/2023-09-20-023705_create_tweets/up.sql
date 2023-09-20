-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `tweets` (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `message` TEXT NOT NULL,
  PRIMARY KEY (`id`)
);
