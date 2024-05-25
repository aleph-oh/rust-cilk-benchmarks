This directory contains microbenchmarks for Cilk programs to compare them against Rust + Cilk implementations in the `rust` directory contained in the same parent.

**Setup**
Add a file .env that sets one variable CILK\_CC. It should look something like the following:
```
CILK_CC=path/to/your/opencilk/compiler
```
On Linux, you may also need to install `lld`. To do this, follow either the instructions for LLVM or for OpenCilk on how to add additional projects and add `lld` as a project. Otherwise, LTO may not work as expected.
