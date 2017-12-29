// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Xorshift* random number generators

use rand_core::{Rng, SeedableRng, Error, impls, le};

#[derive(Clone)]
pub struct XorshiftMt32Rng {
    s0: u32,
    s1: u32,
}

impl SeedableRng for XorshiftMt32Rng {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 2];
        le::read_u32_into(&seed, &mut seed_u32);

        if seed_u32.iter().all(|&x| x == 0) {
            seed_u32 = [0xBAD_5EED, 0xBAD_5EED];
        }

        Self { s0: seed_u32[0], s1: seed_u32[1] }
    }
}

impl Rng for XorshiftMt32Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = self.s0;
        let y = self.s1;
        let t = x ^ (x >> 11) ^ (y ^ (y >> 4)); // b, c
        self.s0 = y ^ (y << 13); // a, first xorshift step of the next round
        self.s1 = t;
        ((t as u64 * 741103597) >> 16) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    #[cfg(feature = "i128_support")]
    fn next_u128(&mut self) -> u128 {
        impls::next_u128_via_u64(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        ::rand_core::impls::fill_bytes_via_u32(self, dest)
    }

    fn try_fill(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}



#[derive(Clone)]
pub struct XorshiftMt64Rng {
    s0: u64,
    s1: u64,
}

impl XorshiftMt64Rng {
    #[inline]
    fn xorshift(&mut self) -> u64 {
        let x = self.s0;
        let y = self.s1;
        let t = x ^ (x >> 18) ^ (y ^ (y >> 5)); // b, c
        self.s0 = y ^ (y << 23); // a, first xorshift step of the next round
        self.s1 = t;
        t
    }
}

impl SeedableRng for XorshiftMt64Rng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);

        if seed_u64.iter().all(|&x| x == 0) {
            seed_u64 = [0x0DD_B1A5E5_BAD_5EED, 0x0DD_B1A5E5_BAD_5EED];
        }

        Self { s0: seed_u64[0], s1: seed_u64[1] }
    }
}

impl Rng for XorshiftMt64Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.xorshift().wrapping_mul(2685821657736338717) >> 16) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        ((self.xorshift() as u128 * 2685821657736338717 as u128) >> 32) as u64
    }

    #[cfg(feature = "i128_support")]
    fn next_u128(&mut self) -> u128 {
        impls::next_u128_via_u64(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        ::rand_core::impls::fill_bytes_via_u64(self, dest)
    }

    fn try_fill(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
