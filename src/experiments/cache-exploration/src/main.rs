mod bandwidth;
mod cacheline;

use bandwidth::measure_bandwidth;
use cacheline::{measure_stride_read, measure_stride_write};
use harness::{AlignedBuffer, MB, WallTimer};

// BANDWIDTH constants
const EPOCHS: u32 = 10;
const SAMPLES: usize = 10;
const STREAM_ELEMENTS: usize = 16 * MB;

// CACHELINESIZE constants
const STRIDE_BUFFER: usize = 512 * MB;
const STRIDE_MAX: usize = 512;
const CACHE_LINE: usize = 64;

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
}
