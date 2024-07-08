use std::sync::atomic::{AtomicU64, Ordering};

pub struct Random {
    seed: AtomicU64,
}

impl Random {
    const MULTIPLIER: u64 = 0x5DEECE66D;
    const ADDEND: u64 = 0xB;
    const MASK: u64 = (1 << 48) - 1;

    pub fn new(seed: u64) -> Self {
        Random {
            seed: AtomicU64::new(Self::initial_scramble(seed)),
        }
    }

    fn initial_scramble(seed: u64) -> u64 {
        (seed ^ Self::MULTIPLIER) & Self::MASK
    }

    fn next(&self, bits: u32) -> u32 {
        let old_seed = self.seed.load(Ordering::Relaxed);
        let next_seed = (old_seed
            .wrapping_mul(Self::MULTIPLIER)
            .wrapping_add(Self::ADDEND))
            & Self::MASK;
        self.seed.store(next_seed, Ordering::Relaxed);
        (next_seed >> (48 - bits)) as u32
    }

    #[allow(dead_code)]
    pub fn next_int(&self) -> i32 {
        self.next(32) as i32
    }

    pub fn next_int_bound(&self, bound: u32) -> i32 {
        if bound <= 0 {
            panic!("bound must be positive");
        }

        let mut bits;
        let mut val;
        loop {
            bits = self.next(31);
            val = bits % bound;
            if bits - val + (bound - 1) >= bound {
                break;
            }
        }
        val as i32
    }
}
