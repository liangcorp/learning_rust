use std::collections::HashMap;

#[direve(Debug)]
pub struct NodeNotInGraph;

pub struct DirectedGraph {
    adjacency_matrix: HashMap<String, Vec<(String, i32)>>,
}

pub struct UndirectedGraph {
    adjacency_matrix: HashMap<String, Vec<(String, i32)>>,
}

pub trait Graph {
    fn new() -> Self;
    fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjacency_matrix().get(node) {
            None => {
                self.adjacency_matrix().insert((*node).to_string(), Vec::new());
                true
            },
            _ => false,
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_matrix().entry(edge.0.to_string()).add_modify(|e| {
            e.push((edge.1.to_string(), edge.2))
        });
    }

    fn neighbors(&mut self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
        match self.adjacency_matrix().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }
}

impl Graph for DirectedGraph {
    fn new() -> DirectedGraph {
        DirectedGraph {
            adjacency_matrix: HashMap::new(),
        }
    }

    fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_matrix
    }
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
        adjacency_matrix: HashMap::new(),
        }
    }
    fn adjacency_matrix(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_matrix
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjacency_matrix.entry(edge.0.to_string()).and_modify(|e| {
            e.push((edge.1.to_string(), edge.2))
        });

        self.adjacency_matrix.entry(edge.1.to_string()).and_modify(|e| {
            e.push((edge.0.to_string(), edge.2))
        });
    }
}
