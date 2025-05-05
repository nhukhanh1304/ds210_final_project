// main.rs: this is the main for running graph analysis on the facebook_combined.txt dataset

use analysis::print_degree_distribution;
use analysis::find_top_jaccard_similarities;
use analysis::find_most_similar_pair;
use data_loader::load_graph_from_file;
use analysis::average_shortest_path_length;

mod data_loader;
mod network;
mod analysis;

fn main() {
    // load the graph from the edge list file
    let graph = load_graph_from_file("facebook_combined.txt");

    println!("Number of users (nodes): {}", graph.num_nodes());
    println!("User 0 has {} direct friends", graph.degree(0));

    // compute & print average shortest path length from node 0
    let avg_path_length = average_shortest_path_length(&graph, 0);
    println!("On average, User 0 is {:.2} connections away from other users in the graph", avg_path_length);

    // prints degree distribution as ascii histogram made from '*'
    println!("\nFriendship degree distribution - number of users with X friends");
    print_degree_distribution(&graph);
    
    // finds and prints top 5 nodes most similar to node 0 using jacard similarity
    find_top_jaccard_similarities(&graph, 0, 5);

    // finds and prints the most similar node pair in the graph by jaccard similarity
    find_most_similar_pair(&graph);
} 
