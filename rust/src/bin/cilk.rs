use pps_lib::common::ParallelScan;
use pps_lib::cilk::CilkJoiner;

fn main() {
    // Does a few runs of the Cilk benchmark to determine parallel scalability.
    let mut last_sum = 0;
    // We run this many times so we're above hyperfine's minimum time.
    for _ in 0..5 {
        let mut a = (0..1_000_000).collect::<Vec<_>>();
        ParallelScan::<CilkJoiner>::scan(&mut a, 0, i32::wrapping_add);
        last_sum += a.last().unwrap();
    }
    println!("Cilk benchmark complete. a[last] = {}", last_sum / 5); 
}
