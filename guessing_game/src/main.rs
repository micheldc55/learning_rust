use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // If an error occurs, the program will crash and display the message.

    let guess: u32 = guess.trim().parse().expect("Please input a number :("); // If an error occurs, the program will crash and display the message.

    println!("You guessed: {guess}");

    // Other ways to format the string
    // let x: i32 = 7;
    // println!("x is equal to: {}", x);
    // println!("x plus two is: x + 2 = {}", x + 2);
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Number is too small"),
        Ordering::Equal => println!("Success!!"),
        Ordering::Greater => println!("Number is too large"),
    }
}