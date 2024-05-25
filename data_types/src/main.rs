fn main() {
    // Addition
    let x: u8 = 5;
    let y: u8 = 10;
    let sum = x + y;
    println!("Sum: {}", sum);

    // Subtraction (using i8 (signed) to allow negative numbers)
    let x: i8 = 10;
    let y: i8 = 5;
    let difference: i8 = y - x;
    println!("Difference: {}", difference);

    // Multiplication
    let x: u8 = 5;
    let y: u8 = 10;
    let product = x * y;
    println!("Product: {}", product);

    // Floating point Division: (regular division)
    let x: f64 = 10.0;
    let y: f64 = 3.0;
    let quotient: f64 = x / y;
    println!("Quotient: {}", quotient);

    // Integer Division: (truncates the decimal part)
    let x: u8 = 10;
    let y: u8 = 3;
    let quotient: u8 = x / y;
    println!("Quotient: {}", quotient);
}
