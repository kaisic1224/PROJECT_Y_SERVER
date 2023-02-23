use self::models::Post;
use diesel::prelude::*;
use postgres::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post required an id")
        .parse::<i32>()
        .expect("Invalid ID");

    let conn = &mut establish_connection();

    let post: Post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .unwrap();

    println!("printed out post: {}", post.title)
}
