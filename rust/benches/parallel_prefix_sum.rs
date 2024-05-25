use criterion::BenchmarkGroup;
use criterion::{
    black_box, criterion_group, criterion_main, BatchSize, Bencher, BenchmarkId, Criterion,
    Throughput,
};

use bench_lib::pps::prelude::*;

fn bench_with<F>(b: &mut Bencher, f: F)
where
    F: Fn(&mut [i32]),
{
    b.iter_batched_ref(
        || (0..1_000_000).collect::<Vec<_>>(),
        |a| f(a),
        BatchSize::SmallInput,
    );
}

fn serial_scan<T, F>(s: &mut [T], id: T, f: F)
where
    T: Copy + Send + Sync,
    F: Fn(T, T) -> T + Sync + Copy {
        ParallelScan::<SerialJoiner, 1, 1, 1, 1>::scan(s, id, f);
}

fn rayon_scan<T, F>(s: &mut [T], id: T, f: F)
where
    T: Copy + Send + Sync,
    F: Fn(T, T) -> T + Send + Sync + Copy {
        ParallelScan::<RayonJoiner, 1, 1, 1024, 1024>::scan(s, id, f);
}

fn cilk_scan<T, F>(s: &mut [T], id: T, f: F)
where 
    T: Copy + Send + Sync,
    F: Fn(T, T) -> T + Send + Sync + Copy {
        ParallelScan::<CilkJoiner, 1, 1, 1024, 1024>::scan(s, id, f);
}

pub fn serial(c: &mut Criterion) {
    c.bench_function("serial_scan(A) where A.len() = 1,000,000", |b| {
        bench_with(b, |a| serial_scan(black_box(a), 0, i32::wrapping_add))
    });
}

pub fn parallel_rayon(c: &mut Criterion) {
    c.bench_function("rayon_scan(A) where A.len() = 1,000,000", |b| {
        bench_with(b, |a| rayon_scan(black_box(a), 0, i32::wrapping_add))
    });
}

pub fn parallel_cilk(c: &mut Criterion) {
    c.bench_function("cilk_scan(A) where A.len() = 1,000,000", |b| {
        bench_with(b, |a| cilk_scan(black_box(a), 0, i32::wrapping_add))
    });
}

fn bench_scaling<M: criterion::measurement::Measurement>(
    group: &mut BenchmarkGroup<M>,
    sizes: &[u64],
    f: impl Fn(&mut [i32]),
) {
    for &size in sizes {
        group.throughput(Throughput::Elements(size));
        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            &(size as i32),
            |b, &size| {
                b.iter_batched_ref(
                    || (0..size).collect::<Vec<_>>(),
                    |a| f(a),
                    BatchSize::SmallInput,
                )
            },
        );
    }
}

static SCALING_SIZES: [u64; 6] = [
    1_000_000, 2_000_000, 4_000_000, 8_000_000, 16_000_000, 32_000_000,
];

pub fn parallel_rayon_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("rayon with varying length");
    bench_scaling(&mut group, &SCALING_SIZES, |a| {
        rayon_scan(black_box(a), 0, i32::wrapping_add)
    });
    group.finish();
}

pub fn parallel_cilk_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("cilk with varying length");
    bench_scaling(&mut group, &SCALING_SIZES, |a| {
        cilk_scan(black_box(a), 0, i32::wrapping_add)
    });
    group.finish();
}

criterion_group!(all,
serial,
parallel_rayon,
parallel_cilk,
parallel_rayon_scaling,
parallel_cilk_scaling,
);

criterion_main!(all);
