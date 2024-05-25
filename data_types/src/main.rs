fn main() {
    // ** SCALAR TYPES **
    
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

    // Modulus
    let x: u8 = 10;
    let y: u8 = 3;
    let remainder: u8 = x % y;
    println!("Remainder: {}", remainder);

    // Booleans
    let tr = true; // type inferred (no annotation)
    let fa: bool = false; // type declared 
    println!("This is {}", tr);
    println!("This is {}", fa);

    // Character types --> USE SINGLE QUOTES '' | (Strings use double quotes "")
    let letter = 'X'; // type inferred (no annotation)
    let letter_alt: char = 'h'; // explicit type annotation
    let emoji: char = '😻';
    println!("This is the letter: {letter}");
    println!("This is the letter_alt: {}", letter_alt);
    println!("This is the emoji: {emoji}");


    // ** COMPOUND TYPES **
    let tup: (u16, f32, char) = (12, -1.1, 'h');

    // tuple destructuring
    let (a, b, c) = tup;
    println!("{a} | {b} | {c}");

    // tuple access by index
    let other_tuple: (i16, u8, f32) = (-7, 16, 7.3);
    let minus_seven = other_tuple.0;
    let sixteen = other_tuple.1;
    let seven_point_three = other_tuple.2;
    println!(
        "First Element: {} | Second Element: {} | Thrid Element {}", 
        minus_seven, 
        sixteen, 
        seven_point_three
    );
}
