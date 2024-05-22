use crate::common::Joiner;

pub struct CilkJoiner;

impl Joiner for CilkJoiner {
    #[inline]
    fn join<A: Send, B: Send>(f: impl FnOnce() -> A + Send, g: impl FnOnce() -> B + Send) -> (A, B) {
        let a = cilk_spawn { f() };
        let b = g();
        cilk_sync;
        (a, b)
    }
}