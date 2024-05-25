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
    // Tuples --> Can have multiple types in the same tuple
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

    // Arrays --> Only accepts Single Types and a FIXED number of elements
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    // useful like above when you know the number of elements isn't going to change

    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // type annotation indicates an array of 5 i32 elements
    let arr2 = [3; 5]; // this format creates an array of 5 elements with the value 3 
    let arr3 = [3, 3, 3, 3, 3]; // this is the same as arr2

    // Accessing array elements
    let first_month = months[0];
    println!("First Month: {first_month}");

    let first = arr[0];
    let first2 = arr2[0];
    let first3 = arr3[0];
    println!("First Element of arrays 1, 2, 3 respectively: {first} | {first2} | {first3}");

    let t = ([1; 2], [3; 4]);
    let (a, b) = t;
    println!("{}", a[0] + t.1[0]); 
}
