use std::{cmp::Ordering, io::{self, Read}};

use rand::Rng;


fn main() {
    println!("Guess the no btwn 1..=10");
    let secret_no = rand::thread_rng().gen_range(1..=10);

    loop{
        println!("Please input your guess ..");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed .. {}",guess);
        match guess.cmp(&secret_no) {
            Ordering::Less => println!("Too Small !"),
            Ordering::Greater => println!("Too Big !"),
            Ordering::Equal => {
                println!("You won !");
                break;
            }
            
        }
    }
}
