// Traits

pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
       return format!("User {} is {} years old", self.name, self.age)
    }
}

fn main() {
    let user = User {
        name: String::from("Vedansh"),
        age: 19,
    };
    println!("{}" , user.summarize())
}



pub trait Summary {
    fn summarize(&self) -> String {
        return String::from("Summarize");
    }
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {}

fn main() {
    let user = User {
        name: String::from("Vedansh"),
        age: 19,
    };
    println!("{}" , user.summarize())
}



pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let user = User {
        name: String::from("Vedansh"),
        age: 19,
    };
    notify(&user);
}