fn main() {
    let v1 = vec![1,2,3];

    // let mut v1_iter = v1.iter_mut();
    let v1_iter = v1.into_iter();  // takes ownership of v1 and moves it into the iterator
    println!("{:?}", v1_iter);    
    let sum :i32 = v1_iter.sum();  // sum takes ownership of the iterator and consumes it
    println!("Sum : {}", sum);  // prints 6   // it takes self as a argument 
    // if it takes &self as a argument then it will not consume the iterator


    let v1 = vec![1, 2, 3]; // recreate the vector since v1 was moved
    let iter2 = v1.into_iter().map(|x| x + 1);
    let v2 :Vec<i32> = iter2.collect();
    println!("{:?}", v2);  // prints the transformed vector
   
} 