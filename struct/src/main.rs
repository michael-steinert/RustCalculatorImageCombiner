/* Derive basic Implementations of `Debug` Trait */
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

impl User {
    /* `self` is the Instance that the Method is called on */
    fn print_sign_count(&self) -> u64 {
        self.sign_in_count
    }
}

fn main() {
    let mut user1 = User {
        username: String::from("Bruno"),
        email: String::from("bruno@mail.com"),
        sign_in_count: 1,
        is_active: true,
    };

    let user1_username = user1.username;
    user1.username = String::from("Buddy");

    let user2 = build_user(
        String::from("Bruno"),
        String::from("bruno@mail.org"),
    );

    let user3 = User {
        username: String::from("Buddy"),
        email: String::from("buddy@mail.org"),
        /* Reuse the remaining Fields from `user2` */
        ..user2
    };

    println!("{:#?}", user1);
    println!("Sign Count is {}", user2.print_sign_count());
    print_username_and_email(&user3);
}

fn build_user(username: String, email: String) -> User {
    /* Implicit Return */
    User {
        username,
        email,
        sign_in_count: 1,
        is_active: true,
    }
}

fn print_username_and_email(user: &User) {
    println!("{} - {}", user.username, user.email);
}