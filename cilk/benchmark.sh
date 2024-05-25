#! /bin/bash

make
hyperfine './pps_bench_single' --warmup 5 --export-markdown pps_bench_single.md -N
hyperfine -P num_threads 1 16 'CILK_NWORKERS={num_threads} ./pps_bench_single' --warmup 5 --export-markdown pps_bench_single_parallelism.md
./pps_bench