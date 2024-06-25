fn constant() -> i32 {
    // Set a function that returns a constant value
    const NEW_CONSTANT: i32 = 2 * 2;
    println!("{NEW_CONSTANT}");
    NEW_CONSTANT
}

fn shadowing() {
    // Shadowing a variable
    let x = 5;
    println!("{x}");
    let x = x + 1;
    println!("{x}");
    {
        let x = x * 2;
        println!("This is x in the new scope: {x}");
    }
    println!("This is x after the new scope: {x}");
    let x = x * 3;
    println!("This is doing x * 3 but using the x outside the previous scope: {x}");
}

// fn main() {
//     // Set x to the constant value
//     let x = constant();
//     println!("This is the constant: {x}");
//     let _p = shadowing();
// }

// const TWO: u32 = 1 + 1;
// fn main() {
//   println!("{TWO}");
// }


// fn main() {
//     let mut x: u32 = 1;
//     {
//       let mut x = x;
//       x += 2;
//     }
//     println!("{x}");
//   }


fn main() {
    let mut x: u32 = 1;
    x = "Hello world";
    println!("{x}");
  }