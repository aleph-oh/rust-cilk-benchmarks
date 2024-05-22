use crate::common::Joiner;
pub struct RayonJoiner;

impl Joiner for RayonJoiner {
    #[inline]
    fn join<A: Send, B: Send>(f: impl FnOnce() -> A + Send, g: impl FnOnce() -> B + Send) -> (A, B) {
        rayon::join(f, g)
    }
}