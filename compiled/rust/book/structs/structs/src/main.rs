fn main () {
    let user1 = User {
        email: String::from("s@b.com"),
        username: String::from("abc"),
        active: true,
        sign_in_count: 1,
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
