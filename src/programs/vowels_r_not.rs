fn main() {
    let name = String::from("SUPER_MAN_D");
    println!("number of vowels present in the name is {}", vowels(&name));
  }
  
  fn vowels(name: &str) -> i32{
    let vowels = ['a','e','i','o','u','A','E','I','O','U'];
    let mut count = 0;
    for (index, char) in name.chars().enumerate() {
      if vowels.contains(&char) {
        count += 1;
      }
  }
    count;
  }