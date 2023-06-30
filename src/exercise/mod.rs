pub mod one {

    use std::io;

    use crate::helpers;

    pub fn convert_fah_cel(fahrenheit: f64) -> f64 {
        ((fahrenheit - 32.0) * 5.0) / 9.0
    }

    pub fn fibonacci(terms: usize) -> Vec<u32> {
        let mut items = vec![0, 1];
        let mut index = 2;
        while index < terms {
            items.push(items[index - 2] + items[index - 1]);
            index += 1;
        }

        items
    }

    pub fn main() {
        unit_conversion();

        fibonacci_program();

        // TODO: here
        helpers::spaces(2);
        println!("3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.");
    }

    fn unit_conversion() {
        let mut fahrenheit = String::new();

        helpers::spaces(1);
        println!("1. Convert temperatures between Fahrenheit and Celsius.");

        helpers::spaces(1);
        println!("Enter a fahrenheit: ");
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Please enter a fahrenheit with decimal places!");

        println!(
            "The celsius of fahrenheit {fahrenheit} is {:.3}",
            convert_fah_cel(fahrenheit.trim().parse().unwrap())
        );
    }

    fn fibonacci_program() {
        helpers::spaces(1);
        let mut terms = String::new();
        println!("2. Generate the nth Fibonacci number.");

        helpers::spaces(1);
        println!("Input number of terms: ");
        io::stdin().read_line(&mut terms).expect("Input a number!");

        helpers::spaces(1);

        let terms = terms.trim().parse().unwrap();
        for item in fibonacci(terms) {
            print!("{item} ");
        }
    }
}

pub mod guessing_games {
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
}
