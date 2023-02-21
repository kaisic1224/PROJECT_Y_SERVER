use self::models::*;
use diesel::prelude::*;
use PROJECT_Y_SERVER::*;

fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection);
}
