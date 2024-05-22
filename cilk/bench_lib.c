#include "bench_lib.h"
#include <stdlib.h>
#include <stddef.h>
#include <stdio.h>
#include <time.h>

typedef void (*prefix_sum_f)(int32_t *, size_t);

static void init_arr(int32_t *arr, int n) {
  for (int i = 0; i < n; i++) {
    arr[i] = i + 1;
  }
}

static int64_t time_ns(struct timespec t) {
  return 1000 * 1000 * 1000 * t.tv_sec + t.tv_nsec;
}


void bench(int num_runs, int size, prefix_sum_f f) {
  int64_t total_time = 0;
  int discarded_runs = 0;
  int32_t *arr = malloc(size * sizeof(*arr));
  for (int i = 0; i < num_runs; i++) {
    init_arr(arr, size);
    struct timespec t1, t2;
    clock_gettime(CLOCK_MONOTONIC_RAW, &t1);
    f(arr, size);
    clock_gettime(CLOCK_MONOTONIC_RAW, &t2);
    int64_t delta = time_ns(t2) - time_ns(t1);
    if (delta > 0) {
      // This isn't a great way to measure because it completely discards
      // the cost of some runs that must have had at least 1 steal.
      total_time += delta;
    } else {
      discarded_runs += 1;
    }
  }

  free(arr);
  printf(
      "size: %d\n\ttime elapsed per run: %lld us\n\tdiscarded runs: %d of %d\n",
      size, total_time / (num_runs * 1000), discarded_runs, num_runs);
}
