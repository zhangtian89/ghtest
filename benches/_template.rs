#![allow(unused)]

pub use criterion::{black_box, criterion_group, criterion_main, Criterion};
pub use ghtest::*;

fn bench_(c: &mut Criterion) {
    let mut group = c.benchmark_group("test");
    let data = gen_vec(32);
    let key = gen_array::<32>();

    group.bench_function("test", |b| {
        b.iter(|| {
            let x = black_box(0..10000_usize)
                .enumerate()
                .map(|(x, y)| x * 2 + y)
                .sum::<usize>();
            black_box(x);
        })
    });

    group.finish();
}

criterion_group!(benches, bench_);
criterion_main!(benches);
