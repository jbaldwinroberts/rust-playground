extern crate rand;

use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1, 101);

    let mut tries = 0;

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim()
            .parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        tries += 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won in {} tries!", tries);
                break;
            }
        }
    }
}
