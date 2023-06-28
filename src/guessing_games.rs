use rand::Rng;
use std::{cmp::Ordering, io};

pub fn start() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    let mut attempt: u32 = 0;
    println!("Secret number is {secret_number}");

    loop {
        attempt += 1;

        println!("Your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        print!("You guessed: {guess}\n");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win :)");
                break;
            }
        }
    }

    println!("{attempt} attempt(s)");
}
