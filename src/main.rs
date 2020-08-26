use rust_algos::search::binary_search;
use rust_algos::sort::{selection_sort, quicksort, mergesort};
use rust_algos::dc::{sum_dc, count_dc, max_dc, binary_search_dc};
use std::collections::HashMap;

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

