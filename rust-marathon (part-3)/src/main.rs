fn main() {
    println!("Hello, world!");
}

// Crates

// chrono

use chrono::{Local, Utc};

fn main() {
    let utc_time = Utc::now();
    let local_time = Local::now();
    println!("local time is {}", utc_time);
    println!("native time is {}", local_time);
}

// dotenv

use dotenv::dotenv;
use std::env;

fn main() {
  dotenv().ok();
  let var = env::var("REDIS_ADDRESS").unwrap();
  println!("{}", var);
}

// uuid

use uuid::Uuid;

fn main() {
    let random_uuid = Uuid::new_v4();
    println!("{}", random_uuid);
}


//  Generics and trait bounds

pub fn main() {
    let v = vec![1, 2, 3];
    let v2 = vec![String::from("Vedansh"), String::from("Saini")];
    let v3 = vec![1.0, 2.0, 3.0];
    println!("{}", first_element(v).unwrap());
    println!("{}", first_element(v2).unwrap());
    println!("{}", first_element(v3).unwrap());
}

fn first_element<T>(v: Vec<T>) -> Option<T> {
    return v.into_iter().nth(0);
}


pub fn main() {
    let v = vec![1, 2, 3];
    let v2 = vec![String::from("Vedansh"), String::from("Saini")];
    let v3 = vec![1.0, 2.0, 3.0];
    println!("{}", first_element(v.clone()).unwrap());
    println!("{}", first_element(v2).unwrap());
    println!("{}", first_element(v3).unwrap());

    println!("{}", does_exist(v, 1));
}

fn first_element<T>(v: Vec<T>) -> Option<T> {
    return v.into_iter().nth(0);
}

fn does_exist<T: std::cmp::PartialEq>(v: Vec<T>, element: T) -> bool {
    let mut iter = v.iter();
    while let Some(value) = iter.next() {
        if *value == element {
            return true;
        }
    }
    return false;
}


// Generics over structs

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    pub fn area(&self) -> u32 {
        self.height * self.width
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 20,
    };

    println!("{}", r.area());
}


struct Rect<T> {
    width: T,
    height: T,
}

impl<T: std::ops::Mul<Output = T> + Copy> Rect<T> {
    pub fn area(&self) -> T {
        return self.height * self.width;
    }
}

fn main() {
    let r = Rect {
        width: 10,
        height: 20,
    };

    println!("{}", r.area());
}


// Declerative Macros

macro_rules! say_hello {
    () => {
        println!("Hello, world!");
    };
}

fn main() {
    say_hello!();  
}


macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name() {
            println!("Hello from {}", stringify!($func_name));
        }
    };
}

create_function!(hello); 

fn main() {
    hello();  
}


// Procedural Macros

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "user_name")]
    username: String,
    
    #[serde(rename = "pass_word")]
    password: String,
    
    #[serde(rename = "user_age")] 
    age: u32,
}

fn main() {
    let user = User {
        username: String::from("Alice"),
        password: String::from("password123"),
        age: 30,
    };

    // Serializing to JSON
    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json); 
}


// Assignment

macro_rules! generate_functions {
    ($($func_name:ident),*) => {
        $(
            fn $func_name() {
                println!("Hello from {}", stringify!($func_name));
            }
        )*
    };
}

generate_functions!(foo, bar, baz);

fn main() {
    foo();  
    bar();  
    baz();  
}


// Debug and Display trait and macros

use std::fmt;

struct Rect {
	width: u32,
	height: u32
}

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn main() {
    let s = Rect {
        width: 100,
        height: 100
    };
    println!("{:?}", s);    
}


use std::fmt;

#[derive(Debug)]
struct Rect {
	width: u32,
	height: u32
}

impl std::fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}

fn main() {
    let s = Rect {
        width: 100,
        height: 100
    };
    println!("{:?}", s);    
}


//  Copy, Clone macros

fn main() {
    let a = 5;  
    let b = a;  
    println!("a: {}, b: {}", a, b);  
}



#[derive(Clone)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person1 = Person {
        name: String::from("Vedansh"),
        age: 30,
    };

    let person2 = person1.clone();  

    println!("person1: {}, {}", person1.name, person1.age);
    println!("person2: {}, {}", person2.name, person2.age);
}



// Serde 

// Serialized & Deserialized

use serde::{Serialize, Deserialize};
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Vedansh Saini"),
        age: 19,
    };

    // Serialize to JSON
    let json_str = serde_json::to_string(&person).unwrap();
    println!("Serialized JSON: {}", json_str);

    // Deserialize from JSON
    let deserialized_person: Person = serde_json::from_str(&json_str).unwrap();
    println!("Deserialized Person: {:?}", deserialized_person);
}


use serde::{Deserialize as DeserializeSerde, Serialize as SerializeSerde};

#[derive(SerializeSerde, DeserializeSerde, Debug, PartialEq)]
struct User {
    username: String,
    password: String
}

#[derive(Debug, PartialEq, SerializeSerde, DeserializeSerde)]
struct MyStruct {
    id: u64,
    data: String,
    v: Vec<u32>,
    user: User
}

fn main() {
    let s = MyStruct {
        id: 32,
        data: String::from("Vedansh"),
        v: vec![1, 2, 3],
        user: User {
            username: String::from("Vedansh"),
            password: String::from("Saini")
        }
    };
    let json_str = serde_yaml::to_string(&s).unwrap();
    print!("{}", json_str);
    let deserialized: MyStruct = serde_yaml::from_str(&json_str).unwrap();
    assert_eq!(deserialized, s);
    println!("done!")    
}



// Borsh

use borsh::{BorshSerialize, BorshDeserialize};

#[derive(BorshSerialize, BorshDeserialize, Debug, PartialEq)]
struct MyStruct {
    id: u64,
    data: String,
    v: Vec<u32>
}

fn main() {
    let original = MyStruct {
        id: 42,
        data: "Hello, Borsh!".into(),
        v: vec![1, 2, 3]
    };

    let mut buffer: Vec<u8> = Vec::new();

    original.serialize(&mut buffer).unwrap();
    
    // Deserialize
    let deserialized = MyStruct::try_from_slice(&mut buffer).unwrap();
    
    assert_eq!(original, deserialized);
    println!("Successfully serialized and deserialized: {:?}", deserialized);
}



// Lifetimes on structs

struct User<'a> {
    name: &'a str
}
fn main() {
   let name = String::from("Vedansh");
   let user = User { name: &name };
   println!("{}", user.name);
}



struct User<'a> {
    name: &'a str
}

impl<'a> User<'a> {
    fn new(name: &'a str) -> Self {
        User { name }
    }
}
fn main() {
   let name = String::from("Vedansh");
   let user = User::new(&name);
   println!("{}", user.name);
}