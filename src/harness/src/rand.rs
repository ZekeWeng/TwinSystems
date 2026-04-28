const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

pub struct BenchRng {
    state: u64,
}

impl BenchRng {
    #[inline]
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    #[inline]
    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(GAMMA);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    #[inline]
    pub fn gen_below(&mut self, n: u64) -> u64 {
        assert!(n > 0, "gen_below requires n > 0");
        let m = u128::from(self.next_u64()) * u128::from(n);
        (m >> 64) as u64
    }
}

/// In-place Fisher–Yates shuffle
pub fn shuffle<T>(slice: &mut [T], rng: &mut BenchRng) {
    const _: () = assert!(usize::BITS <= 64);

    for i in (1..slice.len()).rev() {
        let j = rng.gen_below((i as u64) + 1) as usize;
        if i != j {
            slice.swap(i, j);
        }
    }
}
