use std::fmt::Debug;

pub fn quicksort<T: PartialOrd + Debug>(items: &mut [T]) {
    do_quicksort(items, 0, items.len());
}

fn do_quicksort<T: PartialOrd + Debug>(items: &mut [T], low: usize, high: usize) {
    if low < high {
        let partition_index = partition_qs(items, low, high);
        do_quicksort(items, low, partition_index);
        do_quicksort(items, partition_index + 1, high);
    }
}

// assume high is exclusive
fn partition_qs<T: PartialOrd + Debug>(items: &mut [T], low: usize, high: usize) -> usize {
    let mut swap_index: usize = low;
    let mid = (low + high) / 2;
    for i in low..high {
        let to_compare = &items[i];
        if to_compare < &items[mid] {
            items.swap(i, swap_index);
            swap_index += 1;
        }
    }
    items.swap(mid, swap_index);
    swap_index
}

pub fn mergesort() {}

#[cfg(test)]
mod tests {
    use crate::iterative::*;

    #[test]
    fn quicksort_works() {
        let mut test_array: Vec<i32> = vec![54, 63, 4, 543, 6, 87, 56, 486, 478, 56, 32, 77, 1, 66, 2, 4, 1, 7];
        quicksort(&mut test_array);
        println!("{:?}", test_array);

        let mut lol = vec![7, 2, 3, 4, 5, 6, 1111, 1, 2, 3, 4, 5, 6];
        quicksort(&mut lol);
        println!("{:?}", lol);
    }
}