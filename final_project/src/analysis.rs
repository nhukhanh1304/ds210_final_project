use std::collections::{HashSet, VecDeque};
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
