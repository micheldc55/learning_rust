fn main() {
    let vec: &[usize] = &[1, 4, 6, 12, 25, 25, 26, 127, 1001];

    let index: Option<usize> = binary_search(vec, 50);

    match index {
        Some(val) => {
            let value_found: usize = vec[val];
            println!("Found element at index = {val} --> vec[{val}] = {value_found}")
        }, 
        None => {
            println!("Value wasn't found")
        }
    }
}

fn binary_search(arr: &[usize], target: usize) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut low: usize = 0;
    let mut high: usize = arr.len() - 1;

    while low <= high {
        let mid: usize = low + (high - low) / 2;
        let mid_val: usize = arr[mid];

        if mid_val == target {
            return Some(mid);
        } else if mid_val < target {
            low = mid + 1;
        } else {
            if mid == 0 {
                break; 
            }
            high = mid - 1;
        }
    }

    None
}