//* Stack vs heap

fn main() {
    stack_fn();
    heap_fn();
    update_string();
}

fn stack_fn() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);

    s.push_str(" and some additional text");
    println!("After update: {}", s);
}
