fn main() {
    println!("Hello, world!");
    let mut nums = vec![5, 23, 6, 435, 234, 234, 5654];

    let found_index = binary_search(&nums, &5654).unwrap();
    println!("found at index {}", found_index);

    let empty = binary_search(&nums, &21);
    println!("empty: {}", empty.is_none());
    selection_sort(&mut nums);
    println!("{:?}", nums);
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

fn selection_sort<T: PartialOrd>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let (smallest, index) = smallest(&vec[i..vec.len()]);
        vec.swap(i, index + i);
    }
}

fn smallest<T: PartialOrd>(vec: &[T]) -> (&T, usize) {
    let mut smallest = vec.get(0).unwrap();
    let mut index: usize = 0;
    for i in 0..vec.len() {
        let current = vec.get(i).unwrap();
        if current < smallest {
            smallest = current;
            index = i;
        }
    }
    (smallest, index)
}