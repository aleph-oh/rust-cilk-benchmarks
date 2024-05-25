#include <stdint.h>
#include <stddef.h>

typedef void (*prefix_sum_f)(int32_t *, size_t);
void bench(int num_rums, int size, prefix_sum_f f);
