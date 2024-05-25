use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, ValueEnum)]
enum WhichFib {
    /// Use Cilk for the Fibonacci computation.
    Cilk,
    /// Use Rayon for the Fibonacci computation.
    Rayon,
}

#[derive(Parser)]
struct Cli {
    /// The Fibonacci number to compute.
    n: u8,
    /// The number of times to run the computation.
    num_runs: u32,
    /// Which implementation to use for the computation.
    #[arg(value_enum)]
    which: WhichFib
}

fn main() {
    let cli: Cli = Cli::parse();
    let num_runs = cli.num_runs;
    let mut sum: usize = 0;
    let fib: fn(usize) -> usize = match cli.which {
        WhichFib::Cilk => bench_lib::fib::cilk_fib,
        WhichFib::Rayon => bench_lib::fib::rayon_fib,
    };
    let n = cli.n as usize;
    for _ in 0..num_runs {
        sum += fib(n);
    }
    println!("Fibonacci sum: {}", sum);
}