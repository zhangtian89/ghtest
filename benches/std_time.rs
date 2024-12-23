use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::{Instant, SystemTime};
mod _utils;

fn benching(c: &mut Criterion) {
    let mut group = c.benchmark_group("std::time");
    benching!(&mut group, Instant::now());
    benching!(&mut group, SystemTime::now());

    let start = (Instant::now(), SystemTime::now());
    benching!(
        &mut group,
        "Instant::duration_since()",
        Instant::now().duration_since(start.0)
    );
    benching!(
        &mut group,
        "SystemTime::duration_since()",
        SystemTime::now().duration_since(start.1).unwrap()
    );

    let duration = (
        Instant::now().duration_since(start.0),
        SystemTime::now().duration_since(start.1).unwrap(),
    );
    benching!(&mut group, "Instant->as_nanos()", duration.0.as_nanos());
    benching!(&mut group, "SystemTime->as_nanos()", duration.1.as_nanos());

    group.finish();
}

criterion_group!(benches, benching);
criterion_main!(benches);
