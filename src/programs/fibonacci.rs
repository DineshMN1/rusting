fn main() {
    println!("{}", fib(12));
}

fn fib(num : i32) -> i32 {
    let mut first = 0;
    let mut second= 1;

    if num == 0 {
        return first;
    } 
    
    if num == 1 {
        return second;
    } 

    for _ in 0..(num - 1) {
        let temp = first + second;
        first = second;
        second = temp;
    }
    return second;
}
