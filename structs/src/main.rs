#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("user1 is {user1:?}");

    user1.email = String::from("anotheremail@example.com");
    println!("user1 is {user1:?}");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("user2 is {user2:#?}");

    let user3 = build_user(String::from("user3@example.com"), String::from("user3"));
    dbg!(&user3);

    let _ = User {
        username: dbg!(String::from("user4")),
        email: String::from("user4@example.com"),
        ..user3
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
