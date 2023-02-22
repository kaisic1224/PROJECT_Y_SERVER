use std::io::{stdin, Read};
use PROJECT_Y_SERVER::*;

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("Enter a title");
    stdin().read_line(&mut title).ok();

    let title = title.trim_end();

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("\n Post created and saved with id: {}", post.id)
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";
