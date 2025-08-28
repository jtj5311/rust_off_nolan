use rand::Rng;
use std::time::Instant;

fn setup(n: usize) -> Vec<f64> {
    // Uniform in [0,1), like np.random.rand
    let mut rng = rand::thread_rng();
    let mut v = vec![0.0f64; n];
    for x in &mut v {
        *x = rng.gen::<f64>();
    }
    v
}

/// Safe baseline. Panics if `steps > array.len()` (close to Python behavior).

/// Faster: removes per-iteration bounds checks. Caller must ensure `steps <= array.len()`.
#[inline]
fn hot_loop_unchecked(array: &[f64], steps: usize, n: usize) -> f64 {
    let mut acc = 0.0f64;
    for i in 0..steps {
        // SAFETY:eliminates bounds checks in the loop.
        unsafe {
            acc += *array.get_unchecked(i % n);
        }
    }
    acc
}

fn main() {
    // Match your defaults
    let n = 1_000_000usize;
    let steps = 2 * 1_000_000usize;

    let array = setup(n);
    // I'm letting numba do this so meh, let's do it here too.
    let _out = hot_loop_unchecked(&array, steps, n);

    // Choose either implementation:
    let t0 = Instant::now();
    let out = hot_loop_unchecked(&array, steps, n);
    let dt = t0.elapsed();

    println!("sum = {out}, elapsed = {:.3?}", dt);
}
