//! # CSV Processing Module
//!
//! This module provides efficient CSV file processing capabilities,
//! demonstrating high-performance data parsing and analysis using Rust.

use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

/// Represents a passenger record from the Titanic dataset
#[derive(Debug, Deserialize, Clone)]
pub struct TitanicPassenger {
    /// Passenger ID number
    #[serde(rename = "PassengerId")]
    pub passenger_id: u32,

    /// Survival status (0 = No, 1 = Yes)
    #[serde(rename = "Survived")]
    pub survived: u32,

    /// Ticket class (1 = 1st, 2 = 2nd, 3 = 3rd)
    #[serde(rename = "Pclass")]
    pub pclass: u32,

    /// Passenger name
    #[serde(rename = "Name")]
    pub name: String,

    /// Passenger sex
    #[serde(rename = "Sex")]
    pub sex: String,

    /// Passenger age in years
    #[serde(rename = "Age")]
    pub age: Option<f32>,

    /// Number of siblings/spouses aboard
    #[serde(rename = "SibSp")]
    pub sib_sp: u32,

    /// Number of parents/children aboard
    #[serde(rename = "Parch")]
    pub parch: u32,

    /// Ticket number
    #[serde(rename = "Ticket")]
    pub ticket: String,

    /// Passenger fare
    #[serde(rename = "Fare")]
    pub fare: f32,

    /// Cabin number
    #[serde(rename = "Cabin")]
    pub cabin: Option<String>,

    /// Port of embarkation (C = Cherbourg, Q = Queenstown, S = Southampton)
    #[serde(rename = "Embarked")]
    pub embarked: Option<String>,
}

/// Statistics about Titanic passengers
#[derive(Debug, Clone, PartialEq)]
pub struct TitanicStatistics {
    /// Total number of passengers
    pub total_passengers: u32,
    /// Number of survivors
    pub survived_passengers: u32,
    /// Number of male passengers
    pub male_passengers: u32,
    /// Number of female passengers
    pub female_passengers: u32,
    /// Survival rate as a percentage
    pub survival_rate: f32,
}

impl TitanicStatistics {
    /// Creates a new TitanicStatistics instance
    pub fn new() -> Self {
        Self {
            total_passengers: 0,
            survived_passengers: 0,
            male_passengers: 0,
            female_passengers: 0,
            survival_rate: 0.0,
        }
    }

    /// Calculates the survival rate percentage
    pub fn calculate_survival_rate(&mut self) {
        if self.total_passengers > 0 {
            self.survival_rate =
                (self.survived_passengers as f32 / self.total_passengers as f32) * 100.0;
        }
    }
}

impl Default for TitanicStatistics {
    fn default() -> Self {
        Self::new()
    }
}

/// Processes a Titanic CSV file and calculates statistics
///
/// # Arguments
///
/// * `file_path` - Path to the CSV file
///
/// # Returns
///
/// Returns `Ok(())` on success, or an error if processing fails
///
/// # Examples
///
/// ```no_run
/// use rust_high_performance_data_processing::csv_processing::process_titanic_csv;
///
/// match process_titanic_csv("./data/titanic.csv") {
///     Ok(_) => println!("Processing complete"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
pub fn process_titanic_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let stats = analyze_titanic_csv(file_path)?;

    println!("\n--- Titanic Data Analysis ---");
    println!(
        "Total passengers processed: {}",
        stats.total_passengers
    );
    println!(
        "Passengers who survived: {}",
        stats.survived_passengers
    );
    println!("Survival rate: {:.2}%", stats.survival_rate);
    println!("Male passengers: {}", stats.male_passengers);
    println!("Female passengers: {}", stats.female_passengers);
    println!("------------------------------------\n");

    Ok(())
}

/// Analyzes a Titanic CSV file and returns statistics
///
/// # Arguments
///
/// * `file_path` - Path to the CSV file
///
/// # Returns
///
/// Returns `TitanicStatistics` on success, or an error if processing fails
pub fn analyze_titanic_csv(file_path: &str) -> Result<TitanicStatistics, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path)?;

    let mut stats = TitanicStatistics::new();

    for result in rdr.deserialize() {
        let passenger: TitanicPassenger = result?;
        stats.total_passengers += 1;

        if passenger.survived == 1 {
            stats.survived_passengers += 1;
        }

        if passenger.sex == "male" {
            stats.male_passengers += 1;
        } else if passenger.sex == "female" {
            stats.female_passengers += 1;
        }
    }

    stats.calculate_survival_rate();
    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_titanic_statistics_new() {
        let stats = TitanicStatistics::new();
        assert_eq!(stats.total_passengers, 0);
        assert_eq!(stats.survived_passengers, 0);
        assert_eq!(stats.male_passengers, 0);
        assert_eq!(stats.female_passengers, 0);
        assert_eq!(stats.survival_rate, 0.0);
    }

    #[test]
    fn test_survival_rate_calculation() {
        let mut stats = TitanicStatistics {
            total_passengers: 100,
            survived_passengers: 38,
            male_passengers: 60,
            female_passengers: 40,
            survival_rate: 0.0,
        };
        stats.calculate_survival_rate();
        assert!((stats.survival_rate - 38.0).abs() < 0.01);
    }

    #[test]
    fn test_survival_rate_zero_passengers() {
        let mut stats = TitanicStatistics::new();
        stats.calculate_survival_rate();
        assert_eq!(stats.survival_rate, 0.0);
    }

    #[test]
    fn test_analyze_titanic_csv_actual_file() {
        // This test uses the actual Titanic dataset
        let result = analyze_titanic_csv("./data/titanic.csv");

        // The file should exist and be processable
        assert!(result.is_ok());

        let stats = result.unwrap();

        // Verify expected values from the Titanic dataset
        assert_eq!(stats.total_passengers, 891);
        assert_eq!(stats.survived_passengers, 342);
        assert_eq!(stats.male_passengers, 577);
        assert_eq!(stats.female_passengers, 314);
        assert!((stats.survival_rate - 38.38).abs() < 0.1);
    }

    #[test]
    fn test_process_nonexistent_file() {
        let result = analyze_titanic_csv("./data/nonexistent.csv");
        assert!(result.is_err());
    }
}
