// similarity.rs
// Finds cosine similarity between two emission profiles.

use crate::country::CountryData;

// Computes cosine similarity between two countries' emissions.
// Inputs: a, b: references to CountryData
// Outputs: cosine similarity with values between -1 and 1
pub fn cosine_similarity(a: &CountryData, b: &CountryData) -> f64 {
    let mut dot = 0.0;
    let mut norm_a = 0.0;
    let mut norm_b = 0.0;

    // Computes the dot products and norms
    for (x, y) in a.values.iter().zip(&b.values) {
        dot += x * y;
        norm_a += x * x;
        norm_b += y * y;
    }

    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        dot / (norm_a.sqrt() * norm_b.sqrt())
    }
}
