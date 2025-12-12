//? Strings

fn main() {
    let greeting = String::from("hello world");
    println!("{}", greeting);

    // print!("{}", greeting.chars().nth(1000))
}



fn main() {
    let name = String::from("vedansh");
    let len = get_str_len(name);
    println!("the length of the string is {}", len);
}

fn get_str_len(str: String) -> usize {
    return str.chars().count();
}