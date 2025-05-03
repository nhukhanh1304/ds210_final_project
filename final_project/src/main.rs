mod data_loader;
mod network;
mod analysis;
mod helpers;

use data_loader::load_graph_from_file;

fn main() {
    let graph = load_graph_from_file("facebook_combined.txt");

    println!("✅ Graph loaded successfully!");
    println!("Number of nodes: {}", graph.num_nodes());
    println!("Degree of node 0: {}", graph.degree(0));
}
