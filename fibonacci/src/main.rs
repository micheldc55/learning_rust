fn main() {
    let x = fibonacci(1, 1, 40);
    println!("Result is: {}", x);
}

fn fibonacci(start_val1: i32, start_val2: i32, num_iter: i32) -> i32 {
    let mut start = start_val1;
    let mut new_val = start_val2;
    let mut i = 0;

    loop {
        let result = start.checked_add(new_val);

        match result {
            Some(val) => {
                start = new_val;
                new_val = val; 
                i += 1;

                if i >= num_iter {
                    break;
                }
            },
            None => {
                println!("Value Overflow at step {}! You can't go over 2,147,483,647 or below -2,147,483,648", i);
                break;
            }
        }

        // println!("Iter: {}, Start: {}, New Value: {}, Result: {:?}", i, start, new_val, result);
    }

    new_val 
}