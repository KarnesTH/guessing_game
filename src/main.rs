use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// A simple number guessing game implemented in Rust.
///
/// This program generates a random number between 1 and 100 and prompts the user to guess the number.
/// After each guess, the program provides feedback indicating whether the guessed number is too low,
/// too high, or correct. The game continues until the user guesses the correct number.
fn main() {
    println!("Welcome to Rust Guessing Game!");

    let rnd_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter a number: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        let guessed_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your number: {}", guessed_num);

        match guessed_num.cmp(&rnd_num) {
            Ordering::Less => println!("{}", "Your given number is to low!".red()),
            Ordering::Greater => println!("{}", "Your given number is to high!".red()),
            Ordering::Equal => {
                println!("{}", "Yeah, your right!".green());
                break;
            }
        };
    }
}
