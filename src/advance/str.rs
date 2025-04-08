use std::string;

fn main() {
    let word =  String::from("Hello World");

    // let word1 = &word[0..5];

    // println!("Word1 : {}", word1);
    let word2 = find_first_word(&word);
    println!("Word2 : {}", word2);


    // 3 types of commonly used 
    // 1. String : mutable , growable , owned
    // 2. String slice : immutable , fixed size , borrowed
    // 3. String literal : immutable , fixed size , borrowed
    // String literal is also &str but it points directly to an address

    let name = String::from("Dinesh MN");
    let string_slice = &name[0..6];    // string slice
    let string_literal = "Dinesh MN"; // string literal
    
    println!("Name : {}", name);
    println!("String slice : {}", string_slice);
    println!("String literal : {}", string_literal);

} 

fn find_first_word(word : &String) -> &str {
    let mut index = 0;
    for (_, i) in word.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
     }
     return &word[0..index];
}