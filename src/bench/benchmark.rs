pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("", |b| b.iter(|| fibonacci(black_box(20))));
}
