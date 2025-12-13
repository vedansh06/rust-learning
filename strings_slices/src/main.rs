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