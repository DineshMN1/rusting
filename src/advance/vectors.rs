fn main() {
    let mut vec = Vec::new();  //creating a new vector
    vec.push(1);
    vec.push(2);

    println!("{:?}", vec);   //implementing debug trait  , we cant print struct , vectors are struct itself

    println!("{:?}", even_vec(vec));
    let number = vec![1,2,3]; //initializing the vector using macros 
    //y 
}

fn even_vec(vec :Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(val);
        }
    }

    return new_vec;
}

