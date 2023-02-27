CREATE TABLE account (
    user_id SERIAL PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    account_username VARCHAR(50) UNIQUE,
    account_password VARCHAR(50),
    date_created TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE friendship (
    user1_id INT NOT NULL,
    user2_id INT NOT NULL,
    date_created TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user1_id, user2_id),
    CONSTRAINT fk_user1
        FOREIGN KEY (user1_id)
            REFERENCES account(user_id),
    CONSTRAINT fk_user2
        FOREIGN KEY (user2_id)
            REFERENCES account(user_id)
);

CREATE TABLE friend_request (
    request_id SERIAL PRIMARY KEY,
    user1_id INT NOT NULL,
    user2_id INT NOT NULL,
    request_date TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_user1
        FOREIGN KEY (user1_id)
            REFERENCES account(user_id),
    CONSTRAINT fk_user2
        FOREIGN KEY (user2_id)
            REFERENCES account(user_id)
);

CREATE TABLE chat (
    chat_id SERIAL PRIMARY KEY,
    user1_id INT NOT NULL,
    user2_id INT NOT NULL,
    CONSTRAINT fk_user1
        FOREIGN KEY (user1_id)
            REFERENCES account(user_id),
    CONSTRAINT fk_user2
        FOREIGN KEY (user2_id)
            REFERENCES account(user_id)
);

CREATE TABLE messages (
    message_id SERIAL PRIMARY KEY,
    s3_path VARCHAR(255),
    chat_id INT NOT NULL,
    user_id INT NOT NULL,
    date_created TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_chat
        FOREIGN KEY (chat_id)
            REFERENCES chat(chat_id),
    CONSTRAINT fk_user1
        FOREIGN KEY (user_id)
            REFERENCES account(user_id)
);