This directory contains microbenchmarks for Cilk programs to compare them against Rust + Cilk implementations in the `rust` directory contained in the same parent.

**Setup**
Add a file .env that sets two variables CILK\_CC and CILK\_LD. It should look something like the following:
```
CILK_CC=path/to/your/opencilk/compiler
CILK_LD=path/to/your/linker
```

**Running Benchmarks**
There are a few benchmarks in this directory:
- `benchmark.sh` runs the below benchmarking binaries with `hyperfine` to get more statistically useful results and is analagous to the similarly-named file in the sibling `rust` directory.
- `fib` runs fib(n) on the user-specified input for a specified number of runs, optionally using cilk_scope rather than cilk_sync.
- `fib_bench` runs fib(n) for a few interesting values of n to benchmark work-heavy tasks.
- `pps_bench` runs a parallel prefix sum over a varying number of elements many times to smooth out variation in runtime.
- `pps_bench_single` runs a parallel prefix sum over 1M elements a few times.
- `pps_bench_serial` runs a serial prefix sum over 1M elements a few times.
- `pps_test` runs a simple test suite for parallel prefix sum.

