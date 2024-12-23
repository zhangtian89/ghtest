#![allow(unused)]

use chrono::{DateTime, Local, Utc};
use criterion::{
    black_box, criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup, Criterion,
};
pub use ghtest::*;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const SAMPLE_SIZE: usize = 20;

macro_rules! try_duration {
    ($duration: expr) => {
        match $duration {
            Ok(duration) => duration,
            Err(_) => panic!("{} before UNIX EPOCH!", stringify!($duration)),
        }
    };
}

macro_rules! benching {
    ($group: expr, $make: expr, $name:tt $(,)?) => {
        $group.bench_function(
            ($name),
            |b| {
                b.iter(|| $make)
            },
        );
    };
    ($(@inner,)? $group: expr, $make: expr, $name:tt, $func: ident $(,)?) => {
        $group.bench_function(
            format!("{}::{}", ($name), stringify!($func)),
            |b| {
                b.iter(|| $make.$func())
            },
        );
    };
    ($(@inner,)? $group: expr, $make: expr, $name:tt, with $v:ident => $func: expr => $func_name: tt $(,)?) => {
        $group.bench_function(
            format!("{}::{}", ($name), ($func_name)),
            |b| {
                b.iter(|| {
                    let $v = $make;
                    $func
                })
            },
        );
    };
    ($(@inner,)? $group: expr, $make: expr, $name:tt, with $v:ident => $func: expr => $func_name: tt, $($etc: tt)+) => {
        benching!($group, $make, $name, with $v=> $func => $func_name);
        benching!(@inner, $group, $make, $name, $($etc)+);
    };
    ($(@inner,)? $group: expr, $make: expr, $name:tt, $func: ident, $($etc: tt)+) => {
        benching!($group, $make, $name, $func);
        benching!(@inner, $group, $make, $name, $($etc)+);
    };
}

fn bench_duration(c: &mut Criterion) {
    let instant = Instant::now();
    let utc = Utc::now();
    let local = Local::now();
    {
        let mut group = c.benchmark_group("std::time::Duration");
        group.sample_size(SAMPLE_SIZE);
        benching!(
            &mut group,
            try_duration!(SystemTime::now().duration_since(UNIX_EPOCH)),
            "std::time::SystemTime",
            as_secs, as_millis, as_micros, as_nanos,
            as_secs_f32, as_secs_f64,
            with d => (d.as_secs(), d.subsec_millis()) => "(as_secs, subsec_millis)",
            with d => (d.as_secs(), d.subsec_micros()) => "(as_secs, subsec_micros)",
            with d => (d.as_secs(), d.subsec_nanos()) => "(as_secs, subsec_nanos)",
        );
        benching!(
            &mut group,
            instant.elapsed(),
            "std::time::Instant",
            as_secs, as_millis, as_micros, as_nanos,
            as_secs_f32, as_secs_f64,
            with d => (d.as_secs(), d.subsec_millis()) => "(as_secs, subsec_millis)",
            with d => (d.as_secs(), d.subsec_micros()) => "(as_secs, subsec_micros)",
            with d => (d.as_secs(), d.subsec_nanos()) => "(as_secs, subsec_nanos)",
        );
        group.finish();
    }
    {
        let mut group = c.benchmark_group("chrono::TimeDelta");
        group.sample_size(SAMPLE_SIZE);
        benching!(
            &mut group,
            utc.signed_duration_since(Utc::now()),
            "chrono::Utc",
            num_weeks,
            num_days,
            num_hours,
            num_minutes,
            num_seconds,
            num_milliseconds,
            num_microseconds,
            num_nanoseconds,
            with d => (d.num_seconds(), d.subsec_nanos()) => "(num_seconds, subsec_nanos)",
        );
        benching!(
            &mut group,
            local.signed_duration_since(Local::now()),
            "chrono::Local",
            num_weeks,
            num_days,
            num_hours,
            num_minutes,
            num_seconds,
            num_milliseconds,
            num_microseconds,
            num_nanoseconds,
            with d => (d.num_seconds(), d.subsec_nanos()) => "(num_seconds, subsec_nanos)",
        );
        group.finish();
    }
}

fn bench_create(c: &mut Criterion) {
    let mut group = c.benchmark_group("Time create instance");
    group.sample_size(SAMPLE_SIZE);

    benching!(&mut group, SystemTime::now(), "std::time::SystemTime",);
    benching!(&mut group, Instant::now(), "std::time::Instant",);
    benching!(&mut group, Utc::now(), "chrono::Utc",);
    benching!(&mut group, Local::now(), "chrono::Local",);

    group.finish();
}

fn bench_timestamp(c: &mut Criterion) {
    let mut group = c.benchmark_group("Timestamp");
    group.sample_size(SAMPLE_SIZE);

    benching!(
        &mut group,
        try_duration!(SystemTime::now().duration_since(UNIX_EPOCH)),
        "std::time::SystemTime",
        as_secs, as_millis, as_micros, as_nanos,
        as_secs_f32, as_secs_f64,
        with d => (d.as_secs(), d.subsec_millis()) => "(as_secs, subsec_millis)",
        with d => (d.as_secs(), d.subsec_micros()) => "(as_secs, subsec_micros)",
        with d => (d.as_secs(), d.subsec_nanos()) => "(as_secs, subsec_nanos)",
    );
    benching!(
        &mut group,
        Utc::now(),
        "chrono::Utc",
        timestamp,
        timestamp_millis,
        timestamp_micros,
        timestamp_nanos_opt
    );
    benching!(
        &mut group,
        Local::now(),
        "chrono::Local",
        timestamp,
        timestamp_millis,
        timestamp_micros,
        timestamp_nanos_opt
    );

    group.finish();
}

criterion_group!(benches, bench_create, bench_duration, bench_timestamp);
criterion_main!(benches);
