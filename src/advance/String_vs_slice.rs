fn main() {
        // String is mutable , growable , owned 
        println!("{}","String is mutable , growable , owned ");

        // String slice is immutable , fixed size , borrowed
        // Slice is immutable , fixed size , borrowed
        println!("{}","Slice is immutable , fixed size , borrowed ");


        let mut name = String::from("Dinesh");
        name.push_str(" MN");
        println!("Name is {}", name);
        name.replace_range(7..9," ");
        println!("Name is {}", name);
        println!("Length :{} Capacity: {} Pointer :{:p} ", name.len(),name.capacity(), name.as_ptr());
    
        let name1 = String::from("Dinesh MN");
        let ans = first_word(name1);
        println!("First word is {}", ans);

}

fn first_word(str: String) -> String {
        let mut ans = String::from("");
        for i in str.chars() {
            if i == ' ' {
                break;
            } 
            ans.push_str(&i.to_string());  // i isnt a string, so we convert it to string 
        }
        return ans;
    }