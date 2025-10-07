// Rust High-Performance Data Processing
// Author: Gabriel Demetrios Lafis
// Year: 2025

use std::time::Instant;

#[derive(Debug)]
struct DataRecord {
    id: u32,
    value: f64,
}

fn process_data(records: &[DataRecord]) -> f64 {
    records.iter().map(|r| r.value).sum::<f64>() / records.len() as f64
}

fn main() {
    println!("===========================================");
    println!("Rust High-Performance Data Processing");
    println!("===========================================");

    let start = Instant::now();

    // Generate sample data
    let records: Vec<DataRecord> = (0..1_000_000)
        .map(|i| DataRecord {
            id: i,
            value: (i as f64) * 1.5,
        })
        .collect();

    let avg = process_data(&records);
    let duration = start.elapsed();

    println!("Processed {} records", records.len());
    println!("Average value: {:.2}", avg);
    println!("Time elapsed: {:?}", duration);
    println!("===========================================");
}
