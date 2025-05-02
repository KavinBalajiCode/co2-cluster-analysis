// utils.rs
// Handles cleaning, smoothing, filling missing values, and normalizing emission data.
// Also contains 5 tests

use crate::country::CountryData;
use std::collections::{HashMap, HashSet};

// Smooths a vector using a moving average.
// Inputs:
// - values (vector to smooth)
// - window (size of smoothing window)
// Outputs: Smoothed vector 
pub fn smooth(values: &mut Vec<f64>, window: usize) {
    let mut smoothed = values.clone();
    for i in 0..values.len() {
        let start = i.saturating_sub(window);
        let end = (i + window + 1).min(values.len());
        let sum: f64 = values[start..end].iter().sum();
        smoothed[i] = sum / (end - start) as f64;
    }
    *values = smoothed;
}

// Cleans and prepares the emission dataset.
// Inputs: raw HashMap of country 
// Outputs: cleaned HashMap of country 
pub fn clean_data(raw_data: HashMap<String, Vec<(i32, f64)>>) -> HashMap<String, CountryData> {
    let mut all_years = HashSet::new();
    for values in raw_data.values() {
        for (year, _) in values {
            all_years.insert(*year);
        }
    }
    let mut all_years: Vec<i32> = all_years.into_iter().collect();
    all_years.sort();

    let mut processed = HashMap::new();
    for (country, values) in raw_data {
        let year_to_value: HashMap<i32, f64> = values.into_iter().collect();
        let mut co2_vector = Vec::new();
        for year in &all_years {
            if let Some(val) = year_to_value.get(year) {
                co2_vector.push(*val);
            } else {
                co2_vector.push(f64::NAN);
            }
        }

        // Fill missing values with average
        let avg = co2_vector.iter().filter(|v| !v.is_nan()).sum::<f64>() 
                / co2_vector.iter().filter(|v| !v.is_nan()).count().max(1) as f64;
        for v in &mut co2_vector {
            if v.is_nan() {
                *v = avg;
            }
        }

        // Smooth the series
        smooth(&mut co2_vector, 1);

        // Normalize to unit scale
        let max = co2_vector.iter().cloned().fold(0.0 / 0.0, f64::max);
        if max > 0.0 {
            for v in &mut co2_vector {
                *v /= max;
            }
        }

        processed.insert(
            country.clone(),
            CountryData {
                values: co2_vector,
            },
        );
    }

    processed

}

// Contains 5 unit tests to check core project functionality
#[cfg(test)]
mod tests {
    use crate::country::CountryData;
    use crate::similarity::cosine_similarity;
    use crate::utils::{smooth, clean_data};
    use crate::graph::build_similarity_graph;
    use std::collections::HashMap;

    // Test that cosine similarity returns 1.0 for identical vectors.
    #[test]
    fn test_cosine_similarity_identical() {
        let a = CountryData { values: vec![1.0, 2.0, 3.0] };
        let b = CountryData { values: vec![1.0, 2.0, 3.0] };
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 1.0).abs() < 1e-6, "Expected similarity to be 1.0, got {}", sim);
    }

    // Test that cosine similarity returns around 0.0 for orthogonal vectors.
    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = CountryData { values: vec![1.0, 0.0] };
        let b = CountryData { values: vec![0.0, 1.0] };
        let sim = cosine_similarity(&a, &b);
        assert!(sim.abs() < 1e-6, "Expected similarity close to 0.0, got {}", sim);
    }

    // Test that smoothing does not change vector length and smooths properly.
    #[test]
    fn test_smooth_basic() {
        let mut v = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        smooth(&mut v, 1);
        assert_eq!(v.len(), 5);
        assert!((v[2] - 3.0).abs() < 1e-6, "Expected middle value to be smoothed properly");
    }

    // Test clean_data produces aligned, smoothed, normalized vectors.
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

    // Test building a simple graph connects similar countries.
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