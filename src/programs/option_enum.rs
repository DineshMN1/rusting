fn main() {
    let s = String::from("Super_Man_D");
    match find_first_a(s) {
        Some(ind) => println!("First 'a' found at index {}", ind),
        None => println!("No 'a' found"),
    }
}

// Option enum 
fn find_first_a(s: String) -> Option<i32> {
    for (ind,c) in s.chars().enumerate() {
        if c == 'a' {
            return Some(ind as i32);
        }
    }
    return None ;
}