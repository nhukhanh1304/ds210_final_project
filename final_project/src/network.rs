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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_edge_and_degree() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);

        assert_eq!(graph.degree(1), 2); // Node 1 should have 2 neighbors
        assert_eq!(graph.degree(2), 1); // Node 2 connected to 1
        assert_eq!(graph.degree(3), 1); // Node 3 connected to 1
    }
}
