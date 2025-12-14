// Lifetimes

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
 fn main() {
    println!("Program started");
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
        first
    } else {
        second
    }
}

fn main() {
    let str1 = String::from("small");
    let str2 = String::from("longer");

    let longest_str = longest(&str1, &str2);

    println!("The longest string is {}", longest_str);
}



fn longest(first: &str, second: &str) -> String {
    if first.len() > second.len() {
        first.to_string()
    } else {
        second.to_string()
    }
}

fn main() {
    let str1 = String::from("small");
    let longest_str;

    {
        let str2 = String::from("longer");
        longest_str = longest(&str1, &str2);
    }

    println!("The longest string is {}", longest_str);
}
