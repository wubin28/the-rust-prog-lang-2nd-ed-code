use std::io;

fn main() {
    // variables are immutable by default
    // let x = 5;
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing vs mut
    let mut spaces = "   ";
    //spaces = spaces.len(); // expected `&str`, but found `usize`

    let mut spaces = "   ";
    let spaces = spaces.len();
    //spaces = 1; // Cannot assign a new value to an immutable variable more than once [E0384]

    let spaces = "   ";
    let mut spaces = spaces.len();
    // spaces = "1"; // expected `usize`, but found `&str`

    // The tuple type
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // To access each element of the tuple directly
    let x: (i32, f64, u8) = (501, 7.4, 2);
    let five_hundred_and_one = x.0;

    let seven_point_four = x.1;

    let two = x.2;
    println!("The value of x.1 is: {}", x.1);

    // invalid array element access
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
