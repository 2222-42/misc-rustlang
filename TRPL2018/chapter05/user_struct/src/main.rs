struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct UnitStruct();

fn main() {
    let mut user1 = build_user(String::from("test@example.com"), String::from("tester"));
    println!("{}", user1.username);
    user1.email = String::from("test2@example.com");
    println!("{}", user1.email);

    let user2 = User {
        email: String::from("test@example.com"),
        username: String::from("tester2"),
        ..user1
    };
    println!("user2's username: {}", user2.username);
    println!("user2's email: {}", user2.email);
    println!("user2's sign_in_count: {}", user2.sign_in_count);
    println!("user2's active: {}", user2.active);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}", black.0);
    println!("{}", origin.0);

    // struct NotOwnedUser {
    //     username: &str,
    //     email: &str,
    //     sign_in_count: u64,
    //     active: bool,
    // };

    // let fakeUser = NotOwnedUser {
    //     email: "someone@example.com",
    //     username: "fake",
    //     sign_in_count: 1,
    //     active: false,
    // };
    let p1 = CartesianPoint { x: 0.0, y: 0.0 };
    let p2 = CartesianPoint { x: 5.0, y: 6.5 };
    p1.distance(&p2);
    (&p1).distance(&p2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug, Copy, Clone)]
struct CartesianPoint {
    x: f64,
    y: f64,
}

impl CartesianPoint {
    fn distance(&self, other: &CartesianPoint) -> f64 {
        let x_squared = f64::powi(other.x - self.x, 2);
        let y_squared = f64::powi(other.y - self.y, 2);

        f64::sqrt(x_squared + y_squared)
    }
}
