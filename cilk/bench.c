#include "prefix_sum.h"
#include "bench_lib.h"


int main() {
  int const num_runs = 50;
  int const sizes[6] = { 1, 2, 4, 8, 16, 32 };
  for (int i = 0; i < sizeof(sizes) / sizeof(*sizes); i++) {
    bench(num_runs, sizes[i] * 1000000, parallel);
  }
}
