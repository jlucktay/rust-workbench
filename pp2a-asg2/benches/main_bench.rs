use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use pp2a_asg2::fibonacci;

fn criterion_benchmark(c: &mut Criterion) {
	c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
	c.bench_function("fib 200", |b| b.iter(|| fibonacci(black_box(200))));
	c.bench_function("fib 2_000", |b| b.iter(|| fibonacci(black_box(2_000))));
	c.bench_function("fib 20_000", |b| b.iter(|| fibonacci(black_box(20_000))));
	c.bench_function("fib 200_000", |b| b.iter(|| fibonacci(black_box(200_000))));
	c.bench_function("fib 2_000_000", |b| {
		b.iter(|| fibonacci(black_box(2_000_000)));
	});
}

const fn do_something(_size: usize) {
	// Do something with the size
}

fn from_elem(c: &mut Criterion) {
	let size: usize = 1024;

	c.bench_with_input(BenchmarkId::new("input_example", size), &size, |b, &s| {
		b.iter(|| do_something(s));
	});
}

criterion_group!(benches, criterion_benchmark, from_elem);
criterion_main!(benches);
