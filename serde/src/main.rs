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
