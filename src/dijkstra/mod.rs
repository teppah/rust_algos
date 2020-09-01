use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Edge {
    pub distance: usize,
    pub target: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
struct State {
    name: String,
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn find_path(graph: &HashMap<&str, HashSet<Edge>>, origin: &str, destination: &str) -> Option<usize> {
    let mut parents: HashMap<&str, Option<String>> = HashMap::new();
    let mut costs: HashMap<&str, usize> = HashMap::new();

    let mut heap: BinaryHeap<State> = BinaryHeap::new();

    heap.push(State { name: origin.to_string(), cost: 0 });
    parents.insert(origin, None);

    while let Some(State { name, cost }) = heap.pop() {
        if name == destination {
            let backtraced = backtrace(destination, &parents);
            println!("{:?}", backtraced);
            return Some(cost);
        }
        // maybe a better way already exist
        //      because it got added later than the current State due to the order
        //      the edges were processed below,
        //      and since priority queue will have already processed the better one,
        //      ignore this one
        if let Some(current_cost) = costs.get(name.as_str()) {
            if *current_cost < cost { continue; }
        }
        if let Some(edge) = graph.get(name.as_str()) {
            for edge in edge.iter() {
                let current_cost = costs.entry(edge.target).or_insert(usize::MAX);
                let new_cost = cost + edge.distance;

                // if the new cost is better than the current cost,
                //      the next node this edge points to will be visited
                //      from the current node
                //      and its cost, parent will be updated from this node
                if new_cost < *current_cost {
                    let current = parents.entry(edge.target).or_insert(None);
                    *current = Some(name.to_string());
                    *current_cost = new_cost;
                    let new_state = State {
                        name: edge.target.to_string(),
                        cost: new_cost,
                    };
                    heap.push(new_state);
                }
            }
        }
    }
    None
}

fn backtrace(end: &str, parents: &HashMap<&str, Option<String>>) -> Vec<String> {
    let mut to_search = end.to_string();
    let mut result = vec![];
    result.push(end.to_string());
    while let Some(Some(current)) = parents.get(to_search.as_str()) {
        result.push(current.to_string());
        to_search = current.to_string();
    }
    result.reverse();
    result
}

