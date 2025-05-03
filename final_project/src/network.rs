use std::collections::HashMap;

/// Represents an undirected graph using an adjacency list.
pub struct Graph {
    pub adj_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    /// Adds an undirected edge between two nodes.
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list.entry(u).or_default().push(v);
        self.adj_list.entry(v).or_default().push(u);
    }

    /// Returns the number of neighbors for a given node.
    pub fn degree(&self, node: usize) -> usize {
        self.adj_list.get(&node).map_or(0, |neighbors| neighbors.len())
    }

    /// Returns total number of nodes in the graph.
    pub fn num_nodes(&self) -> usize {
        self.adj_list.len()
    }
}
