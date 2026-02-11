use std::io;
use rand::prelude::*;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let mut rng = rand::rng();
    let random_number = rng.random_range(1..=100);

    loop {
            let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("Equal"); 
                break;
            },
        }
    }
}
