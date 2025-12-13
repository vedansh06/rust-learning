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