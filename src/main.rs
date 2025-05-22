#[derive(Debug, Clone)]
struct Edge {
    lhs: String,
    rhs: String,
    consumed: bool,
}

struct GraphProcessor {
    edges: Vec<Edge>,
    unused_edges: Vec<Edge>,
    found_nodes: Vec<String>,
    unsearched_nodes: Vec<String>,
}

impl GraphProcessor {
    fn new(edges: Vec<Edge>) -> GraphProcessor {
        let unused_edges = edges.clone();
        let found_nodes = Vec::new();
        let unsearched_nodes = Vec::new();

        GraphProcessor {
            edges,
            unused_edges,
            found_nodes,
            unsearched_nodes,
        }
    }
    fn find_all_nodes(mut self, start_node: String) {
        self.found_nodes.push(start_node.clone());
        self.unsearched_nodes.push(start_node.clone());

        self.print_found_nodes();
        self.print_unsearched_nodes();
        self.print_unused_edges();

        for node in self.unsearched_nodes {
            //println!(">>{}", node);
            self.find_child_nodes_from_node(&node);
        }
    }

    fn find_child_nodes_from_node(&mut self, node: &String) {
        for edge in &mut self.unused_edges {
            if !edge.consumed {
                if edge.lhs == *node {
                    // Add node to nodelists
                    self.found_nodes.push(edge.rhs.clone());
                    self.unsearched_nodes.push(edge.rhs.clone());
                    // Mark edge consumed
                    edge.consumed = true;
                } else if edge.rhs == *node {
                    // Add node to nodelists
                    self.found_nodes.push(edge.lhs.clone());
                    self.unsearched_nodes.push(edge.lhs.clone());
                    // Mark edge consumed
                    edge.consumed = true;
                }
            }
        }
        // Remove input node from list of unsearched nodes
        self.unsearched_nodes.retain(|n| *n != *node);
    }
    fn print_found_nodes(&self) {
        println!("Found Nodes: {:#?}", self.found_nodes);
    }
    fn print_unsearched_nodes(&self) {
        println!("Unsearched Nodes: {:#?}", self.unsearched_nodes);
    }
    fn print_unused_edges(&self) {
        println!("Unused: {:#?}", self.unused_edges);
    }
}

fn main() {
    let edges = vec![
        Edge {
            lhs: String::from("A"),
            rhs: String::from("B"),
            consumed: false,
        },
        Edge {
            lhs: String::from("C"),
            rhs: String::from("B"),
            consumed: false,
        },
        Edge {
            lhs: String::from("B"),
            rhs: String::from("D"),
            consumed: false,
        },
        Edge {
            lhs: String::from("E"),
            rhs: String::from("A"),
            consumed: false,
        },
    ];

    let gp = GraphProcessor::new(edges);
    let start_node = String::from("A");

    gp.find_all_nodes(start_node);
    gp.print_found_nodes();
    gp.print_unsearched_nodes();
    gp.print_unused_edges();
}
