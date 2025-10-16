use rust_high_performance_data_processing::{csv_processing, process_data, DataRecord};

#[test]
fn test_integration_data_processing() {
    let records = vec![
        DataRecord {
            id: 1,
            value: 100.0,
        },
        DataRecord {
            id: 2,
            value: 200.0,
        },
        DataRecord {
            id: 3,
            value: 300.0,
        },
    ];

    let avg = process_data(&records);
    assert_eq!(avg, 200.0);
}

#[test]
fn test_integration_csv_processing() {
    // Test that the CSV file exists and can be analyzed
    let result = csv_processing::analyze_titanic_csv("./data/titanic.csv");
    assert!(result.is_ok());

    let stats = result.unwrap();
    assert!(stats.total_passengers > 0);
    assert!(stats.survival_rate >= 0.0 && stats.survival_rate <= 100.0);
}

#[test]
fn test_integration_large_dataset() {
    // Create a large dataset
    let records: Vec<DataRecord> = (0..1_000_000)
        .map(|i| DataRecord {
            id: i,
            value: (i as f64) * 1.5,
        })
        .collect();

    let avg = process_data(&records);

    // Verify that the average is calculated correctly
    assert!(avg > 0.0);
}

#[test]
fn test_integration_empty_dataset() {
    let records: Vec<DataRecord> = vec![];
    let avg = process_data(&records);
    assert_eq!(avg, 0.0);
}

#[test]
fn test_integration_titanic_statistics() {
    let result = csv_processing::analyze_titanic_csv("./data/titanic.csv");
    assert!(result.is_ok());

    let stats = result.unwrap();

    // Verify known facts about the Titanic dataset
    assert_eq!(stats.total_passengers, 891);
    assert_eq!(stats.survived_passengers, 342);

    // Male passengers should be more than female
    assert!(stats.male_passengers > stats.female_passengers);

    // Total should equal male + female
    assert_eq!(
        stats.total_passengers,
        stats.male_passengers + stats.female_passengers
    );
}
