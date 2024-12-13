#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user = build_uer(String::from("_"), String::from("Ellie"));
    let email = user.email;
    println!("user email: {email}");
    // user
    user.email = String::from("xx@foxmail.com");
    println!("user email: {}", user.email);
    // user2 by update struct
    let user2 = User {
        email: String:: from("yy@foxmail.com"),
        ..user
    };
    println!("user2 email: {:#?}", user2);
    // println!("we cant use user anymore: {:#?}", user); // partial move occurs because `user.username` has type `String`

    // Tuple struct
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}", black);
    println!("origin: {:?}", origin);
}

fn build_uer(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
