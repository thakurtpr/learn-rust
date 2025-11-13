fn main() {
    // Ownership - Chapter 4
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved, no longer valid
    println!("s2: {}", s2);
    // println!("s1: {}", s1); // This would error

    let s3 = String::from("world");
    take_ownership(s3); // s3 moved into function
    // println!("s3: {}", s3); // Error

    let x = 5;
    makes_copy(x); // i32 has Copy trait, not moved
    println!("x: {}", x);

    // References and Borrowing - Chapter 4
    let s4 = String::from("hello");
    let len = calculate_length(&s4); // Borrow s4
    println!("Length of '{}' is {}", s4, len); // s4 still valid

    // Mutable References - Chapter 4
    let mut s5 = String::from("hello");
    change(&mut s5);
    println!("s5: {}", s5);

    // Lifetimes - Chapter 10 (simple example)
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

fn take_ownership(some_string: String) {
    println!("some_string: {}", some_string);
} // some_string goes out of scope and is dropped

fn makes_copy(some_integer: i32) {
    println!("some_integer: {}", some_integer);
} // some_integer goes out of scope, but no drop

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
