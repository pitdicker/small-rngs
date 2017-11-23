// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A Small Fast Counting RNG, version 4.

use rand_core::{Rng, SeedFromRng, Error, impls};

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

impl SeedFromRng for Sfc32Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut state = Sfc32Rng { a: other.next_u32(),
                                   b: other.next_u32(),
                                   c: other.next_u32(),
                                   counter: 1};
        for _ in 0..15 {
            state.next_u32();
        }
        Ok(state)
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

impl SeedFromRng for Sfc64Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut state = Sfc64Rng { a: other.next_u64(),
                                   b: other.next_u64(),
                                   c: other.next_u64(),
                                   counter: 1};
        for _ in 0..20 {
            state.next_u32();
        }
        Ok(state)
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
