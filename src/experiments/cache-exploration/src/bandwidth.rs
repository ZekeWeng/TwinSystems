use core::hint::black_box;

use harness::{AlignedBuffer, Timer};

const CACHE_LINE_SIZE: usize = 128;

#[derive(Debug, Clone, Copy)]
pub struct Stats {
    pub min_gbs: f64,
    pub median_gbs: f64,
    pub max_gbs: f64,
}

pub struct BandwidthResult {
    pub copy: Stats,
    pub triad: Stats,
}

pub fn measure_bandwidth(
    timer: &impl Timer,
    elements: usize,
    epochs: u32,
    samples: usize,
) -> BandwidthResult {
    let mut a = AlignedBuffer::<f64>::new(elements, CACHE_LINE_SIZE);
    let mut b = AlignedBuffer::<f64>::new(elements, CACHE_LINE_SIZE);
    let mut c = AlignedBuffer::<f64>::new(elements, CACHE_LINE_SIZE);

    // Fault every page so we don't time first-touch.
    for (i, x) in a.iter_mut().enumerate() {
        *x = i as f64;
    }
    for (i, x) in b.iter_mut().enumerate() {
        *x = i as f64;
    }
    for (i, x) in c.iter_mut().enumerate() {
        *x = i as f64;
    }

    let q = 3.0_f64;
    let bytes_per_pass = a.byte_len();
    let copy_bytes = 2 * bytes_per_pass * epochs as usize;
    let triad_bytes = 3 * bytes_per_pass * epochs as usize;

    timer.measure(|| {
        for _ in 0..epochs {
            for i in 0..elements {
                a[i] = b[i];
            }
            black_box(a.as_ptr());
        }
    });
    let mut copy_gbs = Vec::with_capacity(samples);
    for _ in 0..samples {
        let (_, d) = timer.measure(|| {
            for _ in 0..epochs {
                for i in 0..elements {
                    a[i] = b[i];
                }
                black_box(a.as_ptr());
            }
        });
        copy_gbs.push(copy_bytes as f64 / d.as_secs_f64() / 1e9);
    }

    timer.measure(|| {
        for _ in 0..epochs {
            for i in 0..elements {
                a[i] = b[i].mul_add(q, c[i]);
            }
            black_box(a.as_ptr());
        }
    });
    let mut triad_gbs = Vec::with_capacity(samples);
    for _ in 0..samples {
        let (_, d) = timer.measure(|| {
            for _ in 0..epochs {
                for i in 0..elements {
                    a[i] = b[i].mul_add(q, c[i]);
                }
                black_box(a.as_ptr());
            }
        });
        triad_gbs.push(triad_bytes as f64 / d.as_secs_f64() / 1e9);
    }

    BandwidthResult {
        copy: stats(&mut copy_gbs),
        triad: stats(&mut triad_gbs),
    }
}

fn stats(samples: &mut [f64]) -> Stats {
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    Stats {
        min_gbs: samples[0],
        median_gbs: samples[samples.len() / 2],
        max_gbs: samples[samples.len() - 1],
    }
}
