#![feature(cilk)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, PlotConfiguration};

const SERIAL_CUTOFF: usize = 10;

fn cilk_fib(n: usize) -> usize {
    match n {
        0 | 1 => n,
        2..=SERIAL_CUTOFF => cilk_fib(n - 1) + cilk_fib(n - 2),
        _ => {
            let x = cilk_spawn { cilk_fib(n - 1) };
            let y = cilk_fib(n - 2);
            cilk_sync;
            x + y
        }
    }
}

fn rayon_fib(n: usize) -> usize {
    match n {
        0 | 1 => n,
        2..=SERIAL_CUTOFF => rayon_fib(n - 1) + rayon_fib(n - 2),
        _ => {
            let (x, y) = rayon::join(|| rayon_fib(n - 1), || rayon_fib(n - 2));
            x + y
        }
    }
}

fn bench_fibs(c: &mut Criterion) {
    // Use a log scale because the asmyptomotic complexity is exponential.
    let plot_config = PlotConfiguration::default().summary_scale(criterion::AxisScale::Logarithmic);
    let mut group = c.benchmark_group("Fibonacci");
    group.plot_config(plot_config);
    for i in [15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 35, 40].iter() {
        group.bench_with_input(BenchmarkId::new("Rust-Cilk", i), i, |b, i| {
            b.iter(|| cilk_fib(black_box(*i)));
        });
        group.bench_with_input(BenchmarkId::new("Rayon", i), i, |b, i| {
            b.iter(|| rayon_fib(black_box(*i)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
