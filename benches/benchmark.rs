use advent_of_code_2023::DAYS;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    for (i, [part1, part2]) in DAYS.iter().enumerate() {
        let input = std::fs::read_to_string(format!("./inputs/input_{:02}.txt", i + 1))
            .expect("Could not open input file");

        c.bench_function(&format!("day{:02}::part1", i + 1), |b| {
            b.iter(|| part1(black_box(&input)))
        });

        c.bench_function(&format!("day{:02}::part2", i + 1), |b| {
            b.iter(|| part2(black_box(&input)))
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
