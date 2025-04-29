//! main.rs
//! Entry point of project

mod csv_loader;
mod country;
mod similarity;
mod graph;
mod utils;

// Does dataset loading, cleaning, graph construction, analysis, and saving to txt file.
fn main() {
    println!("📄 Loading and cleaning data from owid-co2-data.csv...");
    let raw_data = csv_loader::load_data("owid-co2-data.csv");
    let processed = utils::clean_data(raw_data);
    println!("✅ Successfully parsed {} countries.", processed.len());

    let threshold = 0.975;
    println!("🔗 Building similarity graph (threshold = {:.2})...", threshold);
    let graph = graph::build_similarity_graph(&processed, threshold);

    println!("📊 Graph Statistics:");
    graph::print_graph_stats(&graph);
    graph::print_degree_centrality(&graph);
    graph::print_clusters(&graph);
    graph::save_clusters_to_file(&graph, "clusters_output.txt");
}

mod tests {
    use crate::country::CountryData;
    use crate::similarity::cosine_similarity;
    use crate::utils::{smooth, clean_data};
    use crate::graph::build_similarity_graph;
    use petgraph::graph::NodeIndex;
    use std::collections::HashMap;

    /// Test that cosine similarity returns 1.0 for identical vectors.
    #[test]
    fn test_cosine_similarity_identical() {
        let a = CountryData { values: vec![1.0, 2.0, 3.0] };
        let b = CountryData { values: vec![1.0, 2.0, 3.0] };
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-6, "Expected similarity to be 1.0, got {}", sim);
    }

    /// Test that cosine similarity returns around 0.0 for orthogonal vectors.
    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = CountryData { values: vec![1.0, 0.0] };
        let b = CountryData { values: vec![0.0, 1.0] };
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-6, "Expected similarity close to 0.0, got {}", sim);
    }

    /// Test that smoothing does not change vector length and smooths properly.
    #[test]
    fn test_smooth_basic() {
        let mut v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        smooth(&mut v, 1);
        assert_eq!(v.len(), 5);
        assert!((v[2] - 3.0).abs() < 1e-6, "Expected middle value to be smoothed properly");
    }

    /// Test clean_data produces aligned, smoothed, normalized vectors.
    #[test]
    fn test_clean_data_simple() {
        let mut input = HashMap::new();
        input.insert(
            "TestCountry".to_string(),
            vec![(2000, 10.0), (2001, 20.0)],
        );

        let cleaned = clean_data(input);
        assert!(cleaned.contains_key("TestCountry"));
        let country = cleaned.get("TestCountry").unwrap();
        assert_eq!(country.values.len() >= 2, true, "Should have aligned multiple years.");
        assert!(country.values.iter().all(|&v| v <= 1.0), "Values should be normalized to max=1.");
    }

    /// Test building a simple graph connects similar countries.
    #[test]
    fn test_graph_building_simple() {
        let mut countries = HashMap::new();
        countries.insert(
            "A".to_string(),
            CountryData { values: vec![1.0, 2.0, 3.0] }
        );
        countries.insert(
            "B".to_string(),
            CountryData { values: vec![1.0, 2.0, 3.0] }
        );
        countries.insert(
            "C".to_string(),
            CountryData { values: vec![0.0, 0.0, 0.0] }
        );

        let graph = build_similarity_graph(&countries, 0.95);

        // Check nodes exist
        assert_eq!(graph.node_count(), 3);

        // A and B should be connected
        let a_idx = graph.node_indices().find(|idx| graph[*idx] == "A").unwrap();
        let b_idx = graph.node_indices().find(|idx| graph[*idx] == "B").unwrap();
        assert!(graph.find_edge(a_idx, b_idx).is_some(), "A and B should be connected.");

        // A and C should not be connected
        let c_idx = graph.node_indices().find(|idx| graph[*idx] == "C").unwrap();
        assert!(graph.find_edge(a_idx, c_idx).is_none(), "A and C should NOT be connected.");
    }
} 