use criterion::{black_box, criterion_group, criterion_main, Criterion};
use haranae_rs::falling_sand::rules::FallingSandRules;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Rule Generation", |b| {
        b.iter(|| black_box(FallingSandRules::default()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
