// graph.rs
// Constructs the similarity graph, computes clusters, analyzes centrality, and saves results.

use petgraph::graph::Graph;
use petgraph::algo::connected_components;
use petgraph::unionfind::UnionFind;
use petgraph::visit::{EdgeRef, IntoNodeReferences};
use crate::country::CountryData;
use crate::similarity::cosine_similarity;
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};

// Builds the similarity graph based on a threshold.
// Inputs:  
//  - cleaned country emission data
//  - similarity threshold
// Outputs: Graph
pub fn build_similarity_graph(data: &HashMap<String, CountryData>, threshold: f64) -> Graph<String, f64> {
    let mut graph = Graph::<String, f64>::new();
    let mut nodes = HashMap::new();

    // Add nodes
    for (name, _) in data {
        let idx = graph.add_node(name.clone());
        nodes.insert(name.clone(), idx);
    }

    // Add edges if similarity exceeds threshold
    for (name1, data1) in data {
        for (name2, data2) in data {
            if name1 < name2 {
                let sim = cosine_similarity(data1, data2);
                if sim > threshold {
                    graph.add_edge(nodes[name1], nodes[name2], sim);
                }
            }
        }
    }

    graph
}

// Prints basic stats about the graph.
pub fn print_graph_stats(graph: &Graph<String, f64>) {
    println!("- Total Countries (Nodes): {}", graph.node_count());
    println!("- Total Strong Similarity Connections (Edges): {}", graph.edge_count());
    println!("- Number of Connected Components (Clusters): {}", connected_components(graph));
}

// Ranks and prints countries by degree centrality.
pub fn print_degree_centrality(graph: &Graph<String, f64>) {
    println!("🌟 Top Countries by Degree Centrality:");
    let mut centrality = vec![];

    for node in graph.node_references() {
        let degree = graph.edges(node.0).count();
        centrality.push((node.1.clone(), degree));
    }

    // Sort by degree descending
    centrality.sort_by(|a, b| b.1.cmp(&a.1));

    for (name, degree) in centrality.iter().take(10) {
        println!("- {}: {} strong connections", name, degree);
    }
}

// Groups and prints clusters (connected components).
pub fn print_clusters(graph: &Graph<String, f64>) {
    let mut uf = UnionFind::new(graph.node_count());

    // Union nodes connected by an edge
    for edge in graph.edge_references() {
        let (a, b) = (edge.source().index(), edge.target().index());
        uf.union(a, b);
    }

    let mut clusters: HashMap<usize, Vec<String>> = HashMap::new();
    for node_idx in graph.node_indices() {
        let root = uf.find(node_idx.index());
        clusters.entry(root)
            .or_default()
            .push(graph[node_idx].clone());
    }

    println!("\n🗺️ Emission Behavior Clusters:");
    let mut cluster_list: Vec<_> = clusters.values().collect();
    cluster_list.sort_by_key(|v| -(v.len() as isize));

    for (i, cluster) in cluster_list.iter().enumerate() {
        println!("- Cluster {} ({} countries): {:?}", i + 1, cluster.len(), &cluster[..cluster.len().min(5)]);
    }
}

// Saves all clusters to a text file.
// Inputs:
// - reference to graph
// - output file name
// Outputs: txt file written to disk 
pub fn save_clusters_to_file(graph: &Graph<String, f64>, filename: &str) {
    let mut uf = UnionFind::new(graph.node_count());

    for edge in graph.edge_references() {
        let (a, b) = (edge.source().index(), edge.target().index());
        uf.union(a, b);
    }

    let mut clusters: HashMap<usize, Vec<String>> = HashMap::new();
    for node_idx in graph.node_indices() {
        let root = uf.find(node_idx.index());
        clusters.entry(root)
            .or_default()
            .push(graph[node_idx].clone());
    }

    let file = File::create(filename).expect("Failed to create output file.");
    let mut writer = BufWriter::new(file);

    let mut cluster_list: Vec<_> = clusters.values().collect();
    cluster_list.sort_by_key(|v| -(v.len() as isize));

    for (i, cluster) in cluster_list.iter().enumerate() {
        writeln!(writer, "Cluster {} ({} countries):", i + 1, cluster.len())
            .expect("Failed to write to file.");
        for country in *cluster {
            writeln!(writer, "- {}", country)
                .expect("Failed to write to file.");
        }
        writeln!(writer, "").expect("Failed to write newline.");
    }

    println!("✅  Clusters saved to '{}'", filename);
    println!("The file contains a list of all clusters.");
}
