#[derive(Debug)]
struct User {
    activate: bool,
    name: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, name: String) -> User {
    User {
        email,
        name,
        activate: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(String::from("user1@domain"), String::from("user"));
    println!(
        "rect1 - activate: {} - name: {} - email: {} - sign in count: {}",
        user1.activate, user1.name, user1.email, user1.sign_in_count
    );

    let user2 = User {
        email: String::from("user2@domain"),
        ..user1
    };
    dbg!(&user2);

    // tuple structs
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    dbg!(&black);
    dbg!(&origin);
}
