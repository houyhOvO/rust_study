fn main() {
    // let x = 5;   // x is an immutable variable
    // println!("The value of x is: {x}");
    // // x = 6;       // cannot assign twice to x
    // println!("The value of x is: {x}");

    let mut x = 5;   // x is a mutable variable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // declare a constant

    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");
}