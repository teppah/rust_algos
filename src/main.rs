use std::ops::Add;

fn main() {
    let mut nums = vec![5, 23, 6, 45535, 234, 234, 5654];

    selection_sort(&mut nums);
    println!("{:?}", nums);

    let found_index = binary_search(&nums, &234).unwrap();
    println!("found at index {}", found_index);

    let empty = binary_search(&nums, &21);
    println!("empty: {}", empty.is_none());

    let sum = sum_dc(&nums);
    println!("sum is {}", sum);

    let count = count_dc(&nums);
    println!("count is {}", count);

    let largest = max_dc(&nums);
    println!("max is {}", largest);
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
    }
    None
}

fn selection_sort<T: PartialOrd>(vec: &mut [T]) {
    for i in 0..vec.len() {
        let (_, index) = smallest(&vec[i..vec.len()]);
        vec.swap(i, index + i);
    }
}

fn smallest<T: PartialOrd>(vec: &[T]) -> (&T, usize) {
    let mut smallest = vec.get(0).unwrap();
    let mut index: usize = 0;
    for (i, current) in vec.iter().enumerate() {
        if current < smallest {
            smallest = current;
            index = i;
        }
    }
    (smallest, index)
}

// watch out for Add impl with generics
fn sum_dc<T: Add<Output=T> + Copy>(values: &[T]) -> T {
    return if values.len() == 1 {
        values[0]
    } else {
        values[0] + sum_dc(&values[1..])
    };
}

fn count_dc<T>(values: &[T]) -> usize {
    return if values.len() == 1 {
        1
    } else {
        1 + count_dc(&values[1..])
    };
}

fn max_dc<T: PartialOrd + Copy>(values: &[T]) -> &T {
    if values.len() == 1 {
        return &values[0];
    }
    let current = &values[0];
    let next = max_dc(&values[1..]);
    return if current > next { current } else { next };
}