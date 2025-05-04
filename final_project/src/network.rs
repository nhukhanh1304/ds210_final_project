// network.rs: this module defines the graph data structure and basic operations for adding edges and query graph properties

use std::collections::HashMap;

/// represents an undirected graph using an adjacency list
pub struct Graph {
    pub adj_list: HashMap<usize, Vec<usize>>,  // maps each node to its list of neighbors
}

impl Graph {
    /// creates a new empty graph
    pub fn new() -> Self {
        Graph {
            adj_list: HashMap::new(),
        }
    }

    /// adds an undirected edge between two nodes.
    /// updates both nodes' adjacency lists
    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj_list.entry(u).or_default().push(v);  // add v to u's neighbor list
        self.adj_list.entry(v).or_default().push(u);  // add u to v's neighbor list
    }

    /// returns the num of neighbors for a given node
    pub fn degree(&self, node: usize) -> usize {
        self.adj_list.get(&node).map_or(0, |neighbors| neighbors.len())
    }

    /// returns total num of nodes in the graph
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
        graph.add_edge(1, 2);  // add edge 1-2
        graph.add_edge(1, 3);  // add edge 1-3

        assert_eq!(graph.degree(1), 2);  // node 1 should hav 2 neighbors
        assert_eq!(graph.degree(2), 1);  // node 2 connected to 1
        assert_eq!(graph.degree(3), 1);  // node 3 connected to 1
    }
}
