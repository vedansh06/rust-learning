fn main() {
    println!("Hello, world!");
}



fn main() {
    println!("{}", is_even(20))
}
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}



fn main() {
    println!("{}", fib(4));
}
// 0 1 1 2 3
fn fib(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;

    if num == 0 {
        return first;
    }

    if num == 1 {
        return second;
    }

    for _ in 0..(num - 1) {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}



fn main() {
    let name = String::from("vedansh");
    let len = get_str_len(name);
    println!("the length of the string is {}", len);
}

fn get_str_len(str: String) -> usize {
    return str.chars().count();
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



struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self, num: i32) -> i32 {
        2 * (self.width + self.height)
    }

    fn debug() -> i32 {
        return 1;
    }
}

fn main() {
    let rect1 = Rect {
        width: 10,
        height: 20,
    };

    println!("area is {}", rect1.area());
    println!("perimeter is {}", rect1.perimeter(1));
    println!("debug is {}", Rect::debug());
}



enum Shape {
    circle(f64),
    square(f64)
    rectangle(f64, f64),
}

fn calculate_area(shape: Shape) -> f64 {
match shape{
    Shape::circle(radius) => std::f64::consts::PI * radius*radius
    Shape::square(side_length) => side_length * side_length
    Shape::rectangle(width, height) => width * height
}
}

fn main(){
    let circle = Shape::circle(5.0);
    let square= Shape::square(4.0);
    let rectangle= Shape::rectangle(3.0,6.0);
}



enum Direction{
    North,
    South,
    East,
    West
}

fn main(){
    ley my_direction = Direction::North;
    let new_direction = my_direction;
    move_around(new_direction)
}

fn move_around(direction: Direction){

}



enum Shape {
    Rectangle(f64, f64), 
    Circle(f64),         
}

fn main() {
    let rect = Shape::Rectangle(1.0, 2.0);
    calculate_area(rect);
    let circle = Shape::Circle(1.0);
    calculate_area(circle);
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(r) => 3.14 * r * r,
    }
}



fn main() {
    let index = find_first_a(String::from("vedansh"));

    match index {
        Some(value) => println!("index is {}", value),
        None => println!("a not found"),
    }
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }

    return None;
}



use std::fs::read_to_string;

fn main() {
    let ans = read_from_file_vedansh(String::from("a.txt"));
}

fn read_from_file_vedansh(file_path: String) -> Result<String, String> {
    let result = read_to_string(file_path); // Result
    match result {
        Ok(data) => Ok(data),
        Err(err) => Err(String::from("File not read")),
    }
}



use std::fs;

fn main(){
    let greeting_file_result = fs:: read_to_string("hello.txt");
    match greeting_file_result{
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        }
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
}



use chrono::Utc;

fn main() {
    let now = Utc::now();
    println!("current time is {}", now);
}



use chrono::{Local,Utc};

fn main(){
    let now = Utc::now();
    println!("Current date and time in UTC:{}", now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time: {}", formatted);

    let local = Local::now();
    println!("Current date and time in local: {}", local)
}



fn main() {
    let mut s1 = String::from("vedansh");

    s1 = do_something(s1);
    println!("number is {}", s1);
}

fn do_something(s2: String) -> String {
    println!("{}", s2);
    return s2;
}



fn main() {
    let mut s1 = String::from("vedansh");
    do_something(&mut s1);
    println!("number is {}", s1);
}

fn do_something(s2: &mut String) {
    s2.push_str(" saini");
    println!("{}", s2); 
}



use std::thread;

fn main() {
    thread::spawn(|| {
        let mut c = 0;
        for i in 0..500000000 {
            for j in 0..500000000 {
                c = c + 1;
                c = c - 1;
            }
        }
    });

    thread::spawn(|| {
        let mut c = 0;
        for i in 0..500000000 {
            for j in 0..500000000 {
                c = c + 1;
                c = c - 1;
            }
        }
    });

    let mut c = 0;
    for i in 0..500000000 {
        for j in 0..500000000 {
            c = c + 1;
            c = c - 1;
        }
    }
}



use std::{
    sync::mpsc,
    thread::{self, spawn},
};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..10 {
        let producer = tx.clone();
        spawn(move || {
            let mut sum: u64 = 0;
            for j in i * 10000000..(i + 1 * 10000000) - 1 {
                sum = sum + j;
            }
            producer.send(sum).unwrap();
        });
    }
    drop(tx);

    let mut final_sum: u64 = 0;
    for val in rx {
        println!("receive value from thread");
        final_sum = final_sum + val;
    }
    println!("{}", final_sum);
}



fn create_string(){
    let s:String = String::from("Hello, heap memory leak");

    println!("{}", s);
}

fn main(){
    create_string();
}