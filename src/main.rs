use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::ops::Add;

fn main() {
    let mut nums = vec![5, 23, 6, 45535, 234, 234, 5654, 54, 235, 4353, 87, 56, 5, 3, 643, 6, 4, 6655, 767];

    selection_sort(&mut nums);
    println!("{:?}", nums);

    let found_index = binary_search(&nums, &45535).unwrap();
    println!("found at index {}", found_index);

    let empty = binary_search(&nums, &21);
    println!("empty: {}", empty.is_none());

    let sum = sum_dc(&nums);
    println!("sum is {}", sum);

    let count = count_dc(&nums);
    println!("count is {}", count);

    let largest = max_dc(&nums);
    println!("max is {}", largest);

    let found = binary_search_dc(&nums, &45535);
    println!("{:?}", found);


    let nums = vec![5, 23, 6, 45535, 234, 234, 5654, 54, 235, 4353, 87, 56, 5, 3, 643, 6, 4, 6655, 767];
    let result = quicksort(nums);
    println!("{:?}, {}", result, result.len());

    let nums = vec![5, 23, 6, 45535, 234, 234, 5654, 54, 235, 4353, 87, 56, 5, 3, 643, 6, 4, 6655, 767];
    let m_sorted = mergesort(nums);
    println!("{:?}", m_sorted);

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    graph.insert("you", vec!["bob", "claire", "alice"]);
    graph.insert("bob", vec!["anuj", "peggy"]);
    graph.insert("alice", vec!["peggy"]);
    graph.insert("claire", vec!["thom", "jonny"]);
    graph.insert("anuj", vec!["jonny"]);
    graph.insert("peggy", vec!["thom"]);
    graph.insert("thom", vec!["anuj"]);
    graph.insert("jonny", vec!["alice"]);
    graph.insert("alice", vec!["you"]);

    let result = bfs(&mut graph, "bob", "claire");
    println!("{:?}", result);
}

fn binary_search<T: PartialEq + PartialOrd>(vec: &[T], item: &T) -> Option<usize> {
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

fn binary_search_dc<T: PartialEq + PartialOrd>(items: &[T], item: &T) -> Option<usize> {
    if items.len() == 0 {
        return None;
    }
    if items.len() == 1 {
        return if item == &items[0] { Some(0) } else { None };
    }
    let high = items.len() - 1;
    let mid = high / 2;
    let current = items.get(mid).unwrap();
    return if current == item {
        Some(mid)
    } else {
        let (lower, upper, should_add) =
            if current > item {
                (0 as usize, mid - 1, false)
            } else {
                (mid + 1, items.len(), true)
            };
        let maybe = binary_search_dc(&items[lower..upper], item);
        if let Some(val) = maybe {
            if should_add {
                Some(val + mid)
            } else {
                Some(val)
            }
        } else { None }
    };
}

fn quicksort<T: PartialEq + PartialOrd + Copy>(items: Vec<T>) -> Vec<T> {
    let mut items = items;
    if items.len() < 2 {
        return items;
    }
    let mid = items.len() / 2;
    let pivot = items.remove(mid);

    // sorting part
    let mut less = items.clone();
    less.retain(|item| item <= &pivot);
    let mut more = items.clone();
    more.retain(|item| item > &pivot);

    // recursive part
    let mut less = quicksort(less);
    let mut more = quicksort(more);

    let mut result = vec![];
    result.append(&mut less);
    result.push(pivot);
    result.append(&mut more);
    result
}

fn mergesort<T: PartialEq + PartialOrd + Copy>(items: Vec<T>) -> Vec<T> {
    let len = items.len();
    // base case, already sorted array of one/zero
    if len < 2 {
        return items;
    }
    // recursive case
    let mid = len / 2;
    // range is [inclusive..exclusive]
    // i.e. [a,b)
    let left = items[..mid].to_vec();
    let right = items[mid..].to_vec();

    // reduced to a base case of 1 element
    // in which case it's merged
    // then, the stack collapses upwards
    let sorted_left = mergesort(left);
    let sorted_right = mergesort(right);
    return merge(sorted_left, sorted_right);
}

fn merge<T: PartialOrd + Copy>(mut arr1: Vec<T>, mut arr2: Vec<T>) -> Vec<T> {
    let mut sorted: Vec<T> = Vec::with_capacity(arr1.len() + arr2.len());

    // sorted merge
    while arr1.len() > 0 && arr2.len() > 0 {
        if arr1[0] > arr2[0] {
            let a = arr2.remove(0);
            sorted.push(a);
        } else {
            let b = arr1.remove(0);
            sorted.push(b);
        }
    }
    // either/both array(s) is empty
    for val in arr1 {
        sorted.push(val);
    }
    for val in arr2 {
        sorted.push(val);
    }
    sorted
}

fn bfs(graph: &mut HashMap<&str, Vec<&str>>, origin: &str, destination: &str) -> bool {
    let mut visited: HashSet<&str> = HashSet::new();

    // queue of nodes (last node of path) to search
    let mut to_visit: VecDeque<Vec<&str>> = VecDeque::new();

    if !graph.contains_key(origin) || !graph.contains_key(destination) {
        return false;
    }
    to_visit.push_back(vec![origin]);

    while let Some(current) = to_visit.pop_front() {
        // get the last node of the path
        let latest = *current.last().unwrap();
        if latest == destination {
            println!("{:?}", current);
            return true;
        }
        visited.insert(latest);
        if let Some(dests) = graph.get(latest) {
            // for all nodes that the current node points to
            for destination in dests {
                if visited.contains(*destination) { continue; }
                // create a copy of the current path, append the next node
                // then add it back to the search queue
                let mut new_path = current.clone();
                new_path.push(destination);
                to_visit.push_back(new_path);
            }
        }
    }
    false
}
