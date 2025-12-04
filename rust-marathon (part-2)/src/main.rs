fn main() {
    println!("Hello, world!");
}



// Vectors

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    print!("{:?}", vec)
}



fn even_values(v: &mut Vec<i32>) {
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 != 0 {
            v.remove(i);
        } else {
            i += 1
        }
    }
}
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    even_values(&mut vec);
    println!("Updated vector is {:?}", vec)
}



fn even_values(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for value in v {
        if value % 2 == 0 {
            new_vec.push(value)
        }
    }
    return new_vec;
}
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    let new_vec = even_values(vec);
    println!("Updated vector is {:?}", new_vec)
}



fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers {
        print!("{}", number)
    }
}



fn main(){
    let numbers : Vec<i32> = vec![1,2,3];
    for number in numbers {
        print!("{}" , number)
    }
}



// Hashmaps

use std::collections::HashMap;

fn main(){
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("vedansh"), 19);
    users.insert(String::from("saini"), 20);

    let user1 = users.get("vedansh");

    print!("{}", user1.unwrap())
}



use std::collections::HashMap;

fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for (key, value) in pairs {
        map.insert(key, value);
    }
    return map;
}



// Iterators

fn main(){
    let nums = vec![1,2,3,];

    for value in nums {
        print!("{}",value)
    }
}



fn main(){
    let nums = vec![1,2,3];
    let iter = nums.iter();

    for value in iter {
        println!("{}", value)
    }
}



fn main() {
    let nums = vec![1,2,3];
    let mut iter = nums.iter();

    while let Some(val) = iter.next() {
        print!("{}" , val);
    }
}



fn main() {
    let mut nums = vec![1,2,3];
    let iter = nums.iter_mut();

    for value in iter {
        *value = *value + 1;
    }
    println!("{:?}" , nums)
}



fn main() {
    let nums = vec![1,2,3];
    let iter = nums.into_iter();

    for  value in iter {
        println!("{}" , value)
    }
}



fn main() {
    let v1: Vec<i32> = vec![1,2,3];
    let iter = v1.iter();
    let iter2 = iter.map(|x| x + 1);
    for x in iter2 {
        println!("{}" , x)
    }
}



fn main() {
    let v1: Vec<i32> = vec![1,2,3];
    let iter = v1.iter();
    let iter2 = iter.filter(|x| *x % 2 == 0);
    for x in iter2 {
        println!("{}" , x)
    }
}



fn filter_and_map(v: Vec<i32>) -> Vec<i32> {
    let new_iter = v.iter().filter(|x| *x % 2 == 1).map(|x| x + 1);
    let new_vec: Vec<i32> = new_iter.collect();
    return new_vec
}

fn main() {
    let v1: Vec<i32> = vec![1,2,3];
    let ans = filter_and_map(v1);
    println!("{:?}" , ans)
}



use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Alice" , 50);
    scores.insert("Bob" , 40);
    scores.insert("Charlie" , 30);

    println!("Iterating over key-value pairs:");
    for (key, value) in scores.iter() {
        println!("{}: {}", key, value);
    }

    println!("\nIterating over mutable key-value pairs:");
    for(key, value) in scores.iter_mut() {
        *value += 10;
        println!("{}: {}" , key, value)
    }
}



// Strings vs Slices

fn main() {
    let name = String::from("Vedansh");
    println!(" my name is {}" , name);
}



fn main() {
    let mut name = String::from("Vedansh");
    name.push_str(" Saini");
    println!("my name is {}" , name)
}



fn main() {
    let mut name = String::from("Vedansh");
    name.push_str(" Saini");
    println!("name is {}" , name);
    name.replace_range(8..name.len(), "");
    println!("name is {}" , name);
}



fn main() {
    let s = String::from("hello world");
    println!("{}" , s)
}



fn main() {
    let name = String::from("hello world");
    let ans = first_word(name);
    println!("ans is {}" , ans)
}

fn first_word(str: String) -> String {
    let mut ans = String::from("");
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&1.to_string());
    }
    return ans
}



fn main() {
    let name = String::from("hello world");
    let ans = first_word(&name);
    println!("ans is {}" , ans);
}

fn first_word(str: &String) -> &str {
    let mut space_index = 0;
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        space_index = space_index + 1;
    }
    return &str[0..space_index];
}



fn main() {
    let name = String::from("hello world");
    let mut space_index = 0;
    for i in name.chars() {
        if i == ' ' {
            break;
        }
        space_index = space_index + 1
    }
    let ans = &name[0..space_index];
    println!("ans is {}" , ans)
}



fn main() {
    let name = String::from("hello world");
    let string_slice = &name;
    let string_literal = "hello";
}



fn main() {
    let arr = [1,2,3];
    let arr_slice = &arr[0..1];
}



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



use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
     T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



fn longest(a: String, b:String) -> String {
    if a.len() > b.len() {
        return a ;
    } else {
        return b;
    }
}

fn main() {
    let longest_str;
    let str1 = String::from("small");
    let str2 = String::from("longer");
    longest_str = longest(str1, str2);
    println!("{}" , longest_str)
}



fn longest(a: String, b:String) -> String {
    if a.len() > b.len() {
        return a ;
    } else {
        return b;
    }
}

fn main() {
    let longest_str;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        longest_str = longest(str1, str2);
    }
    println!("{}" , longest_str)
}



fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() > second.len() {
        return first;
    } else {
        return second;
    }
}

fn main() {
    let longest_str;
    let str1 = String::from("small");
    {
        let str2 = String::from("longer");
        longest_str = longest(&str1, &str2);
    }
    println!(" The longest string is {}", longest_str);
}



struct User <'a> {
    name: &'a str
}

fn main() {
    let first_name = String::from("Vedansh Saini");
    let user = User { name: &first_name};
    println!("The name of the user is {}", user.name);
}



struct User<'a, 'b> {
    first_name: &'a str,
    last_name: &'b str,
}

fn main() {
    let user: User;
    let first_name = String::from("Vedansh");
    {
        let last_name = String::from("Saini");
        user = User {first_name: &first_name, last_name: &last_name};
    }
    println!("The name of the user is {}", user.first_name)
}



// Multithreading

use std::thread;
use std::time::Duration;

fn main() {
    let sum = 0;
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }    
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}



use std::thread;

fn main() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    
    handle.join().unwrap();
}


// Message Passing

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}



use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    for  i in 0..10 {
        let producer = tx.clone() ;
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..10000000 {
                ans = ans + (i * 10000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }

    drop(tx);

    let mut ans: u64 = 0;
    for val in rx {
        ans = ans +  val;
        println!("found value");
    }
    println!("Ans is {}", ans)
}