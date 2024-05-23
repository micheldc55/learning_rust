use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {    println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // If an error occurs, the program will crash and display the message.

        // trim() removes spaces and new lines (in this case the user input will have a new line character at the end, 
        // so we need to remove it to convert the string to a number)
        // parse() converts the string to a number
        // expect() is used to handle errors
        let guess: u32 = match guess.trim().parse() {
            
        }

        println!("You guessed: {guess}");

        // Other ways to format the string
        // let x: i32 = 7;
        // println!("x is equal to: {}", x);
        // println!("x plus two is: x + 2 = {}", x + 2);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Number is too small"),
            Ordering::Greater => println!("Number is too large"),
            Ordering::Equal => {
                println!("Success!!");
                break;
            }
        }
    }
}