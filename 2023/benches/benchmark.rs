use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_2023::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    let input = include_str!("../src/day16/input.txt");
    c.bench_function("day16", |b| b.iter(|| day16::parse_input(black_box(input))));
    c.bench_function("day16-p1", |b| b.iter(|| day16::part1(black_box(input))));
    c.bench_function("day16-p2", |b| b.iter(|| day16::part2(black_box(input))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
