#![feature(cilk)]

pub mod pps;
pub mod fib {
    const SERIAL_CUTOFF: usize = 10;

    pub fn cilk_fib(n: usize) -> usize {
        match n {
            0 | 1 => n,
            2..=SERIAL_CUTOFF => cilk_fib(n - 1) + cilk_fib(n - 2),
            _ => {
                let x = cilk_spawn { cilk_fib(n - 1) };
                let y = cilk_fib(n - 2);
                cilk_sync;
                x + y
            }
        }
    }

    pub fn cilk_scope_fib(n: usize) -> usize {
        match n {
            0 | 1 => n,
            2..=SERIAL_CUTOFF => cilk_scope_fib(n - 1) + cilk_scope_fib(n - 2),
            _ => cilk_scope {
                    let x = cilk_spawn { cilk_scope_fib(n - 1) };
                    let y = cilk_scope_fib(n - 2);
                    cilk_sync;
                    x + y
                }
        }
    }

    pub fn rayon_fib(n: usize) -> usize {
        match n {
            0 | 1 => n,
            2..=SERIAL_CUTOFF => rayon_fib(n - 1) + rayon_fib(n - 2),
            _ => {
                let (x, y) = rayon::join(|| rayon_fib(n - 1), || rayon_fib(n - 2));
                x + y
            }
        }
    }

    pub fn rayon_spawn_fib(n: usize) -> usize {
        match n {
            0 | 1 => n,
            2..=SERIAL_CUTOFF => rayon_spawn_fib(n - 1) + rayon_spawn_fib(n - 2),
            _ => {
                let mut x: Option<usize> = None;
                let mut y: Option<usize> = None;
                rayon::scope(|s| {
                    s.spawn(|_| x = Some(rayon_spawn_fib(n - 1)));
                    s.spawn(|_| y = Some(rayon_spawn_fib(n - 2)));
                });
                x.zip(y).map(|(x, y)| x + y).expect("closure should've completed!")
            }
        }
    }
}
