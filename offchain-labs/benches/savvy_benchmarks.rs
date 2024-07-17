use criterion::{black_box, criterion_group, criterion_main, Criterion};
use offchain_labs::{Config, OffchainLabs};

fn benchmark_transaction_processing(c: &mut Criterion) {
    let config = Config {
        zk_params_path: "bench_params.json".to_string(),
        state_db_path: "bench_state.db".to_string(),
    };

    let mut hvm = OffchainLabs::new(config).unwrap();
    let transaction = vec![1, 2, 3, 4];

    c.bench_function("process transaction", |b| {
        b.iter(|| {
            hvm.process_transaction(black_box(&transaction)).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_transaction_processing);
criterion_main!(benches);