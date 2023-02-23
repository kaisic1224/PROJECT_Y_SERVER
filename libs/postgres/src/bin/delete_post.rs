use diesel::prelude::*;
use postgres::*;
use std::env::args;

fn main() {
    use self::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}", target);

    let conn = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(conn)
        .expect("Error deleting post");

    println!("deleted post: {}", num_deleted)
}
