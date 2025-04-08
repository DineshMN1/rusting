fn main() {
    let v1 = vec![1,2,3];

    // let mut v1_iter = v1.iter_mut();
    let v1_iter = v1.iter();  
    println!("{:?}", v1_iter); 

    let first = v1_iter.next();
    let second = v1_iter.next();
    let third = v1_iter.next();
    let fourth = v1_iter.next();       //gives None
    println!("First : {:?} , Second : {:?} , Third : {:?} , Fouth : {:?}", first, second, third, fourth);

    while let Some(val) = v1_iter.next() {
        *val += 1;
        println!("Got {}", val);
    }

 
     for i in v1_iter {
        *i = *i + 1;
         println!("Got {}", i);
     }
    println!("{:?}", v1);
}