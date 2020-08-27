use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Copy)]
pub struct Edge {
    pub distance: u32,
    pub target: &'static str,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        other.target == self.target
    }
}

#[derive(Debug, Copy)]
struct State {
    name: &'static str,
    current_distance: u32,
    parent: Option<&'static str>
}

pub fn find_path(graph: HashMap<&str, HashSet<Edge>>, origin: &str, destination: &str) {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut states: HashMap<&str, State> = HashMap::new();

    
}