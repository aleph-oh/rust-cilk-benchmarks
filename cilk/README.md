This directory contains microbenchmarks for Cilk programs to compare them against Rust + Cilk implementations in the `rust` directory contained in the same parent.

**Setup**
Add a file .env that sets one variable CILK\_CC. It should look something like the following:
```
CILK_CC=path/to/your/opencilk/compiler
```
On Linux, you may also need to install `lld`. To do this, follow either the instructions for LLVM or for OpenCilk on how to add additional projects and add `lld` as a project. Otherwise, LTO may not work as expected.

**Running Benchmarks**
There are a few benchmarks in this directory:
- `bench` runs a parallel prefix sum over a varying number of elements many times to smooth out variation in runtime.
- `bench_single` runs a parallel prefix sum over 1M elements a few times.
- `bench_serial` runs a serial prefix sum over 1M elements a few times.
- `fib` runs fib(n) for a few interesting values of n to benchmark work-heavy tasks.
- `benchmark.sh` runs the binaries with `hyperfine` to get more statistically useful results and is analagous to the similarly-named file in the sibling `rust` directory.
- `test` runs a simple test suite for parallel prefix sum.

