use core::hint::black_box;

use harness::Timer;

pub fn measure_stride_read(timer: &impl Timer, arr: &[u8], step: usize, epochs: u32) -> f64 {
    assert!(step > 0);
    let (_, dur) = timer.measure(|| {
        let mut acc: u8 = 0;
        for _ in 0..epochs {
            let mut i = 0;
            while i < arr.len() {
                acc = acc.wrapping_add(arr[i]);
                i += step;
            }
        }
        black_box(acc);
    });
    let accesses = (arr.len() / step) * epochs as usize;
    dur.as_nanos() as f64 / accesses as f64
}

pub fn measure_stride_write(timer: &impl Timer, arr: &mut [u8], step: usize, epochs: u32) -> f64 {
    assert!(step > 0);
    let (_, dur) = timer.measure(|| {
        for _ in 0..epochs {
            let mut i = 0;
            while i < arr.len() {
                arr[i] = i as u8;
                i += step;
            }
        }
        black_box(arr.as_ptr());
    });
    let accesses = (arr.len() / step) * epochs as usize;
    dur.as_nanos() as f64 / accesses as f64
}
