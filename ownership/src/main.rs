fn main() {
    let x: i8 = 42;
    let y: i8 = 10;
    let z: i8 = add_numbers(x, y);

    println!("The sum of {} and {} is {}", x, y, z);

    let s1: String = String::from("hello")
    let s2: String = s1  // This will not compile because Rust is transfering 
                         // ownership of the data from s1 to s2, and s1 is dropped
                         // If we want to do it anyways, we can do s2 = s1.clone()
                         // This assigns s1 and s2 to two different pointers in heap memory

    println!("s1: {s1} | s2: {s2}")
}

fn add_numbers(a: i8, b: i8) -> i8 {
    let c: i8 = a + b;
    c
}
