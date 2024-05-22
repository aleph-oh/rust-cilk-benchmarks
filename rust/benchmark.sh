#! /bin/bash

cargo build --release
echo "Comparing Cilk and Rayon with all system threads on 1M elements"
hyperfine './target/release/cilk-pps' './target/release/rayon-pps' --warmup 5 --export-markdown bench_single.md -N
echo "Comparing Cilk and Rayon with varying thread counts on 1M elements"
hyperfine -P num_threads 1 16 'CILK_NWORKERS={num_threads} ./target/release/cilk-pps' 'RAYON_NUM_THREADS={num_threads} ./target/release/rayon-pps' --warmup 5
cargo bench