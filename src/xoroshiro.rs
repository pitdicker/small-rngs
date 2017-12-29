// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Xoroshiro+ random number generators

use rand_core::{Rng, SeedableRng, Error, impls, le};

/// The Xoroshiro128+ random number generator.
///
/// - Author: David Blackman and Sebastiano Vigna
/// - License: Public domain
/// - Source: [xoroshiro128plus.c](http://xoroshiro.di.unimi.it/xoroshiro128plus.c)
/// - Period: 2<sup>128</sup> - 1
/// - State: 128 bits
/// - Word size: 64 bits
/// - Seed size: 128 bits
#[derive(Clone)]
pub struct Xoroshiro128PlusRng {
    s0: u64,
    s1: u64,
}

impl SeedableRng for Xoroshiro128PlusRng {
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

impl Rng for Xoroshiro128PlusRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.s0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.s1 = s1.rotate_left(36); // c

        result
    }

    #[cfg(feature = "i128_support")]
    fn next_u128(&mut self) -> u128 {
        impls::next_u128_via_u64(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_u64(self, dest)
    }

    fn try_fill(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}


/// A 32-bit variant of Xoroshiro128+, with just 64 bits of state.
#[derive(Clone)]
pub struct Xoroshiro64PlusRng {
    s0: u32,
    s1: u32,
}

impl SeedableRng for Xoroshiro64PlusRng {
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

impl Rng for Xoroshiro64PlusRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let result = s0.wrapping_add(s1);

        s1 ^= s0;
        self.s0 = s0.rotate_left(19) ^ s1 ^ (s1 << 13); // a, b
        self.s1 = s1.rotate_left(10); // c

        result
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
        impls::fill_bytes_via_u32(self, dest)
    }

    fn try_fill(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
