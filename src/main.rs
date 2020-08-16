fn main() {
    println!("Hello, world!");
    let nums = vec![5, 23, 6, 435, 234, 234, 5654];

    let found_index = binary_search(&nums, &5654).unwrap();
    println!("found at index {}", found_index);

    let empty = binary_search(&nums, &21);
    println!("empty: {}", empty.is_none());
}

fn binary_search<T: PartialEq + PartialOrd>(vec: &Vec<T>, item: &T) -> Option<usize> {
    let mut low: usize = 0;
    let mut high = vec.len() - 1;
    let mut mid = (high + low) / 2;

    while low <= high {
        let current = vec.get(mid).unwrap();
        if current == item {
            return Some(mid);
        } else if item < current {
            // guess was too high
            high = mid - 1;
        } else {
            // guess was too low
            low = mid + 1;
        }
        mid = (low + high) / 2;
        println!("low: {}, high: {}, mid: {}", low, high, mid);
    }
    None
}