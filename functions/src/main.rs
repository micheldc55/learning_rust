fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_parameters(101);

    multi_param_function(17.5, 2.0);

    println!("Expressions:\n");

    calling_functions(6);

    expression_in_scope(12);

    let area = return_something(2);

    println!("The area calculated is {area} cm2");
}

// functions can be defined anywhere in the file
fn another_function() {
    println!("Another function.");
}

// function parameters have mandatory type annotations
fn another_function_with_parameters(x: i32) {
    println!("The value of x is: {}", x);
}

// functions can have multiple parameters
fn multi_param_function(x: f32, y: f32) {
    let ratio = x / y;
    println!("The ratio of x and y is: {}", ratio);
}

// *** EXPRESSIONS AND STATEMENTS ***
// This line is commented because it won't compile
// statements like "let" don't return a value, so they cannot be assigned 
// fn statement () {
//     let x = (let y = 6);
// }

// An expression is something that evaluates to a value.
// Operations, Calling functions, calling macros are all expressions
fn expressions(num: i8) -> i8 {
    num + 2
}

fn calling_functions(num: i8) {
    let plus_two = expressions(num); // expression

    println!("Result: {plus_two}")
}

// Also, a new scope inside curly brackets is an expression
fn expression_in_scope(num: u8) {
    let result = {
        let mut p = num as i16;
        p = -p;
        p
    };

    println!("Changed the sign of {num} to {result}")
}

// Return statements --> Functions that have a return have to type hint the return
// The last EXPRESSION of a function is returned. We omit the semicolon for the return
fn return_something(radius: i8) -> f32 {
    let area = radius as f32 * 3.141593;
    area
}