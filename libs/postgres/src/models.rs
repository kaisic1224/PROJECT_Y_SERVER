use diesel::{data_types::PgTimestamp, prelude::*};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Account {
    pub user_id: i32,
    pub email: String,
    pub account_username: String,
    pub account_password: String,
    pub date_created: PgTimestamp,
}

#[derive(Queryable)]
pub struct FriendRequest {
    pub request_id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
}

#[derive(Queryable)]
pub struct Friendship {
    pub user1_id: i32,
    pub user2_id: i32,
    pub date_created: PgTimestamp,
}

#[derive(Queryable)]
pub struct Messages {
    pub message_id: i32,
    pub s3_path: String,
    pub chat_id: i32,
    pub user_id: i32,
    pub date_created: PgTimestamp,
}

#[derive(Queryable)]
pub struct Chat {
    pub chat_id: i32,
    pub user1_id: i32,
    pub user2_id: i32,
}

use crate::schema::posts;
#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

use crate::schema::account;
#[derive(Insertable)]
#[diesel(table_name = account)]
pub struct NewAccount<'a> {
    pub email: &'a str,
    pub account_username: &'a str,
    pub account_password: &'a str,
}
