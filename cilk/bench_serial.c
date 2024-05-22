#include "prefix_sum.h"
#include "bench_lib.h"


int main() {
  int const num_runs = 5;
  int const sizes[6] = {1000 * 1000, 2000 * 1000,  4000 * 1000,
                        8000 * 1000, 16000 * 1000, 32000 * 1000};
  for (int i = 0; i < sizeof(sizes) / sizeof(*sizes); i++) {
    bench(num_runs, sizes[i], serial);
  }
}
