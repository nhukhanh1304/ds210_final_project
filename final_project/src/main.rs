use analysis::print_degree_distribution;
use analysis::find_top_jaccard_similarities;
use analysis::find_most_similar_pair;


mod data_loader;
mod network;
mod analysis;
mod helpers;

use data_loader::load_graph_from_file;
use analysis::average_shortest_path_length;

fn main() {
    let graph = load_graph_from_file("facebook_combined.txt");

    println!("Graph loaded successfully!");
    println!("Number of nodes: {}", graph.num_nodes());
    println!("Degree of node 0: {}", graph.degree(0));

    let avg_path_length = average_shortest_path_length(&graph, 0);
    println!("Average shortest path length from node 0: {:.2}", avg_path_length);

    print_degree_distribution(&graph);
    
    find_top_jaccard_similarities(&graph, 0, 5);

    find_most_similar_pair(&graph);

}
