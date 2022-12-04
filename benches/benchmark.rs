use advent_of_code::day01::{part_one, part_two};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn test<F>(c: &mut Criterion, day: u8, part: u8, mut f: F) -> &mut Criterion
where
    F: FnMut(&str) -> Option<u32>,
{
    let id = format!("Day {} Part {}", day, part);

    let input = advent_of_code::read_file("examples", day);

    if f(&input) == None {
        return c;
    }

    c.bench_function(&id, |b| b.iter(|| f(black_box(&input.to_string()))));

    return c;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    test(c, 1, 1, part_one);
    test(c, 1, 2, part_two);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
