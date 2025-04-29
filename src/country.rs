//! country.rs
//! Defines the CountryData type for storing emission trend vectors.

/// Stores emission per capita values across years for a country.
#[derive(Debug, Clone)]
pub struct CountryData {
    pub values: Vec<f64>,
}
