fn main() {
    // Hello World - from Rust docs Chapter 1
    println!("Hello, world!");

    // Variables and Mutability - Chapter 3
    let x = 5;
    println!("The value of x is: {}", x);

    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 15;
    println!("The value of y is: {}", y);

    // Data Types - Chapter 3
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = 'A';

    println!("Integer: {}, Float: {}, Bool: {}, Char: {}", integer, float, boolean, character);

    // Functions - Chapter 3
    let result = add(5, 3);
    println!("5 + 3 = {}", result);

    // Control Flow - Chapter 3
    if result > 5 {
        println!("Result is greater than 5");
    } else {
        println!("Result is not greater than 5");
    }

    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            break;
        }
        println!("Counter: {}", counter);
    }
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
