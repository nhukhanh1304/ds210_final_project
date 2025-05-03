use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::network::Graph;

/// Loads the graph from a Facebook edge list file.
pub fn load_graph_from_file(path: &str) -> Graph {
    let file = File::open(path).expect("Failed to open file.");
    let reader = BufReader::new(file);
    let mut graph = Graph::new();

    for line in reader.lines() {
        if let Ok(edge) = line {
            let parts: Vec<&str> = edge.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let u = parts[0].parse::<usize>().unwrap();
                let v = parts[1].parse::<usize>().unwrap();
                graph.add_edge(u, v);
            }
        }
    }

    graph
}
