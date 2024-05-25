#include <cilk/cilk.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "fib_lib.h"

static void usage(const char *const binary_name)
{
    printf("Usage: %s <n> <num_runs> <scope|noscope>\n", binary_name);
}

static size_t bench(int n, int num_runs, size_t (*fib)(int))
{
    size_t total = 0;
    for (int i = 0; i < num_runs; i++)
    {
        cilk_scope
        {
            size_t x = fib(n);
            total += x;
        }
    }

    return total;
}

int main(int argc, char **argv)
{
    // First, parse the arguments. We expect the first argument to be the
    // number we're computing the Fibonacci number of, the second argument
    // to be the number of times we should run the computation, and the third
    // to be if we should be using Cilk with or without cilk_scope.
    if (argc != 4)
    {
        usage(argv[0]);
        return 1;
    }

    int n = atoi(argv[1]);
    int num_runs = atoi(argv[2]);

    // Parsing the third argument is a bit more complicated. We expect it to
    // be either "scope" or "noscope". This means we need a size-8 buffer
    // to store the argument.
    int const use_cilk_scope_len = strlen(argv[3]);
    if (use_cilk_scope_len > 7)
    {
        usage(argv[0]);
        return 1;
    }
    char use_cilk_scope[8];
    strncpy(use_cilk_scope, argv[3], sizeof use_cilk_scope);
    if (use_cilk_scope[7] != '\0')
    {
        usage(argv[0]);
        return 1;
    }

    size_t result;
    if (memcmp(use_cilk_scope, "scope", 5) == 0)
    {
        result = bench(n, num_runs, fib_scope);
    }
    else if (memcmp(use_cilk_scope, "noscope", 7) == 0)
    {
        result = bench(n, num_runs, fib_noscope);
    }
    else
    {
        usage(argv[0]);
        return 1;
    }

    printf("total: %lu\n", result);
}
