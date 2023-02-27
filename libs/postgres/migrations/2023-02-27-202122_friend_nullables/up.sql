-- Your SQL goes here
ALTER TABLE account
    ALTER COLUMN account_username SET NOT NULL,
    ALTER COLUMN account_password SET NOT NULL;

ALTER TABLE messages
    ALTER COLUMN s3_path SET NOT NULL;