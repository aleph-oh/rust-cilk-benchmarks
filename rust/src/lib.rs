#![feature(cilk)]
#![feature(maybe_uninit_slice)]
#![feature(backtrace_frames)]

pub mod common;
pub mod rayon;
pub mod cilk;
pub mod serial;
