// data_loader.rs: this module handles reading edge list file and converts it into a graph structure

use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::network::Graph;

/// loads the graph from a facebook edge list file where each line contains two node ids
/// 'path': file path to the input edge list
/// returns a 'Graph' struct with all edges from the file added
pub fn load_graph_from_file(path: &str) -> Graph {
    let file = File::open(path).expect("failed to open file");
    let reader = BufReader::new(file);
    let mut graph = Graph::new();

    for line in reader.lines() {
        if let Ok(edge) = line {
            let parts: Vec<&str> = edge.trim().split_whitespace().collect();
            if parts.len() == 2 {
                let a = parts[0].parse::<usize>().unwrap();  // parse first node
                let b = parts[1].parse::<usize>().unwrap();  // parse second node
                graph.add_edge(a, b);  // add undirected edge to graph
            }
        }
    }

    graph
}
