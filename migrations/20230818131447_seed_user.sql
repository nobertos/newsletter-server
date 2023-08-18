-- Add migration script here
-- 9d43be64-9de4-4786-ae1b-f868809e28d9
-- $argon2id$v=19$m=15000,t=2,p=1$VLoRi07TMGOkmZB2Rft6zA$7XeAvK4+jY2an5MQQ2SBmvJdr5o7TCheYVGdaMUMg3M

INSERT INTO users(user_id, username, password_hash)
VALUES (
  '9d43be64-9de4-4786-ae1b-f868809e28d9',
  'admin',
  '$argon2id$v=19$m=15000,t=2,p=1$VLoRi07TMGOkmZB2Rft6zA$7XeAvK4+jY2an5MQQ2SBmvJdr5o7TCheYVGdaMUMg3M'
)

