use bench_lib::pps::prelude::*;

fn main() {
    // Does a few runs of the Rayon benchmark to determine parallel scalability.
    let mut last_sum = 0;
    // We run this many times so we're above hyperfine's minimum time.
    for _ in 0..5 {
        let mut a = (0..1_000_000).collect::<Vec<_>>();
        ParallelScan::<RayonJoiner>::scan(&mut a, 0, i32::wrapping_add);
        last_sum += a.last().unwrap();
    }
    println!("Rayon benchmark complete. a[last] = {}", last_sum / 5); 
}
