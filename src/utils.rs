//! utils.rs
//! Handles cleaning, smoothing, filling missing values, and normalizing emission data.

use crate::country::CountryData;
use std::collections::{HashMap, HashSet};

/// Smooths a vector using a moving average.
/// Inputs:
/// - values (vector to smooth)
/// - window (size of smoothing window)
/// Outputs: Smoothed vector in-place
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

/// Cleans and prepares the emission dataset.
/// Inputs: raw HashMap of country -> (year, emission) pairs
/// Outputs: cleaned HashMap of country -> CountryData
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
