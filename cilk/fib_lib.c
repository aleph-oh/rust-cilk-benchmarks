#include "fib_lib.h"
#include <cilk/cilk.h>
#include <stddef.h>

size_t fib_scope(int n)
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

size_t fib_noscope(int n)
{
    if (n <= 1)
    {
        return n;
    }
    else if (n <= 10)
    {
        return fib_noscope(n - 1) + fib_noscope(n - 2);
    }
    else
    {
        size_t const x = cilk_spawn fib_noscope(n - 1);
        size_t const y = fib_noscope(n - 2);
        cilk_sync;
        return x + y;
    }
}
