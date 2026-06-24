use criterion::{black_box, criterion_group, criterion_main, Criterion};
use increasing_array::{increasing_array, increasing_array_naive};

fn benchmark_greedy(c: &mut Criterion) {
    let data = vec![3, 2, 5, 1, 7, 4, 8, 2, 10, 1];

    c.bench_function("greedy", |b| {
        b.iter(|| increasing_array(black_box(&data)))
    });
}

fn benchmark_naive(c: &mut Criterion) {
    let data = vec![3, 2, 5, 1, 7, 4, 8, 2, 10, 1];

    c.bench_function("naive", |b| {
        b.iter(|| increasing_array_naive(black_box(&data)))
    });
}

criterion_group!(benches, benchmark_greedy, benchmark_naive);
criterion_main!(benches);