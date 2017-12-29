// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Plain Xorshift rondom number generators

use rand_core::{Rng, SeedableRng, Error, impls, le};
use core::fmt;

/// An Xorshift random number generator (128/32-bit variant).
///
/// - Author: George Marsaglia
/// - License: Public domain
/// - Source: ["Xorshift RNGs"](http://www.jstatsoft.org/v08/i14/paper).
///           *Journal of Statistical Software*. Vol. 8 (Issue 14).
/// - Period: 2<sup>128</sup> - 1
/// - State: 128 bits
/// - Word size: 32 bits
/// - Seed size: 128 bits
/// - Low quality
/// - The small RNG currently available in rand (0.3.18)
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Xorshift128_32Rng {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl fmt::Debug for Xorshift128_32Rng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Xorshift128_32Rng {{}}")
    }
}

impl SeedableRng for Xorshift128_32Rng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 4];
        le::read_u32_into(&seed, &mut seed_u32);

        if seed_u32.iter().all(|&x| x == 0) {
            seed_u32 = [0xBAD_5EED, 0xBAD_5EED, 0xBAD_5EED, 0xBAD_5EED];
        }

        Self {
            x: seed_u32[0],
            y: seed_u32[1],
            z: seed_u32[2],
            w: seed_u32[3],
        }
    }
}

impl Rng for Xorshift128_32Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
/*
        // optimized version
        let x = self.x;
        // already do the first xorshift step of the next round
        self.x = self.y ^ (self.y << 11);
        self.y = self.z;
        self.z = self.z;
        let t = self.w;
        self.w = (x ^ (x >> 19)) ^ t ^ (t >> 8);
        self.w
*/
        let x = self.x;
        let t = x ^ (x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        let w = self.w;
        self.w = w ^ (w >> 19) ^ (t ^ (t >> 8));
        self.w
    }
    
    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }
    #[cfg(feature = "i128_support")]
    fn next_u128(&mut self) -> u128 {
        impls::next_u128_via_u64(self)
    }
    
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_u32(self, dest);
    }

    fn try_fill(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

/// An Xorshift random number generator (128/64-bit variant).
///
/// - Author: George Marsaglia
/// - License: Public domain
/// - Source: ["Xorshift RNGs"](http://www.jstatsoft.org/v08/i14/paper).
///           *Journal of Statistical Software*. Vol. 8 (Issue 14).
/// - Period: 2<sup>128</sup> - 1
/// - State: 128 bits
/// - Word size: 64 bits
/// - Seed size: 128 bits
/// - Low quality, very fast
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Xorshift128_64Rng {
    s0: u64,
    s1: u64,
}

impl fmt::Debug for Xorshift128_64Rng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Xorshift128_64Rng {{}}")
    }
}

impl SeedableRng for Xorshift128_64Rng {
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

impl Rng for Xorshift128_64Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let x = self.s0;
        let y = self.s1;
        let t = x ^ (x >> 18) ^ (y ^ (y >> 5));
        self.s0 = y ^ (y << 23); // first xorshift step of the next round
        self.s1 = t;
        t
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
