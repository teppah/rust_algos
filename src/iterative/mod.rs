use std::fmt::Debug;

pub fn quicksort<T: PartialOrd>(items: &mut [T]) {
    do_quicksort(items, 0, items.len());
}

fn do_quicksort<T: PartialOrd>(items: &mut [T], low: usize, high: usize) {
    if low < high {
        let partition_index = partition_qs(items, low, high);
        do_quicksort(items, low, partition_index);
        do_quicksort(items, partition_index + 1, high);
    }
}

// assume high is exclusive
fn partition_qs<T: PartialOrd>(items: &mut [T], low: usize, high: usize) -> usize {
    if low == high {
        return low;
    }
    let mut swap_index: usize = low;
    let pivot_index = high - 1;
    for i in low..high {
        let to_compare = &items[i];
        if to_compare < &items[pivot_index] {
            items.swap(i, swap_index);
            swap_index += 1;
        }
    }
    items.swap(pivot_index, swap_index);
    swap_index
}

pub fn mergesort<T: PartialOrd + Copy>(items: &mut [T]) {
    do_mergesort(items, 0, items.len());
}

// assume high is exclusive
fn do_mergesort<T: PartialOrd + Copy>(array: &mut [T], low: usize, high: usize) {
    if low < high - 1 {
        let mid = (low + high) / 2;
        do_mergesort(array, low, mid);
        do_mergesort(array, mid, high);
        merge(array, low, mid, high);
    }
}

fn merge<T: PartialOrd + Copy>(array: &mut [T], low: usize, mid: usize, high: usize) {
    let mut first_index = low;
    let mut second_index = mid;
    let mut result: Vec<T> = Vec::with_capacity(high - low);

    while first_index < mid && second_index < high {
        let first = array[first_index];
        let second = array[second_index];
        if first < second {
            result.push(first);
            first_index += 1;
        } else {
            result.push(second);
            second_index += 1;
        }
    }
    while first_index < mid {
        result.push(array[first_index]);
        first_index += 1;
    }
    while second_index < high {
        result.push(array[second_index]);
        second_index += 1;
    }

    for (index, v) in result.into_iter().enumerate() {
        array[low + index] = v;
    }
}

#[cfg(test)]
mod tests {
    use crate::iterative::*;

    #[test]
    fn quicksort_works() {
        let mut test_array: Vec<i32> = vec![54, 63, 4, 543, 6, 87, 56, 486, 478, 56, 32, 77, 1, 66, 2, 4, 1, 7];
        quicksort(&mut test_array);
        println!("{:?}", test_array);

        let mut test_two = vec![7, 2, 3, 4, 5, 6, 1111, 1, 2, 3, 4, 5, 6];
        quicksort(&mut test_two);
        println!("{:?}", test_two);
    }

    #[test]
    fn mergesort_works() {
        let mut test_array: Vec<i32> = vec![54, 63, 4, 543, 6, 87, 56, 486, 478, 56, 32, 77, 1, 66, 2, 4, 1, 7];
        mergesort(&mut test_array);
        println!("{:?}", test_array);

        let mut test_two = vec![7, 2, 3, 4, 5, 6, 1111, 1, 2, 3, 4, 5, 6];
        mergesort(&mut test_two);
        println!("{:?}", test_two);
    }
}