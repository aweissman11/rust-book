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
        username: String::from("some_username123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    user1.email = String::from("another_email@example.com");
    // user2.username = String::from("another_username123");
    println!("user1 {:?}:", user1);
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         username,
//         email,
//         sign_in_count: 1,
//     }
// }
