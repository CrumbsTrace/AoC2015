use criterion::{black_box, criterion_group, criterion_main, Criterion};

use aoc2015::*;
use aoc2015::helper::*;

pub fn full_benchmark(c: &mut Criterion) {
    c.bench_function("All days", |b| b.iter(|| 
    {
        day1::p1(black_box(read_to_string("inputs/day1.txt")));
        day1::p2(black_box(read_to_string("inputs/day1.txt")));
        day2::p1(black_box(read_to_string("inputs/day2.txt")));
        day2::p2(black_box(read_to_string("inputs/day2.txt")));
        day3::p1(black_box(read_to_string("inputs/day3.txt")));
        day3::p2(black_box(read_to_string("inputs/day3.txt")));
        day4::p1(black_box("bgvyzdsv"));
        day4::p2(black_box("bgvyzdsv"));
        day5::p1(black_box(read_to_string("inputs/day5.txt")));
        day5::p2(black_box(read_to_string("inputs/day5.txt")));
    }));
}

criterion_group!(benches, full_benchmark);
criterion_main!(benches);