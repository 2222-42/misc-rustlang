struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = build_user(String::from("test@example.com"), String::from("tester"));
    println!("{}", user1.username);
    user1.email = String::from("test2@example.com");
    println!("{}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
