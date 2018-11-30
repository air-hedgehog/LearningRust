fn main() {
    //instance of struct
    let mut user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));

    //assign speccific field of the struct. Whole struct must be mutable
    user1.email = String::from("anotheremail@example.com");

    //update struct syntax:
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        // '..' syntax takes remaining fields from user1 untouched
        ..user1
    };

    //tuple struct:
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let m = Message::Write(String::from("hello"));
    m.call();
}

//function to create instance of struct. As struct is the last expression
//of the function, build_user returns struct "User"
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

//using variable Shorthands
fn build_user_short(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

//defining struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
//implementations can be applied to enums as well as the structs
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}
