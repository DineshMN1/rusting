use std::collections::HashMap;

fn group_by_values(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = Hashmap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}

fn main() {
    let inpu_vec = vec![(String::from("Dinesh"), 20), (String::from("Sun"), 20)];
    let mut hm = group_by_values(inpu_vec);
    println!("{:?}", hm);
}