type Node = String;

#[derive(Debug, Clone)]
struct Edge {
    lhs: Node,
    rhs: Node,
}
struct GraphProcessor {}

impl GraphProcessor {
    fn new() -> GraphProcessor {
        Self {}
    }
    fn find_direct_child_nodes(&self, parent: Node, edges: &Vec<Edge>) -> Vec<Node> {
        let mut children: Vec<Node> = Vec::new();

        for edge in edges.iter() {
            if edge.lhs == parent {
                // Add node to nodelists
                children.push(edge.rhs.clone());
                // Mark edge consumed
            } else if edge.rhs == parent {
                // Add node to nodelists
                children.push(edge.lhs.clone());
                // Mark edge consumed
            }
        }
        children
    }

    fn find_all_nodes(&self, initial_node: Node, edges: &Vec<Edge>) -> Vec<Node> {
        let mut children: Vec<Node> = Vec::new(); // Accumulates all nodes
        let mut unsearched_nodes: Vec<Node> = Vec::new(); // Accumulates
        let mut new_nodes: Vec<Node> = Vec::new(); // Accumulates
        unsearched_nodes.push(initial_node);

        for node in unsearched_nodes.iter() {
            new_nodes.extend(self.find_direct_child_nodes(node.clone(), edges));
        }
        children.extend(unsearched_nodes);

        unsearched_nodes.extend(new_nodes.clone());
        new_nodes.clear();

        // Return
        children
    }
}

fn main() {
    let edges = vec![
        Edge {
            lhs: String::from("A"),
            rhs: String::from("B"),
        },
        Edge {
            lhs: String::from("C"),
            rhs: String::from("B"),
        },
        Edge {
            lhs: String::from("B"),
            rhs: String::from("D"),
        },
        Edge {
            lhs: String::from("E"),
            rhs: String::from("A"),
        },
    ];

    let gp = GraphProcessor::new();
    let start_node: Node = String::from("A");

    let children = gp.find_all_nodes(start_node, &edges);

    println!("{:#?}", children);
}
