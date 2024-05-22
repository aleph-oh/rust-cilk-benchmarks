#include "prefix_sum.h"
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void print_arr(int32_t *arr, int n) {
  for (int i = 0; i < n - 1; i++) {
    printf("%d,", arr[i]);
  }
  printf("%d\n", arr[n - 1]);
}

static int test(int32_t *arr, int n) {
  int32_t *arr2 = malloc(n * sizeof(*arr));
  memcpy(arr2, arr, sizeof(*arr) * n);
  serial(arr, n);
  parallel(arr2, n);
  int ret = memcmp(arr, arr2, sizeof(*arr) * n);
  if (ret != 0) {
    printf("expected:");
    print_arr(arr, n);
    printf("actual:");
    print_arr(arr2, n);
  }
  free(arr2);
  return ret;
}

typedef struct {
  int32_t *vector;
  int n;
  bool do_free;
} spec_t;

static spec_t empty(void) {
  spec_t spec;
  spec.vector = NULL;
  spec.n = 0;
  spec.do_free = false;
  return spec;
}

static spec_t k_elt(int k) {
  int *vector = malloc(k * sizeof(int32_t));
  for (int i = 0; i < k; i++) {
    vector[i] = i + 1;
  }
  spec_t spec;
  spec.vector = vector;
  spec.n = k;
  spec.do_free = true;
  return spec;
}

static int32_t TEST_VECTOR[] = {6, 4, 16, 10, 16, 14, 2, 8};
static spec_t TEST_SPEC = {.vector = TEST_VECTOR, .n = 8, .do_free = false};

int main() {
  spec_t vectors[] = {empty(),  k_elt(1), k_elt(2),  k_elt(3),   k_elt(4),
                      k_elt(5), k_elt(6), k_elt(10), TEST_VECTOR};
  int fails = 0;
  for (int i = 0; i < sizeof(vectors) / sizeof(*vectors); i++) {
    if (test(vectors[i].vector, vectors[i].n)) {
      printf("test failed :(\n");
      fails += 1;
    }
  }
  for (int i = 0; i < sizeof(vectors) / sizeof(*vectors); i++) {
    if (vectors[i].do_free) {
      free(vectors[i].vector);
    }
  }

  if (fails > 0) {
    printf("failed %d of %lu tests\n", fails,
           sizeof(vectors) / sizeof(*vectors));
  } else {
    printf("all tests passed! :)\n");
  }
  return fails != 0;
}
