#ifndef PREFIX_SUM_H_
#define PREFIX_SUM_H_

#include <stddef.h>
#include <stdint.h>

void serial(int32_t *arr, size_t n);
void parallel(int32_t *arr, size_t n);

#endif // PREFIX_SUM_H_
