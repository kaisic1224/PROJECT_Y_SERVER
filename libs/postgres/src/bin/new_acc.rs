use postgres::{create_acc, establish_connection};

fn main() {
    let conn = &mut establish_connection();

    create_acc(conn, "vincent.fong868@gmail.com", "kzxc", "123456");
}
