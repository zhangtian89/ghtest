#![allow(unused)]

pub use criterion::{black_box, criterion_group, criterion_main, Criterion};
pub use ghtest::*;

pub mod bench_chunk_util {
    use super::*;

    pub fn get_sizes() -> Vec<usize> {
        action_input().map_or_else(
            || vec![20_usize /* 1 MiB */],
            |x| {
                x.split(",")
                    .map(|x| x.trim().parse())
                    .collect::<Result<_, _>>()
                    .expect("Invalid input")
            },
        )
    }

    pub fn bench_chunk<T: Clone, R, M: criterion::measurement::Measurement>(
        group: &mut criterion::BenchmarkGroup<M>,
        name: &str,
        mut init: impl FnMut(usize) -> T,
        mut bench: impl FnMut(T) -> R,
    ) {
        let sizes = get_sizes();
        for size in sizes {
            let data = init(1 << size);
            group.bench_function(format!("{} (1 << {})", name, size), |b| {
                b.iter_batched(
                    || data.clone(),
                    |data| {
                        let data = black_box(data);
                        let data = bench(data);
                        black_box(data)
                    },
                    criterion::BatchSize::SmallInput,
                )
            });
        }
    }
}
