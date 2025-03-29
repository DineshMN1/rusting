fn main() {
    
//borrow
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("s2 : {}",s2 );
    // println!("s1 : {}",s1 );         {cant be print becuse s1 is borrwed by s2}

// borrow reference
    let s3 = String::from("Borrow-ref");
    let s4 = &s3;
    println!("s3 : {}",s3);
    println!("s4 : {}",s4);   // can be print because is just carry the refernece

// borrow mutuable reference
    let mut s6 = String::from("Mut-refernece");
    mut_ref(&mut s6);
    println!("s6 : Mut-ref : {}",s6);

}

fn mut_ref(s5:&mut String) {
    s5.push_str(" Borrowing");
    println!("s5 : {}",s5);
}

// Multiple ref is valid

// Only one mutable ref is valid