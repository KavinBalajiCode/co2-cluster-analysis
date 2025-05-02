// main.rs
// Entry point of project

mod csv_loader;
mod country;
mod similarity;
mod graph;
mod utils;

// Does dataset loading, cleaning, graph construction, analysis, and saving to txt file.
fn main() {
    println!(" ");
    println!("📄 Loading and cleaning data from owid-co2-data.csv...");
    let raw_data = csv_loader::load_data("owid-co2-data.csv");
    let processed = utils::clean_data(raw_data);
    println!("✅ Successfully parsed {} countries and regions.", processed.len());

    let threshold = 0.975;
    println!("🔗 Building similarity graph (threshold = {:.3})...", threshold);
    let graph = graph::build_similarity_graph(&processed, threshold);

    println!("📊 Graph Statistics:");
    graph::print_graph_stats(&graph);
    graph::print_degree_centrality(&graph);
    graph::print_clusters(&graph);
    graph::save_clusters_to_file(&graph, "clusters_output.txt");
}
