use std::collections::{HashSet, VecDeque};
use std::collections::HashMap;
use crate::network::Graph;

/// Runs BFS from the given start node and returns the shortest distance to each reachable node.
pub fn bfs_shortest_paths(graph: &Graph, start: usize) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut distance = vec![usize::MAX; graph.num_nodes()];
    let mut queue = VecDeque::new();

    visited.insert(start);
    distance[start] = 0;
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = graph.adj_list.get(&current) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    distance[neighbor] = distance[current] + 1;
                    queue.push_back(neighbor);
                }
            }
        }
    }

    distance
}

/// Computes the average shortest path length from a given node.
pub fn average_shortest_path_length(graph: &Graph, start: usize) -> f64 {
    let distances = bfs_shortest_paths(graph, start);
    let mut total = 0;
    let mut count = 0;

    for &d in &distances {
        if d != usize::MAX && d != 0 {
            total += d;
            count += 1;
        }
    }

    if count == 0 {
        0.0
    } else {
        total as f64 / count as f64
    }
}

/// Computes the degree distribution of a graph.
/// Returns a map from degree value → number of nodes with that degree.
pub fn compute_degree_distribution(graph: &Graph) -> HashMap<usize, usize> {
    let mut distribution = HashMap::new();

    for (&node, neighbors) in &graph.adj_list {
        let degree = neighbors.len();
        *distribution.entry(degree).or_insert(0) += 1;
    }

    distribution
}

/// Prints the degree distribution to the console.
/// Prints a CLI histogram of the degree distribution.
pub fn print_degree_distribution(graph: &Graph) {
    let distribution = compute_degree_distribution(graph);
    println!("\nDegree Distribution (degree: count, histogram):");

    // Sort degrees by value
    let mut degrees: Vec<_> = distribution.iter().collect();
    degrees.sort_by_key(|&(degree, _)| *degree);

    // Optional: normalize bar width
    let max_count = distribution.values().copied().max().unwrap_or(1);

    for (degree, count) in degrees {
        // Scale histogram bar to max 50 characters
        let bar_len = (50 * *count) / max_count;
        let bar = "*".repeat(bar_len.max(1));  // always show something
        println!("{:>3}: {:>4} {}", degree, count, bar);
    }
}

