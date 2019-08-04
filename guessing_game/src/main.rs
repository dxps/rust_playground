extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("\n---------- Guess the number ----------\n");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        print!("Enter your guessing: ");
        std::io::stdout().flush().unwrap();
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error: Failed to read line.");
        if guess.trim() == "quit" {
            println!("\nGoodbye\n");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!();
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Nope, it's greater.\n"),
            Ordering::Greater => println!("Nope, it's smaller.\n"),
            Ordering::Equal => {
                println!("Great! You guessed it!\n");
                break;
            }
        }
    }
}
