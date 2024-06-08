use std::io;

fn main() {
    println!("Input temperature in Celsius:");

    let mut celsius = String::new();

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: f32 = match celsius.trim().parse() {
        Ok(num) => num,                 // If parsing is successful, use the number
        Err(_) => {                     // If parsing fails, handle the error
            println!("Please type a number!");
            return;                     // Exit the function early
        }
    };

    let fahrenheit: f32 = celsius_to_fahrenheit(celsius);

    println!("{fahrenheit}")
}

fn celsius_to_fahrenheit(temp: f32) -> f32 {
    9.0 * temp / 5.0 + 32.0
}
