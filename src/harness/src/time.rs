use std::hint::black_box;
use std::time::{Duration, Instant};

pub trait Timer {
    fn measure<F, R>(&self, f: F) -> (R, Duration)
    where
        F: FnOnce() -> R;
}

pub struct WallTimer;

impl Timer for WallTimer {
    #[inline]
    fn measure<F, R>(&self, f: F) -> (R, Duration)
    where
        F: FnOnce() -> R,
    {
        let start = Instant::now();
        let out = black_box(f());
        (out, start.elapsed())
    }
}
