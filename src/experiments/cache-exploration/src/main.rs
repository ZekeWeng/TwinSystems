mod bandwidth;
mod cacheline;
mod latency;

use bandwidth::measure_bandwidth;
use cacheline::{measure_stride_read, measure_stride_write};
use harness::{AlignedBuffer, KB, MB, WallTimer};
use latency::measure_pointer_chase;

// BANDWIDTH constants
const EPOCHS: u32 = 10;
const SAMPLES: usize = 10;
const STREAM_ELEMENTS: usize = 16 * MB;

// CACHELINESIZE constants
const STRIDE_BUFFER: usize = 512 * MB;
const STRIDE_MAX: usize = 512;
const CACHE_LINE: usize = 64;

// LATENCY constants
const LATENCY_ITERS: u64 = 50_000_000;
const LATENCY_EPOCHS: u32 = 5;
const LATENCY_SEED: u64 = 0x00C0_FFEE_BEEF;
const LATENCY_SIZES: &[usize] = &[
    4 * KB,
    8 * KB,
    16 * KB,
    32 * KB,
    64 * KB,
    128 * KB,
    256 * KB,
    512 * KB,
    MB,
    2 * MB,
    4 * MB,
    8 * MB,
    16 * MB,
    32 * MB,
];

fn main() {
    let timer = WallTimer;

    println!("Part 1 — RAM bandwidth (STREAM, n={SAMPLES} samples, GB/s)");
    println!("{:─<55}", "");
    println!("{:<8}{:>12}{:>12}{:>12}", "", "min", "median", "max");
    let bw = measure_bandwidth(&timer, STREAM_ELEMENTS, EPOCHS, SAMPLES);
    for (label, s) in [("Copy", &bw.copy), ("Triad", &bw.triad)] {
        let (min, median, max) = (s.min_gbs, s.median_gbs, s.max_gbs);
        println!("{label:<8}{min:>10.3}{median:>11.3}{max:>11.3}");
    }
    println!();

    println!("Part 2 — Cache-line size (sequential stride, ns/access)");
    println!("{:─<55}", "");
    println!("{:>8}{:>12}{:>12}", "stride", "read", "write");
    let mut buf = AlignedBuffer::<u8>::new(STRIDE_BUFFER, CACHE_LINE);
    let mut step = 1;
    while step <= STRIDE_MAX {
        let read_ns = measure_stride_read(&timer, &buf, step, EPOCHS);
        let write_ns = measure_stride_write(&timer, &mut buf, step, EPOCHS);
        println!("{step:>8}{read_ns:>12.3}{write_ns:>12.3}");
        step *= 2;
    }
    drop(buf);
    println!();

    println!("Part 3 — Load-to-use latency (random pointer chase, ns/access)");
    println!("{:─<55}", "");
    println!("{:>12}{:>14}", "WS", "ns/access");
    for &ws in LATENCY_SIZES {
        let ns = measure_pointer_chase(&timer, ws, LATENCY_ITERS, LATENCY_EPOCHS, LATENCY_SEED);
        println!("{:>12}{ns:>14.3}", format_bytes(ws));
    }
}

fn format_bytes(n: usize) -> String {
    if n >= MB {
        format!("{} MB", n / MB)
    } else {
        format!("{} KB", n / KB)
    }
}
