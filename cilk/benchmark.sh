#! /bin/bash

make
hyperfine './bench_single' --warmup 5 --export-markdown bench_single.md -N
hyperfine -P num_threads 1 16 'CILK_NWORKERS={num_threads} ./bench_single' --warmup 5 --export-markdown bench_single_parallelism.md
./bench