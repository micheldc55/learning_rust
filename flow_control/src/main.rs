use std::io;

fn main() {
    println!("Please input your guess.");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess: {guess}");

    let guess: i8 = guess.trim().parse().expect("Please type a number!");

    parse_guess(guess);
}

fn parse_guess(num: i8) {
    if num < 5 {
        println!("The number is lower than 5!");
    } else if num == 5 {
        println!("The number is exactly 5!");
    } else {
        println!("The number is greater than 5!")
    }
}
