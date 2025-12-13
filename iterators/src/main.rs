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