#include "bench_lib.h"
#include "prefix_sum.h"

int main()
{
    bench(5, 1000000, parallel);
}