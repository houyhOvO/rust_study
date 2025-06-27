fn main() {
    // another_function(5);

    let m = five();
    println!("The value of m is: {m}");

    // let x = plus_one(5);
    // println!("The value of x is: {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
