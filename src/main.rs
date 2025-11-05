use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    let mut how_many_to_guess: String = String::new();
    let mut numbers_to_guess: Vec<u8> = Vec::new();
    let mut guess_index: usize = 0;

    println!("How many numbers you wish to guess?");

    io::stdin()
        .read_line(&mut how_many_to_guess)
        .expect("Failed to read line");

    let how_many_to_guess: u8 = match how_many_to_guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error while parsing the number");
            return;
        }
    };

    for _ in 0..how_many_to_guess {
        numbers_to_guess.push(rand::rng().random_range(1..=10));
    }

    println!("Guess the number between 1 and 10!");

    while guess_index < numbers_to_guess.len() {
        let number_to_guess: u8 = numbers_to_guess[guess_index];
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
        };

        match guess.cmp(&number_to_guess) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Number found!");
                guess_index += 1;
            }
        }
    }

    println!("You found all numbers:");
    for number in numbers_to_guess {
        println!("{number}");
    }
}
