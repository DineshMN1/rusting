fn main() {
    let name = String::from("SUPER_MAN_D");
    let len = get_string_len(name);
    println!("Length of the string is {}", len);
}

fn get_string_len(s : String) -> usize {
    return s.chars().count();
}