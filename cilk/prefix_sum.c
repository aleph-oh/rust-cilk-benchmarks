#include "prefix_sum.h"
#include <cilk/cilk.h>

#define UPSWEEP_CUTOFF 1
#define DOWNSWEEP_CUTOFF 1
#define REV_CUTOFF 1024
#define ROTATE_CUTOFF 1024

// TODO: would be nice if this was variadic and spawned all except the last one.
#define MAYBE_PAR(n, CUTOFF, f1, f2)                                           \
  if ((n) <= (CUTOFF)) {                                                       \
    (f1);                                                                      \
    (f2);                                                                      \
  } else {                                                                     \
    cilk_scope {                                                               \
      cilk_spawn(f1);                                                          \
      (f2);                                                                    \
    }                                                                          \
  }

void serial(int32_t *arr, size_t n) {
  for (int i = 1; i < n; i++) {
    arr[i] += arr[i - 1];
  }
}

static void up_sweep(int32_t *arr, size_t n) {
  if (n > 1) {
    int32_t *left = arr;
    size_t mid = n / 2;
    int32_t *right = arr + mid;
    MAYBE_PAR(mid, UPSWEEP_CUTOFF, up_sweep(left, mid),
              up_sweep(right, n - mid))

    arr[n - 1] += arr[mid - 1];
  }
}

static void down_sweep(int32_t *arr, int32_t p, size_t n) {
  if (n == 0) {
    return;
  } else if (n == 1) {
    *arr = p;
  } else {
    size_t mid = n / 2;
    int32_t left_acc = arr[mid - 1];
    int32_t *left = arr;
    int32_t *right = arr + mid;
    MAYBE_PAR(mid, DOWNSWEEP_CUTOFF, down_sweep(left, p, mid),
              down_sweep(right, p + left_acc, n - mid))
  }
}

static void rev_half(int32_t *arr, size_t n, size_t i, size_t j)
{
  if ((j - i) * 2 <= REV_CUTOFF)
  {
    for (int k = i; k < j; k++)
    {
      int32_t tmp = arr[k];
      arr[k] = arr[n - k - 1];
      arr[n - k - 1] = tmp;
    }

    return;
  }

  size_t left_len = (j - i) / 2;
  size_t right_len = j - i - left_len;
  size_t q1_len = left_len / 2;
  size_t q2_len = left_len - q1_len;
  size_t q3_len = right_len / 2;
  size_t q4_len = right_len - q3_len;
  if (q1_len != q4_len || q2_len != q3_len)
  {
    for (int k = i; k < j; k++)
    {
      int32_t tmp = arr[k];
      arr[k] = arr[n - k - 1];
      arr[n - k - 1] = tmp;
    }
    return;
  }

  cilk_scope
  {
    cilk_spawn rev_half(arr, n, i, (i + j) / 2);
    rev_half(arr, n, (i + j) / 2, j);
  }
}

static void rev(int32_t *arr, size_t n)
{
  rev_half(arr, n, 0, n / 2);
}

static void rotate_left(int32_t *arr, size_t n, size_t offset) {
  offset %= n;
  // We could be conditionally parallel here, but I don't think it helps us much
  // since rotate_left is never called recursively.
  cilk_scope {
    cilk_spawn rev(arr, offset);
    rev(arr + offset, n - offset);
  }
  rev(arr, n);
}

static int32_t parallel_excl(int32_t *arr, size_t n) {
  up_sweep(arr, n);
  int32_t acc = arr[n - 1];
  down_sweep(arr, 0, n);
  return acc;
}

void parallel(int32_t *arr, size_t n) {
  if (n == 0) {
    return;
  }

  int32_t acc = parallel_excl(arr, n);
  rotate_left(arr, n, 1);
  arr[n - 1] = acc;
}
