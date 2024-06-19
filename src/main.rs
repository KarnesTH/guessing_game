use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// This is the main function of the Guessing Game.
/// It initializes a random number between 1 and 100, then enters a loop where it prompts the user to guess the number.
/// After each guess, it provides feedback whether the guessed number is too high, too low, or correct.
fn main() {
    println!("Willkommen zum Guessing Game!\n");

    // Generate a random number between 1 and 100
    let rnd_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Bitte gebe eine Zahl ein: ");

        // Initialize a mutable string to store the user's guess
        let mut guess = String::new();

        // Read the user's input from the standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Es konnte die Eingabe nicht gelesen werden.");

        // Parse the user's input into a u32 and handle any parsing errors
        let guessed_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, skip to the next iteration of the loop
        };

        println!("Deine Zahl: {}", guessed_num);

        // Compare the guessed number with the random number and provide feedback
        match guessed_num.cmp(&rnd_num) {
            Ordering::Less => println!("{}", "Die eingegebene Zahl ist zu klein!".red()),
            Ordering::Greater => println!("{}", "Die eingegebene Zahl ist zu groß!".red()),
            Ordering::Equal => {
                println!("{}", "Richtig geraten!".green());
                break; // If the guessed number is correct, exit the loop
            }
        };
    }
}
