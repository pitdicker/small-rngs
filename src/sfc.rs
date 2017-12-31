// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A Small Fast Counting RNG, version 4.

use rand_core::{Rng, SeedableRng, Error, impls, le};
use core::slice;

/// A Small Fast Counting RNG designed by Chris Doty-Humphrey (32-bit version).
///
/// - Author: Chris Doty-Humphrey
/// - License: Public domain
/// - Source: [PractRand](http://pracrand.sourceforge.net/)
/// - Period: avg ~ 2<sup>127</sup>, min >= 2<sup>32</sup>
/// - State: 128 bits
/// - Word size: 32 bits
/// - Seed size: 96 bits
/// - Passes BigCrush and PractRand
#[derive(Clone)]
pub struct Sfc32Rng {
    a: u32,
    b: u32,
    c: u32,
    counter: u32,
}

impl SeedableRng for Sfc32Rng {
    type Seed = [u8; 12];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 3];
        le::read_u32_into(&seed, &mut seed_u32);
        let mut state = Self { a: seed_u32[0],
                               b: seed_u32[1],
                               c: seed_u32[2],
                               counter: 1};
        // Skip the first 15 outputs, just in case we have a bad seed.
        for _ in 0..15 {
            state.next_u32();
        }
        state
    }

    fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
        // Custom `from_rng` function. Because we can assume the seed to be of
        // good quality, it is not neccesary to discard the first couple of
        // rounds.
        let mut seed_u32 = [0u32; 3];
        unsafe {
            let ptr = seed_u32.as_mut_ptr() as *mut u8;

            let slice = slice::from_raw_parts_mut(ptr, 4*3);
            rng.try_fill(slice)?;
        }
        Ok(Self { a: seed_u32[0], b: seed_u32[1], c: seed_u32[2], counter: 1 })
    }
}

impl Rng for Sfc32Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        // good sets include {21,9,3} and {15,8,3}
        const BARREL_SHIFT: u32 = 21;
        const RSHIFT: u32 = 9;
        const LSHIFT: u32 = 3;

        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.counter += 1;
        self.a = self.b ^ (self.b >> RSHIFT);
        self.b = self.c.wrapping_add(self.c << LSHIFT);
        self.c = self.c.rotate_left(BARREL_SHIFT).wrapping_add(tmp);
        tmp
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





/// A Small Fast Counting RNG designed by Chris Doty-Humphrey (64-bit version).
///
/// - Author: Chris Doty-Humphrey
/// - License: Public domain
/// - Source: [PractRand](http://pracrand.sourceforge.net/)
/// - Period: avg ~ 2<sup>255</sup>, min >= 2<sup>64</sup>
/// - State: 256 bits
/// - Word size: 64 bits
/// - Seed size: 192 bits
/// - Passes BigCrush and PractRand
#[derive(Clone)]
pub struct Sfc64Rng {
    a: u64,
    b: u64,
    c: u64,
    counter: u64,
}

impl SeedableRng for Sfc64Rng {
    type Seed = [u8; 24];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 3];
        le::read_u64_into(&seed, &mut seed_u64);
        let mut state = Self { a: seed_u64[0],
                               b: seed_u64[1],
                               c: seed_u64[2],
                               counter: 1};
        // Skip the first 18 outputs, just in case we have a bad seed.
        for _ in 0..18 {
            state.next_u64();
        }
        state
    }

    fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
        // Custom `from_rng` function. Because we can assume the seed to be of
        // good quality, it is not neccesary to discard the first couple of
        // rounds.
        let mut seed_u64 = [0u64; 3];
        unsafe {
            let ptr = seed_u64.as_mut_ptr() as *mut u8;

            let slice = slice::from_raw_parts_mut(ptr, 8*3);
            rng.try_fill(slice)?;
        }
        Ok(Self { a: seed_u64[0], b: seed_u64[1], c: seed_u64[2], counter: 1 })
    }
}

impl Rng for Sfc64Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        // good sets include {24,11,3} and {25,12,3}
        const BARREL_SHIFT: u32 = 24;
        const RSHIFT: u32 = 11;
        const LSHIFT: u32 = 3;

        let tmp = self.a.wrapping_add(self.b).wrapping_add(self.counter);
        self.counter += 1;
        self.a = self.b ^ (self.b >> RSHIFT);
        self.b = self.c.wrapping_add(self.c << LSHIFT);
        self.c = self.c.rotate_left(BARREL_SHIFT).wrapping_add(tmp);
        tmp
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
