use std::io; // brings io library into scope
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    
    println!("Please input your guess.");

    // "mut" makes variables mutable
    let mut guess = String::new(); //creates an empty instance of a String

    io::stdin() 
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {guess}");
}

// if we hadn't imported the io library, we would have to write std::io::stdin() 
// and instead of just io::stdin().