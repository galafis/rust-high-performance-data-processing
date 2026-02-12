//! # Rust High-Performance Data Processing Library
//!
//! This library provides high-performance data processing capabilities using Rust,
//! including CSV processing and in-memory data analysis.

pub mod csv_processing;

/// Represents a single data record with an ID and value
#[derive(Debug, Clone, PartialEq)]
pub struct DataRecord {
    /// Unique identifier for the record
    pub id: u32,
    /// Numerical value associated with the record
    pub value: f64,
}

/// Calculates the average value of all records in a dataset
///
/// # Arguments
///
/// * `records` - A slice of DataRecord items to process
///
/// # Returns
///
/// The average value across all records
///
/// # Examples
///
/// ```
/// use rust_high_performance_data_processing::{DataRecord, process_data};
///
/// let records = vec![
///     DataRecord { id: 1, value: 10.0 },
///     DataRecord { id: 2, value: 20.0 },
/// ];
/// let avg = process_data(&records);
/// assert_eq!(avg, 15.0);
/// ```
pub fn process_data(records: &[DataRecord]) -> f64 {
    if records.is_empty() {
        return 0.0;
    }
    records.iter().map(|r| r.value).sum::<f64>() / records.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_data_average() {
        let records = vec![
            DataRecord { id: 1, value: 10.0 },
            DataRecord { id: 2, value: 20.0 },
            DataRecord { id: 3, value: 30.0 },
        ];
        let avg = process_data(&records);
        assert_eq!(avg, 20.0);
    }

    #[test]
    fn test_process_data_empty() {
        let records: Vec<DataRecord> = vec![];
        let avg = process_data(&records);
        assert_eq!(avg, 0.0);
    }

    #[test]
    fn test_data_record_clone() {
        let record = DataRecord { id: 1, value: 42.0 };
        let cloned = record.clone();
        assert_eq!(record, cloned);
    }
}
