use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Willkommen zum NumberGuessingGame!\n");

    let rnd_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Bitte gebe eine Zahl ein: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Es konnte die Eingabe nicht gelesen werden.");

        let guessed_num: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Deine Zahl: {}", guessed_num);

        match guessed_num.cmp(&rnd_num) {
            Ordering::Less => println!("{}", "Die eingegebene Zahl ist zu klein!".red()),
            Ordering::Greater => println!("{}", "Die eingegebene Zahl ist zu groß!".red()),
            Ordering::Equal => {
                println!("{}", "Richtig geraten!".green());
                break;
            }
        };
    }

}
