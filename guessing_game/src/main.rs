use std::io;
use rand::Rng;
use std::cmp;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let secret_number = generate_random_number();

    loop {
        // By default rust creates immutable variables
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;}
        }

        println!("You guessed: {}", guess);
    }
}

fn generate_random_number() -> u32 {
    // Generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);
    secret_number
}