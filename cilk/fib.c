#include <cilk/cilk.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "fib_lib.h"

#define STRCAP(S) (sizeof(S) - 1)
#define MAX(X, Y) ((X) > (Y) ? (X) : (Y))

char const SCOPE[] = "scope";
char const NOSCOPE[] = "noscope";

static void usage(const char *const binary_name)
{
    printf("Usage: %s <n> <num_runs> <%s|%s>\n", binary_name, SCOPE, NOSCOPE);
}

static size_t run(int n, int num_runs, size_t (*fib)(int))
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
    // be either SCOPE or NOSCOPE, and we compute at compile-time the
    // expected length of the longest string.

    char use_cilk_scope[MAX(sizeof(SCOPE), sizeof(NOSCOPE))];

    if (strlen(argv[3]) > STRCAP(use_cilk_scope))
    {
        usage(argv[0]);
        return 1;
    }
    strncpy(use_cilk_scope, argv[3], sizeof(use_cilk_scope));
    // Because of the above check, we now know that the buffer must be null-terminated,
    // since what we copied into it was at most as long as the buffer and strncpy
    // pads with null.

    size_t result;
    // Last, let's make sure that the argument is either SCOPE or NOSCOPE.
    // Based on whichever one it is, we call the appropriate function.
    if (memcmp(use_cilk_scope, SCOPE, STRCAP(SCOPE)) == 0)
    {
        result = run(n, num_runs, fib_scope);
    }
    else if (memcmp(use_cilk_scope, NOSCOPE, STRCAP(NOSCOPE)) == 0)
    {
        result = run(n, num_runs, fib_noscope);
    }
    else
    {
        usage(argv[0]);
        return 1;
    }

    printf("total: %lu\n", result);
}
