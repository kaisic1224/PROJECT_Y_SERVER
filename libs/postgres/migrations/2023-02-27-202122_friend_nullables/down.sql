-- This file should undo anything in `up.sql`
ALTER TABLE account
    MODIFY COLUMN account_username DROP NOT NULL,
    MODIFY COLUMN account_password DROP NOT NULL;

ALTER TABLE messages
    MODIFY COLUMN s3_path DROP NOT NULL;