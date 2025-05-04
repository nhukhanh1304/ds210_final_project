# ds210_final_project

# Rust Social Network Graph Analysis
This Rust project analyzes the structure of a social network using graph theory concepts like degree distribution, shortest path lengths, and Jaccard similarity. The goal is to explore how connected a network is and how similar different users are based on their connections.

## Dataset
- **Source:** [SNAP Facebook Combined Dataset](https://snap.stanford.edu/data/ego-Facebook.html)
- **Size:** 4,039 nodes and 88,234 undirected edges
- **Format:** Each line in `facebook_combined.txt` contains a pair of connected node IDs.

## Features
- Load a Facebook ego-network dataset into an adjacency list
- Compute and print:
  - Degree distribution histogram
  - Average shortest path length from a node
  - Top 5 most similar nodes to a target node using Jaccard similarity
  - Most similar node pair across the entire graph

## Graph Analysis Used
- Breadth-First Search (BFS)
- Degree Distribution
- Jaccard Similarity
- Graph traversal and analysis in Rust
