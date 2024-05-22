use std::marker::PhantomData;

pub trait Joiner {
    fn join<A: Send, B: Send>(f: impl FnOnce() -> A + Send, g: impl FnOnce() -> B + Send) -> (A, B);
}

pub struct ParallelScan<J, const UPSWEEP: usize = 1, const DOWNSWEEP: usize = 1, const ROTATE: usize = 1024, const REV: usize = 1024> {
    // We only want this type for its trait method [Joiner::join].
    _joiner: PhantomData<J>
}

impl<J, const UPSWEEP: usize, const DOWNSWEEP: usize, const ROTATE: usize, const REV: usize> ParallelScan<J, UPSWEEP, DOWNSWEEP, ROTATE, REV> {
    pub fn new() -> Self {
        ParallelScan {
            _joiner: PhantomData
        }
    }
}

impl<J: Joiner, const UPSWEEP: usize, const DOWNSWEEP: usize, const ROTATE: usize, const REV: usize> 
    ParallelScan<J, UPSWEEP, DOWNSWEEP, ROTATE, REV> {
    #[inline]
    fn join<A: Send, B: Send, const CUTOFF: usize>(n: usize, f: impl FnOnce() -> A + Send, g: impl FnOnce() -> B + Send) -> (A, B) {
        debug_assert_ne!(n, 0);
        if n <= CUTOFF {
            let a = f();
            let b = g();
            (a, b)
        } else {
            J::join(f, g)
        }
    }

    #[inline]
    fn up_sweep<T, F>(s: &mut [T], f: F) where T: Copy + Send + Sync, F: Fn(T, T) -> T + Sync + Copy {
        // T(n) = 2T(n / 2) + O(1) = O(n)
        // S(n) = S(n / 2) + O(1) = O(log n)
        match s {
            [] | [_] => {}
            _ => {
                let n = s.len();
                let mid = n / 2;
                let (left, right) = s.split_at_mut(mid);
                Self::join::<_, _, {UPSWEEP}>(n, || Self::up_sweep(left, f), || Self::up_sweep(right, f));

                // We use mid - 1 here because while the paper says
                // to use mid, it's also 1-indexed. mid is still a
                // reasonable split point because it evenly subdivides
                // the input and is end-exclusive while in the paper it
                // seems to be inclusive.
                let mid = *s.get(mid - 1).unwrap();
                let old = *s.last().unwrap();
                *s.last_mut().unwrap() = f(old, mid);
            }
        }
    }

    /// [down_sweep(s, p, f)] combines the accumulated prefixes in-place
    /// in [s] via [f], using [p] as the accumulated prefix of the first entry in [s].
    #[inline]
    fn down_sweep<T, F>(s: &mut [T], p: T, f: F)
    where
        T: Copy + Send + Sync,
        F: Fn(T, T) -> T + Sync + Copy,
    {
        // T(n) = 2T(n / 2) + O(1) = O(n)
        // S(n) = S(n / 2) + O(1) = O(log n)
        match s {
            // When s is empty, we have no work.
            // When s has one element, we should set that element to our propagated prefix.
            // When s has any other number of elements, we recurse.
            [] => {}
            [x] => *x = p,
            _ => {
                let n = s.len();
                let mid = n / 2;
                let left_acc = *s.get(mid - 1).unwrap();
                let (left, right) = s.split_at_mut(mid);
                Self::join::<_, _, {DOWNSWEEP}>(
                    n,
                    || Self::down_sweep(left, p, f),
                    || Self::down_sweep(right, f(p, left_acc), f),
                );
            }
        }
    }


    #[inline]
    fn scan_excl<T, F>(s: &mut [T], id: T, f: F) -> T
    where
        T: Copy + Send + Sync,
        F: Fn(T, T) -> T + Sync + Copy,
    {
        // T(n) = O(n)
        // S(n) = O(log n)
        match s {
            [] => id,
            _ => {
                Self::up_sweep(s, f);
                let acc = *s.last().unwrap();
                Self::down_sweep(s, id, f);
                acc
            }
        }
    }

    #[inline]
    fn rev<T: Send + Sync>(s: &mut [T]) {
        // T(n) = O(n)
        // S(n) = O(log n)
        let n = s.len();
        let (left, right) = s.split_at_mut(n / 2);
        Self::rev_half(left, right);
    }

    #[inline]
    /// Stores left, reversed, into right, and right, reversed, into left.
    fn rev_half<T: Send + Sync>(left: &mut [T], right: &mut [T]) {
        // We assume that left and right have very similar lengths, and treat n as left.len().
        // T(n) = 2T(n / 2) + O(1) = O(n)
        // S(n) = S(n / 2) + O(1) = O(log n)
        let left_len = left.len();
        if left_len + right.len() <= REV {
            left.iter_mut().zip(right.iter_mut().rev()).for_each(|(l, r)| std::mem::swap(l, r));
            return;
        }

        // Otherwise, we want to get quadrants and write into the right portions of them.
        let (q1, q2) = left.split_at_mut(left_len / 2);
        let (q3, q4) = right.split_at_mut(right.len() / 2);
        if q1.len() != q4.len() || q2.len() != q3.len() {
            // We have a different serial fallback here where we reverse left and swap them.
            // In this case the split is such that we don't have evenly sized quadrants.
            left.iter_mut().zip(right.iter_mut().rev()).for_each(|(l, r)| std::mem::swap(l, r));
            return;
        }
        Self::join::<_, _, {REV}>(left_len, 
            || Self::rev_half(q1, q4), 
            || Self::rev_half(q2, q3)
        );
    }

    #[inline]
    fn rotate_left<T: Send + Sync>(s: &mut [T], offset: usize) {
        let n = s.len();
        if n <= ROTATE {
            s.rotate_left(offset);
            return;
        }
        // T(n) = O(n)
        // S(n) = O(log n)
        let offset = offset % s.len();
        // rotate_left(A, k) = rev(concat(rev(A[..k]), rev(A[k..])))
        let (left, right) = s.split_at_mut(offset);
        Self::join::<_, _, {ROTATE}>(n, || Self::rev(left), || Self::rev(right));
        Self::rev(s);
    }

    /// [scan(s, id, f)] performs a parallel prefix scan of [s] in-place, using
    /// [id] as the identity element and [f] as the combining element.
    #[inline]
    pub fn scan<T, F>(s: &mut [T], id: T, f: F)
    where
        T: Copy + Send + Sync,
        F: Fn(T, T) -> T + Sync + Copy,
    {
        // T(n) = O(n)
        // S(n) = O(log n)
        match s {
            [] => {}
            _ => {
                let acc = Self::scan_excl(s, id, f);

                // Fix up the array, shifting all values left by 1
                // and setting the last value to the overall sum.
                Self::rotate_left(s, 1);
                *s.last_mut().unwrap() = acc;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cilk::CilkJoiner; 
    use crate::rayon::RayonJoiner;
    use crate::serial::SerialJoiner;

    #[test]
    fn test_rev_cilk() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        ParallelScan::<CilkJoiner, 1, 1, 1, 1>::rev(&mut a);
        assert_eq!(a, vec![8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_rev_rayon() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        ParallelScan::<RayonJoiner, 1, 1, 1, 1>::rev(&mut a);
        assert_eq!(a, vec![8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_rev_serial() {
        let mut a = vec![1, 2, 3, 4, 5, 6, 7, 8];
        ParallelScan::<SerialJoiner, 1, 1, 1, 1>::rev(&mut a);
        assert_eq!(a, vec![8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_rev_serial_small() {
        let mut a = vec![1, 2, 3, 4, 5, 6];
        ParallelScan::<SerialJoiner, 1, 1, 1, 1>::rev(&mut a);
        assert_eq!(a, vec![6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_rev_tiny() {
        let mut a = vec![1, 2];
        ParallelScan::<SerialJoiner, 1, 1, 1, 1>::rev(&mut a);
        assert_eq!(a, vec![2, 1]);
    }

    #[test]
    fn test_rev_large() {
        const N: usize = 32768;
        let mut input = (0..N).collect::<Vec<_>>();
        let expected = (0..N).rev().collect::<Vec<_>>();
        ParallelScan::<SerialJoiner, 1, 1, 1024, 1024>::rev(&mut input);
        assert_eq!(input, expected);
    }

    #[test]
    fn test_rev_large_cilk() {
        const N: usize = 32768;
        let mut input = (0..N).collect::<Vec<_>>();
        let expected = (0..N).rev().collect::<Vec<_>>();
        ParallelScan::<CilkJoiner, 1, 1, 1024, 1024>::rev(&mut input);
        assert_eq!(input, expected);
    }
}