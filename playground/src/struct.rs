struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@email.com"),
        sign_in_count: 1,
    };

    println!("email = {}", user1.email);
    user1.email = String::from("haha@email.com");
    println!("email = {}", user1.email);

    let user2 = User {
        username: String::from("user2"),
        ..user1
    };
    println!("email = {}", user2.email);
}
