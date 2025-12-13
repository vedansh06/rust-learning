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