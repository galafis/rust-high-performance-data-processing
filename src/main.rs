// Rust High-Performance Data Processing
// Author: Gabriel Demetrios Lafis
// Year: 2025

use rust_high_performance_data_processing::{csv_processing, process_data, DataRecord};
use std::time::Instant;

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

    // Display some sample records
    println!("\nSample records:");
    for record in records.iter().take(3) {
        println!("  Record ID: {}, Value: {:.2}", record.id, record.value);
    }
    println!("===========================================");

    // Advanced CSV Processing Example
    println!("\nIniciando exemplo de processamento CSV avançado (Titanic)...");
    let csv_file_path = "./data/titanic.csv";
    match csv_processing::process_titanic_csv(csv_file_path) {
        Ok(_) => println!("Processamento CSV do Titanic concluído com sucesso."),
        Err(e) => eprintln!("Erro ao processar CSV do Titanic: {}", e),
    }
    println!("===========================================");
}
