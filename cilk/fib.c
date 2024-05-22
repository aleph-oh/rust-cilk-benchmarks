#include <cilk/cilk.h>
#include <stdlib.h>
#include <stddef.h>
#include <stdio.h>
#include <time.h>

static inline size_t fib_scope(int n)
{
    if (n <= 1)
    {
        return n;
    }
    else if (n <= 10)
    {
        return fib_scope(n - 1) + fib_scope(n - 2);
    }
    else
    {
        size_t x;
        size_t y;
        cilk_scope
        {
            x = cilk_spawn fib_scope(n - 1);
            y = fib_scope(n - 2);
        }
        return x + y;
    }
}

static inline size_t fib(int n)
{
    if (n <= 1)
    {
        return n;
    }
    else if (n <= 10)
    {
        return fib(n - 1) + fib(n - 2);
    }
    else
    {
        size_t const x = cilk_spawn fib(n - 1);
        size_t const y = fib(n - 2);
        cilk_sync;
        return x + y;
    }
}

static int64_t time_ns(struct timespec t)
{
    return 1000 * 1000 * 1000 * t.tv_sec + t.tv_nsec;
}

typedef size_t (*fib_t)(int);

void bench(int const num_runs, fib_t fib)
{
    int const n[] = {15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 30, 35, 40};
    size_t total = 0;
    for (int i = 0; i < sizeof(n) / sizeof(*n); i++)
    {
        struct timespec t1, t2;
        int64_t delta = 0;
        cilk_scope
        {
            for (int t = 0; t < num_runs; t++)
            {
                clock_gettime(CLOCK_MONOTONIC_RAW, &t1);
                int x = fib(n[i]);
                clock_gettime(CLOCK_MONOTONIC_RAW, &t2);
                total += x;
                delta += time_ns(t2) - time_ns(t1);
            }
        }
        printf("n: %d\n\ttime elapsed per run: %lld ns\n", n[i], delta / num_runs);
    }

    printf("total (to prevent optimizing this out): %lu\n", total);
}

int main()
{
    printf("benchmarking fib without cilk_scope\n");
    bench(100, fib);
    printf("benchmarking fib with cilk_scope\n");
    bench(100, fib_scope);
}