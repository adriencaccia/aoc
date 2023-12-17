use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_2023::*;

pub fn day16(c: &mut Criterion) {
    let input = include_str!("../src/day16/input.txt");
    c.bench_function("day16", |b| b.iter(|| day16::parse_input(black_box(input))));
    c.bench_function("day16-p1", |b| b.iter(|| day16::part1(black_box(input))));
    c.bench_function("day16-p2", |b| b.iter(|| day16::part2(black_box(input))));
}

pub fn day17(c: &mut Criterion) {
    let input = include_str!("../src/day17/input.txt");
    c.bench_function("day17", |b| b.iter(|| day17::parse_input(black_box(input))));
    c.bench_function("day17-p1", |b| b.iter(|| day17::part1(black_box(input))));
    c.bench_function("day17-p2", |b| b.iter(|| day17::part2(black_box(input))));
}

criterion_group!(benches, day16, day17);
criterion_main!(benches);
