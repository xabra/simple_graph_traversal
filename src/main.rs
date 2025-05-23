use std::collections::{HashSet, VecDeque};

/// Simulated function to get all edges connected to a node.
/// You will replace this with your actual data source or API.
fn get_edges(node: &str) -> Vec<(String, String)> {
    match node {
        "A" => vec![
            ("A".to_string(), "B".to_string()),
            ("A".to_string(), "C".to_string()),
        ],
        "B" => vec![("B".to_string(), "D".to_string())],
        "C" => vec![("C".to_string(), "E".to_string())],
        "E" => vec![("E".to_string(), "F".to_string())],
        _ => vec![],
    }
}

/// Find all nodes connected to the starting node by dynamically exploring edges.
fn find_connected_nodes_dynamic(start: String) -> HashSet<String> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start.clone());
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        let edges = get_edges(&node);
        for (a, b) in edges {
            let neighbor = if a == node { b } else { a };
            if visited.insert(neighbor.clone()) {
                queue.push_back(neighbor);
            }
        }
    }

    visited
}

fn main() {
    let start_node = "A".to_string();
    let connected = find_connected_nodes_dynamic(start_node);
    println!("Connected nodes: {:?}", connected);
}
