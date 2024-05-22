CC=xcrun /Users/jay/Code/MEng/opencilk/build/bin/clang

CFLAGS = -fopencilk -g

DEBUG ?= 0
ifneq ($(DEBUG), 0)
	CFLAGS += -DDEBUG -Og
else
	CFLAGS += -O3 -march=native -DNDEBUG 
endif

ASAN ?= 0
ifneq ($(ASAN), 0)
	CFLAGS += -fsanitize=address
endif

CILKSAN ?= 0
ifneq ($(CILKSAN), 0)
	CFLAGS += -fsanitize=cilk
endif

DEPS = prefix_sum.h bench_lib.h
OBJS = prefix_sum.o bench_lib.o
CS_OBJS = prefix_sum_cs.o bench_lib_cs.o
CS_BENCH_OBJS = prefix_sum_cs_bench.o bench_lib_cs_bench.o

all: bench bench_serial bench_single test

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

%_cs.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS) -fcilktool=cilkscale

%_cs_bench.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS) -fcilktool=cilkscale-benchmark

bench: $(OBJS) bench.o
	$(CC) -o $@ $^ $(CFLAGS) -flto

bench_serial: $(OBJS) bench_serial.o
	$(CC) -o $@ $^ $(CFLAGS) -flto

bench_single: $(OBJS) bench_single.o
	$(CC) -o $@ $^ $(CFLAGS) -flto

test: $(OBJS) test.o
	$(CC) -o $@ $^ $(CFLAGS)

bench_cs: $(CS_OBJS) bench_cs.o
	$(CC) -o $@ $^ $(CFLAGS) -fcilktool=cilkscale

bench_cs_bench: $(CS_BENCH_OBJS) bench_cs_bench.o
	$(CC) -o $@ $^ $(CFLAGS) -fcilktool=cilkscale-benchmark

fib: fib.c
	$(CC) -o fib fib.c $(CFLAGS)

.PHONY: clean scale

clean:
	rm -f *.o bench test bench_cs_bench bench_cs bench_serial bench_single fib

# TODO: make sure you can generate visible plots.
# There's some issue where it can't open the csv
# of the outputs to make a plot, but I'm not sure
# why this happens.
scale: bench_cs bench_cs_bench