// csv_loader.rs
// Handles loading and parsing the CO2 emissions CSV dataset.

use std::collections::HashMap;
use serde::Deserialize;

// Represents a single record from the CO2 dataset.
#[derive(Debug, Deserialize)]
pub struct Record {
    pub country: String,
    pub year: i32,
    pub co2_per_capita: Option<f64>,
}

// Loads and parses the dataset from a given CSV file path.
// Inputs: Path to the CSV file
// Outputs:  HashMap of country -> vector of (year, co2_per_capita) pairs
pub fn load_data(path: &str) -> HashMap<String, Vec<(i32, f64)>> {
    let mut rdr = csv::Reader::from_path(path).expect("Failed to open CSV file.");
    let mut data: HashMap<String, Vec<(i32, f64)>> = HashMap::new();

    for result in rdr.deserialize() {
        let record: Record = result.expect("Failed to deserialize CSV record.");
        if let Some(co2) = record.co2_per_capita {
            // Group by country name
            data.entry(record.country.clone())
                .or_default()
                .push((record.year, co2));
        }
    }

    // Sort the emissions data chronologically for each country
    for values in data.values_mut() {
        values.sort_by_key(|k| k.0);
    }

    data
}
