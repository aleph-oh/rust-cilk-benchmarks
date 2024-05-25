use super::common::Joiner;
pub struct SerialJoiner;
impl Joiner for SerialJoiner {
    #[inline]
    fn join<A: Send, B: Send>(f: impl FnOnce() -> A + Send, g: impl FnOnce() -> B + Send) -> (A, B) {
        (f(), g())
    }
}