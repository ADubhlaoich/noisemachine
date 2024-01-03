use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("We're going to try a guessing game.");    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Type a number.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
     
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The target number is bigger."),
            Ordering::Greater => println!("The target number is smaller."),
            Ordering::Equal =>  {
                println!("Correct. An exact match!");
                break;
            }
        }
    }
}
