-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `likes` (
  `id` BIGINT NOT NULL AUTO_INCREMENT,
  `created_at` TIMESTAMP NOT NULL DEFAULT NOW(),
  `tweet_id` BIGINT NOT NULL REFERENCES `tweets`(id),
  PRIMARY KEY (`id`),
  FOREIGN KEY (`tweet_id`) REFERENCES `tweets`(id)
);
