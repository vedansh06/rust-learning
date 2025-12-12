//? Structs

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    print!("User 1 username: {:?}", user1.username);
}



struct User {
    active: bool,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
    };

    print_name(user1);
    print!("User 1 username: {}", user1.active); 
}

fn print_name(user1: User) {
    print!("User 1 username: {}", user1.active);
}



struct User {
    active: bool,
    sign_in_count: u64,
    username: String,
}

fn main() {
    let mut user1 = User {
        active: true,
        sign_in_count: 1,
        username: "harkirat".to_string()
    };

    change_name(user1);
    print!("User 1 username: {}", user1.active); 
}

fn change_name(user1: User) {
    print!("User 1 username: {:?}", user1.active);
}



struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let user = User {
        first_name: String::from("Vedansh"),
        last_name: String::from("Saini"),
        age: 32,
    };

    println!("{}", user.first_name);
}
