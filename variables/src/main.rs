fn constant() -> i32 {
    // Set a function that returns a constant value
    const NEW_CONSTANT: i32 = 2 * 2;
    println!("{NEW_CONSTANT}");
    NEW_CONSTANT

}

fn main() {
    // Set x to the constant value
    let x = constant();
    println!("{x}");
}

// const TWO: u32 = 1 + 1;
// fn main() {
//   println!("{TWO}");
// }