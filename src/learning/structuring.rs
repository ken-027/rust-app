use crate::helpers::spaces;

#[derive(Debug)]
struct User {
    id: u64,
    username: String,
    email: String,
    password: String,
}

impl User {
    fn create_another(&self) -> Self {
        User {
            id: 23,
            email: String::from("ken@email.coms"),
            username: String::from("kean"),
            password: String::from("password123"),
        }
    }
}

pub fn main() {
    users();
}

fn users() {
    let user = User {
        id: 23,
        email: String::from("ken@email.com"),
        username: String::from("ken"),
        password: String::from("password123"),
    };

    let users = [
        User {
            id: 1,
            email: String::from("ken@email.com"),
            username: String::from("ken"),
            password: String::from("password123"),
        },
        User {
            id: 2,
            email: String::from("ken2@email.com"),
            username: String::from("kean"),
            password: String::from("password123"),
        },
        User {
            id: 3,
            email: String::from("ken2@email.com"),
            username: String::from("kean"),
            password: String::from("password123"),
        },
        User {
            email: user.email.clone(),
            username: user.username.clone(),
            password: user.password.clone(),
            ..user
        },
    ];

    println!("User: {:?}", user.create_another());
    spaces(1);
    println!("Users: {:?}", users);
}
