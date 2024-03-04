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
}
