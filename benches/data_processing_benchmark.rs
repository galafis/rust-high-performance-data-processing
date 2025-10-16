use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rust_high_performance_data_processing::{csv_processing, process_data, DataRecord};

fn benchmark_process_data(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_processing");

    for size in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let records: Vec<DataRecord> = (0..size)
                .map(|i| DataRecord {
                    id: i as u32,
                    value: (i as f64) * 1.5,
                })
                .collect();

            b.iter(|| process_data(black_box(&records)));
        });
    }

    group.finish();
}

fn benchmark_csv_processing(c: &mut Criterion) {
    c.bench_function("csv_titanic_analysis", |b| {
        b.iter(|| {
            csv_processing::analyze_titanic_csv(black_box("./data/titanic.csv"))
                .expect("CSV processing failed")
        });
    });
}

fn benchmark_data_generation(c: &mut Criterion) {
    c.bench_function("generate_1M_records", |b| {
        b.iter(|| {
            let _records: Vec<DataRecord> = (0..1_000_000)
                .map(|i| DataRecord {
                    id: i,
                    value: (i as f64) * 1.5,
                })
                .collect();
        });
    });
}

criterion_group!(
    benches,
    benchmark_process_data,
    benchmark_csv_processing,
    benchmark_data_generation
);
criterion_main!(benches);
