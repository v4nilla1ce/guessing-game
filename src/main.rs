use rand::Rng;
use std::io::{self, Write}; // ✅ Import `Write` trait

fn main() {
    println!("🎯 Welcome to the Number Guessing Game!");
    println!("🔢 Guess a number between 1 and 100!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        // Get user input
        print!("Enter your guess: ");
        io::stdout().flush().unwrap(); // ✅ Now flush() works

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // Convert input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Please enter a valid number!");
                continue;
            }
        };

        // Compare guess with secret number
        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("📉 Too low! Try again."),
            std::cmp::Ordering::Greater => println!("📈 Too high! Try again."),
            std::cmp::Ordering::Equal => {
                println!("🎉 You guessed it! The number was {}", secret_number);
                break;
            }
        }
    }
}
