use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generate random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        // creates mutable variable, bound to new empty instance of String
        let mut guess = String::new();

        // call stdin function from io module to handle user input
        io::stdin()
            .read_line(&mut guess)          // append standard input to string 
            .expect("Failed to read line"); // handle Result variant Err

        // convert String input to number type
        // trim eliminates trailing whitespace, parse converts string to another type
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}