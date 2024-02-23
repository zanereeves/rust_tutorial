struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count:u64
}

struct Color(i32, i32, i32);

fn main() {
    let user = User {
        active: true,
        username: String::from("hellovanier"),
        email: String::from("blahblah@gmail.com"),
        sign_in_count: 1
    };

    user.username = String::from("hellojeb");


    // The entire instance of a struct is mutable
    let user2 = create_user("akshdkah@example.com", "zreeves");

    println!("{}", user2.username);

    let user3 = User {
        email: String::from("struct update syntax"),
        .. user2
    };


    let black = Color(0,0,0);
}


fn create_user(email, username) -> User {
     User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    }

}
