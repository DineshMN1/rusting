fn main() {
    letn();
    heap_str();
    let x = 10;
    let y = 20;
    println!("Sum is {}", sum(x, y));

    let my_string = String::from("Hello World");

    takes_ownership(my_string);
    //println!("{}", my_string); // this line wont work as my_string is no longer valid
    let my_string = String::from("Hello World");

    let my_string = takes_ownership_and_gives_back(my_string);
    println!("{}", my_string);

    refr();
    let mut sting = String::from("Helloborrw"); // mutable reference , borrows the ownership
    mutborrw(&mut sting);

    let user1 = User {
        name : String::from("Dinesh"),
        age : 20,
        active : true,
    };

    println!("Name : {} , Age : {} , Active : {}", user1.name, user1.age, user1.active);

    let rect1 = Rect {
        width : 20,
        height : 30,
    };
    println!("Area of rectangle is : {}", rect1.area());

    let dir = Direction::North;
    move_dir(dir);

    let input = String::from("Hello SuperMan");
    match find_first_a(input) {
        Some(ind) => println!("First a is at index : {}", ind),
        None => println!("No a found"),
    }
}

fn letn() {
    let greet= String::from( "Hello");  // heap memory of string
    println!("greet :{}  , Length : {} , slice : {} , ",greet , greet.len(), &greet[0..2]); 
    let num = true;
    if num {
        println!("The number is true");
    } else {
        println!("The number is false");
    }

    for i in 1..10{
        print!("{} ",i);
    }

    // iterate over arrays, maps, strings
}

fn heap_str() {  // heap memory of string which is more complex
    let mut s = String::from("Hello");
    println!("{}", s);
    println!("Length {} , Capacity {} , Pointer{:p}", s.len(), s.capacity(), s.as_ptr());

    s.push_str("SUP");
    println!("{}", s);
    println!("Length {} , Capacity {} , Pointer{:p}", s.len(), s.capacity(), s.as_ptr());
    let s1 = s; // uses same pointer of heap memory of s and assigns it to s1 
    println!("{}", s1);
    println!("Length {} , Capacity {} , Pointer{:p}", s1.len(), s1.capacity(), s1.as_ptr());
   // println!("{}", s);
    //println!("Length {} , Capacity {} , Pointer{:p}", s.len(), s.capacity(), s.as_ptr());
// the above line wont work , 
// as s1 takes ownership of s and s is no longer valid
}

fn sum(a: i32, b: i32) -> i32 {    // stack memory which is straight forward
    let c =a + b;
    return  c;
}

fn takes_ownership(s: String) {
    println!("{}", s);
} // s goes out of scope and is dropped

fn takes_ownership_and_gives_back(s: String) -> String {
    println!("{}", s);
    s
} // s goes out of scope and is dropped

fn refr() {
    let s = String::from("Hello");
    let s1 = &s; // s1 is a reference to s     ,  it is valind as long as s is valid
    println!("{}", s1);
    println!("{}", s);
}

fn mutborrw( st:&mut String ) {
    st.push_str("Hello");
    println!("{}", st);
}
//structs 
struct User {
    name :String,
    age : u32,
    active : bool,
}

// methods in structs
struct Rect {
    width : u32,
    height : u32,
}

// impl block is used to define methods in structs
impl Rect {
    fn area(&self) -> u32 {
        return self.width *self.height;
    }
}

// enums
enum Direction {
    North,
    South,
    East,
    West,
}
// pattern matching
fn move_dir(direction: Direction) {
    match direction {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::East => println!("Moving East"),
        Direction::West => println!("Moving West"),
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
