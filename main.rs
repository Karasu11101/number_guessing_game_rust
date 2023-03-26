use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("Welcome to Guess a Number!");

    let number = rand::thread_rng().gen_range(1, 11);


    loop {

        println!("Please input your guess: ");

        let mut guess: String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // _ is a catch all value
        };

        println!("Your guess is {}.", guess);

        match guess.cmp(&number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        };
    }

    println!("Thanks for playing!");
}
