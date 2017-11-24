// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Plain Xorshift rondom number generators

use rand_core::{Rng, SeedFromRng, Error, impls};
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
/// - Seed size: 128 bits. Will panic if seed is all zeros.
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

impl SeedFromRng for Xorshift128_32Rng {
    fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
        let mut tuple: (u32, u32, u32, u32);
        loop {
            tuple = (rng.next_u32(), rng.next_u32(), rng.next_u32(), rng.next_u32());
            if tuple != (0, 0, 0, 0) {
                break;
            }
        }
        let (x, y, z, w) = tuple;
        Ok(Xorshift128_32Rng { x: x, y: y, z: z, w: w })
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
/// - Seed size: 128 bits. Will panic if seed is all zeros.
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

impl SeedFromRng for Xorshift128_64Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut tuple: (u64, u64);
        loop {
            tuple = (other.next_u64(), other.next_u64());
            if tuple != (0, 0) {
                break;
            }
        }
        let (s0, s1) = tuple;
        Ok(Xorshift128_64Rng{ s0: s0, s1: s1 })
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
