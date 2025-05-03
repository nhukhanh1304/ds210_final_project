use std::collections::{HashMap, HashSet, VecDeque};
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



/// Computes the Jaccard similarity between two nodes in the graph.
fn jaccard_similarity(graph: &Graph, a: usize, b: usize) -> f64 {
    let neighbors_a = match graph.adj_list.get(&a) {
        Some(neighbors) => neighbors,
        None => return 0.0,
    };

    let neighbors_b = match graph.adj_list.get(&b) {
        Some(neighbors) => neighbors,
        None => return 0.0,
    };

    let set_a: HashSet<_> = neighbors_a.iter().copied().collect();
    let set_b: HashSet<_> = neighbors_b.iter().copied().collect();

    let intersection: usize = set_a.intersection(&set_b).count();
    let union: usize = set_a.union(&set_b).count();

    if union == 0 {
        0.0
    } else {
        intersection as f64 / union as f64
    }
}

/// Finds and prints the top 5 most similar nodes to a given node using Jaccard similarity.
pub fn find_top_jaccard_similarities(graph: &Graph, target: usize, top_n: usize) {
    println!("\nTop {top_n} nodes most similar to node {target} (by Jaccard similarity):");

    let mut similarities: Vec<(usize, f64)> = graph.adj_list
        .keys()
        .filter(|&&node| node != target)
        .map(|&node| (node, jaccard_similarity(graph, target, node)))
        .collect();

    // Sort by similarity score, descending
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    for (node, score) in similarities.iter().take(top_n) {
        println!("Node {:>4}: Similarity {:.3}", node, score);
    }
}

/// Finds and prints the pair of nodes in the entire graph with the highest Jaccard similarity.
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
        "\n💡 Most similar node pair (by Jaccard): {} & {} with similarity {:.3}",
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

        let distribution = compute_degree_distribution(&graph);

        // Expecting all 3 nodes to have degree 2
        assert_eq!(distribution.get(&2), Some(&3));
        assert!(!distribution.contains_key(&1)); // No node with degree 1
    }
}
