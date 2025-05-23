use std::collections::HashMap;

fn main() {
    let mut users = HashMap::new();
    
    users.insert(String::from("Dinesh"),20);
    users.insert(String::from("Sun"),20);

    let first_user = users.get("Sun");

    match first_user {
        Some(data) => println!("{}", data),
        None => println!("User not found"),
    }    
}


// Methods 1) Insert, 2) Get, 3) Remove, 4) Clear 
// HashMap is a collection of key-value pairs