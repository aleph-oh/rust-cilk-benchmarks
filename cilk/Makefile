# Feel free to define relevant environment variables in .env.
include .env

ifndef CILK_CC
    $(error Please set CILK_CC to use this benchmark suite!)
endif

ifndef CILK_LD
    $(error Please set CILK_LD to use this benchmark suite!)
endif

OS := $(shell uname)
ifeq ($(OS),Darwin)
	CC := xcrun $(CILK_CC)
else
	CC := $(CILK_CC)
endif

CFLAGS = -fopencilk -g
LDFLAGS=--ld-path=$(CILK_LD)

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

PPS_DEPS = prefix_sum.h pps_bench_lib.h
PPS_OBJS = prefix_sum.o pps_bench_lib.o
PPS_CS_OBJS = prefix_sum_cs.o pps_bench_lib_cs.o
PPS_CS_BENCH_OBJS = prefix_sum_cs_bench.o pps_bench_lib_cs_bench.o

FIB_DEPS = fib_lib.h
FIB_OBJS = fib_lib.o
FIB_CS_OBJS = fib_lib_cs.o
FIB_CS_BENCH_OBJS = fib_lib_cs_bench.o

DEPS = $(PPS_DEPS) $(FIB_DEPS)

all: all_pps all_fib

all_pps: pps_bench pps_bench_serial pps_bench_single pps_test

all_fib: fib_bench fib

%.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS)

%_cs.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS) -fcilktool=cilkscale 

%_cs_bench.o: %.c $(DEPS)
	$(CC) -c -o $@ $< $(CFLAGS) -fcilktool=cilkscale-benchmark 

pps_bench: $(PPS_OBJS) pps_bench.o
	$(CC) -o $@ $^ $(CFLAGS) -flto $(LDFLAGS)

pps_bench_serial: $(PPS_OBJS) pps_bench_serial.o
	$(CC) -o $@ $^ $(CFLAGS) -flto $(LDFLAGS)

pps_bench_single: $(PPS_OBJS) pps_bench_single.o
	$(CC) -o $@ $^ $(CFLAGS) -flto $(LDFLAGS)

pps_test: $(PPS_OBJS) pps_test.o
	$(CC) -o $@ $^ $(CFLAGS) $(LDFLAGS)

pps_bench_cs: $(PPS_CS_OBJS) pps_bench_cs.o
	$(CC) -o $@ $^ $(CFLAGS) -fcilktool=cilkscale $(LDFLAGS)

pps_bench_cs_bench: $(PPS_CS_BENCH_OBJS) pps_bench_cs_bench.o
	$(CC) -o $@ $^ $(CFLAGS) -fcilktool=cilkscale-benchmark $(LDFLAGS)

fib: $(FIB_OBJS) fib.o
	$(CC) -o $@ $^ $(CFLAGS) $(LDFLAGS)

fib_bench: $(FIB_OBJS) fib_bench.o
	$(CC) -o $@ $^ $(CFLAGS) $(LDFLAGS)

fib_bench_cs: $(FIB_CS_OBJS) fib_bench.o
	$(CC) -o $@ $^ $(CFLAGS) -fcilktool=cilkscale $(LDFLAGS)

fib_bench_cs_bench: $(FIB_CS_BENCH_OBJS) fib_bench.o
	$(CC) -o $@ $^ $(CFLAGS) -fcilktool=cilkscale-benchmark $(LDFLAGS)

.PHONY: all_fib all_pps clean scale pps_scale fib_scale

clean:
	rm -f *.o pps_bench pps_test pps_bench_cs_bench pps_bench_cs pps_bench_serial pps_bench_single fib_bench fib_bench_cs fib_bench_cs_bench fib

scale: pps_scale fib_scale

pps_scale: pps_bench_cs pps_bench_cs_bench

fib_scale: fib_bench_cs fib_bench_cs_bench
