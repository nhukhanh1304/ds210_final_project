// analysis.rs: This module is for network analysis functions including: degree distribution, average path length, and jaccard similarity for a graph

use std::collections::{HashMap, HashSet, VecDeque};
use crate::network::Graph;

/// runs a BFS from the given start node and returns the shortest distance to each reachable node.
/// 'graph': the input graph as a reference to the graph struct; 'start': the starting node index
/// returns a vector of shortest distances from `start` to every other node (usize::MAX if unreachable)
pub fn bfs_shortest_paths(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = HashSet::new();  // track visited nodes
    let mut distance = vec![usize::MAX; graph.num_nodes()];  // initialize all distances to 'infinity'
    let mut queue = VecDeque::new();  // queue for bfs traversal

    visited.insert(start);  // mark the start node as visited
    distance[start] = 0;    // distance to itself is 0
    queue.push_back(start); // start bfs traversal from start node

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.adj_list.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);  // visit neighbor
                    distance[neighbor] = distance[current] + 1;  // set neighbor's distance
                    queue.push_back(neighbor);  
                }
            }
        }
    }

    distance
}

/// computes the average shortest path length from a given starting node using bfs
/// returns average distance to all reachable nodes as 'f64'
pub fn average_shortest_path_length(graph: &Graph, start: usize) -> f64 {
    let shortest_path_lengths = bfs_shortest_paths(graph, start);
    let mut total = 0;
    let mut count = 0;

    for &dist in &shortest_path_lengths {
        if dist != usize::MAX && dist != 0 {
            total += dist;  // acumulate total distance
            count += 1;  // count reachable nodes (excluding self)
        }
    }

    if count == 0 {
        0.0
    } else {
        total as f64 / count as f64
    }
}

/// calculates the degree distribution of a graph.
/// returns a hashmap mapping each degree value to the num of nodes with that degree
pub fn compute_degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut degree_count = HashMap::new();

    for neighbors in graph.adj_list.values() {
        let degree = neighbors.len();
        *degree_count.entry(degree).or_insert(0) += 1;
    }

    degree_count
}

/// prints a histogram of the degree distribution to the terminal using simple bars built from '*'
pub fn print_degree_distribution(graph: &Graph) {
    let degree_count = compute_degree_distribution(graph);

    // sort degrees by value
    let mut degrees: Vec<_> = degree_count.iter().collect();
    degrees.sort_by_key(|&(degree, _)| *degree);

    // normalize bar width based on the highest frequency
    let max_count = degree_count.values().copied().max().unwrap_or(1);

    for (degree, count) in degrees {
        // scale histogram bar to max 50 char
        let bar_len = (50 * *count) / max_count;
        println!("{:>3} friend(s): {}", degree, "*".repeat(bar_len.max(1)));

    }
}

/// computes the Jaccard similarity between two nodes based on their neighbirs
/// returns the similarity score as an 'f64'
fn jaccard_similarity(graph: &Graph, a: usize, b: usize) -> f64 {
    let neighbors_a = match graph.adj_list.get(&a) {
        Some(neighbors) => neighbors,
        None => return 0.0,
    };

    let neighbors_b = match graph.adj_list.get(&b) {
        Some(neighbors) => neighbors,
        None => return 0.0,
    };

    let set_a: HashSet<_> = neighbors_a.iter().copied().collect();  // convert neighbor list to set
    let set_b: HashSet<_> = neighbors_b.iter().copied().collect();

    let intersection: usize = set_a.intersection(&set_b).count();
    let union: usize = set_a.union(&set_b).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// finds and prints the top 5 most similar nodes to a given node using jaccard similarity
pub fn find_top_jaccard_similarities(graph: &Graph, target: usize, top_n: usize) {
    println!("\nTop {top_n} users most similar to User 0 (based on Jaccard similarity):");

    let mut similarities: Vec<(usize, f64)> = graph.adj_list
        .keys()
        .filter(|&&node| node != target)
        .map(|&node| (node, jaccard_similarity(graph, target, node)))
        .collect();

    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());  // sort descending

    for (node, score) in similarities.iter().take(top_n) {
        println!("User {:>4} has similarity {:.3}", node, score);
    }
}

/// finds and prints the pair of nodes in the entire graph with the highest jaccard similarity by iterating through all node pairs
pub fn find_most_similar_pair(graph: &Graph) {
    let nodes: Vec<_> = graph.adj_list.keys().copied().collect();
    let mut best_pair = (0, 0);
    let mut best_score = 0.0;

    for i in 0..nodes.len() {
        for j in (i + 1)..nodes.len() {
            let a = nodes[i];
            let b = nodes[j];
            let score = jaccard_similarity(graph, a, b);
            if score > best_score {
                best_score = score;
                best_pair = (a, b);
            }
        }
    }

    println!(
        "The most similar pair of users in the entire network (with most overlap in friends) is User {} & User {} with similarity {:.3}",
        best_pair.0, best_pair.1, best_score
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::network::Graph;

    #[test]
    fn test_compute_degree_distribution() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(2, 3);

        let degree_count = compute_degree_distribution(&graph);

        assert_eq!(degree_count.get(&2), Some(&3));  // all nodes have degree 2
        assert!(!degree_count.contains_key(&1));     // no node has degree 1
    }

    #[test]
    fn test_empty_graph_degree_distribution() {
        let graph = Graph::new();
        let degree_count = compute_degree_distribution(&graph);
        assert!(degree_count.is_empty());
    }

    #[test]
    fn test_graph_with_isolated_nodes() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.adj_list.insert(3, vec![]);  // isolated node
        let degree_count = compute_degree_distribution(&graph);

        assert_eq!(degree_count.get(&0), Some(&1));  // node 3
        assert_eq!(degree_count.get(&1), Some(&2));  // nodes 1 and 2
    }

    #[test]
    fn test_duplicate_edges() {
        let mut graph = Graph::new();
        graph.add_edge(1, 2);
        graph.add_edge(1, 2);  // duplicate

        assert_eq!(graph.degree(1), 2);
        assert_eq!(graph.degree(2), 2);
    }

    #[test]
    fn test_bfs_shortest_paths_triangle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let distances = bfs_shortest_paths(&graph, 0);

        assert_eq!(distances[0], 0);
        assert_eq!(distances[1], 1);
        assert_eq!(distances[2], 1);
    }

    #[test]
    fn test_average_shortest_path_length_triangle() {
        let mut graph = Graph::new();
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let avg = average_shortest_path_length(&graph, 0);
        assert!((avg - 1.0).abs() < 1e-6);  // (1+1)/2 = 1.0
    }
}
