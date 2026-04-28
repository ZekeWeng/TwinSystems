use core::hint::black_box;

use harness::{AlignedBuffer, BenchRng, Timer, shuffle};

const NODE_USIZES: usize = 16;
const NODE_BYTES: usize = NODE_USIZES * size_of::<usize>();

pub fn measure_pointer_chase(
    timer: &impl Timer,
    working_set_bytes: usize,
    iterations: u64,
    epochs: u32,
    seed: u64,
) -> f64 {
    assert!(working_set_bytes >= 2 * NODE_BYTES);
    assert!(working_set_bytes.is_multiple_of(NODE_BYTES));
    assert!(epochs > 0);

    let n_nodes = working_set_bytes / NODE_BYTES;
    let mut buf = AlignedBuffer::<usize>::new(n_nodes * NODE_USIZES, NODE_BYTES);

    // Build a cycle
    let mut order: Vec<usize> = (0..n_nodes).collect();
    let mut rng = BenchRng::new(seed);
    shuffle(&mut order, &mut rng);
    for k in 0..n_nodes {
        let from = order[k] * NODE_USIZES;
        let to = order[(k + 1) % n_nodes] * NODE_USIZES;
        buf[from] = to;
    }
    let start = order[0] * NODE_USIZES;

    // Warmup: walk the entire cycle once to fault pages
    {
        let mut i = start;
        for _ in 0..n_nodes {
            i = buf[i];
        }
        black_box(i);
    }

    // Pointer Chase
    let mut samples_ns = Vec::with_capacity(epochs as usize);
    for _ in 0..epochs {
        let (_, dur) = timer.measure(|| {
            let mut i = start;
            for _ in 0..iterations {
                i = buf[i];
            }
            black_box(i);
        });
        black_box(buf.as_ptr());
        samples_ns.push(dur.as_nanos() as f64 / iterations as f64);
    }
    samples_ns.sort_by(|a, b| a.partial_cmp(b).unwrap());
    samples_ns[samples_ns.len() / 2]
}
