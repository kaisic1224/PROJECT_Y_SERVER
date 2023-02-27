// @generated automatically by Diesel CLI.

diesel::table! {
    account (user_id) {
        user_id -> Int4,
        email -> Varchar,
        account_username -> Varchar,
        account_password -> Varchar,
        date_created -> Timestamp,
    }
}

diesel::table! {
    chat (chat_id) {
        chat_id -> Int4,
        user1_id -> Int4,
        user2_id -> Int4,
    }
}

diesel::table! {
    friend_request (request_id) {
        request_id -> Int4,
        user1_id -> Int4,
        user2_id -> Int4,
        request_date -> Timestamp,
    }
}

diesel::table! {
    friendship (user1_id, user2_id) {
        user1_id -> Int4,
        user2_id -> Int4,
        date_created -> Timestamp,
    }
}

diesel::table! {
    messages (message_id) {
        message_id -> Int4,
        s3_path -> Varchar,
        chat_id -> Int4,
        user_id -> Int4,
        date_created -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::joinable!(messages -> chat (chat_id));

diesel::allow_tables_to_appear_in_same_query!(
    account,
    chat,
    friend_request,
    friendship,
    messages,
    posts,
);
