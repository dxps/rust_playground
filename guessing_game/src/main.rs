extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("---------- Guess the number ----------");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Enter your guessing: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}", err);
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Nope, it's smaller.\n"),
            Ordering::Greater => println!("Nope, it's greater.\n"),
            Ordering::Equal => {
                println!("Great! You guessed it!\n");
                break;
            }
        }

    }
    
}
