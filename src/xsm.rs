// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The XSM random number generator.

use rand_core::{Rng, SeedFromRng, Error, impls};

/// XSM (32-bit version).
///
/// - Author: Chris Doty-Humphrey
/// - License: Public domain
/// - Source: [PractRand](http://pracrand.sourceforge.net/)
/// - Period: 2^64
/// - State: 95 bits
/// - Word size: 32 bits
/// - Seed size: 96 bits
/// - Passes BigCrush and PractRand
#[derive(Clone)]
pub struct Xsm32Rng {
    lcg_low: u32,
    lcg_high: u32,
    lcg_adder: u32,
    history: u32,
}

impl SeedFromRng for Xsm32Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut state = Xsm32Rng { lcg_low: other.next_u32(),
                                   lcg_high: other.next_u32(),
                                   lcg_adder: other.next_u32() | 1,
                                   history: 0};
        state.next_u32();
        Ok(state)
    }
}

impl Rng for Xsm32Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        const K: u32 = 0x6595a395;

        let mut rv = self.history.wrapping_mul(K);
        let mut tmp = self.lcg_high
                  .wrapping_add((self.lcg_high ^ self.lcg_low).rotate_left(11));
        tmp = tmp.wrapping_mul(K);
        let mut old_lcg_low = self.lcg_low;
        self.lcg_low = self.lcg_low.wrapping_add(self.lcg_adder);
        old_lcg_low = old_lcg_low.wrapping_add((self.lcg_low < self.lcg_adder) as u32);
        self.lcg_high = self.lcg_high.wrapping_add(old_lcg_low);

        rv ^= rv >> 16;
        self.history = tmp ^ (tmp >> 16);
        rv = rv.wrapping_add(self.history);
        rv
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



/// XSM (64-bit version).
///
/// - Author: Chris Doty-Humphrey
/// - License: Public domain
/// - Source: [PractRand](http://pracrand.sourceforge.net/)
/// - Period: 2^128
/// - State: 191 bits
/// - Word size: 64 bits
/// - Seed size: 192 bits
/// - Passes BigCrush and PractRand
#[derive(Clone)]
pub struct Xsm64Rng {
    lcg_low: u64,
    lcg_high: u64,
    lcg_adder: u64,
    history: u64,
}

impl SeedFromRng for Xsm64Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut state = Xsm64Rng { lcg_low: other.next_u64(),
                                   lcg_high: other.next_u64(),
                                   lcg_adder: other.next_u64() | 1,
                                   history: 0};
        state.next_u32();
        Ok(state)
    }
}

impl Rng for Xsm64Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        const K: u64 = 0xa3ec647659359acd;

        self.history = self.history.wrapping_mul(K);
        let mut tmp = self.lcg_high
                  .wrapping_add((self.lcg_high ^ self.lcg_low).rotate_left(19));
        tmp = tmp.wrapping_mul(K);

        let mut old = self.lcg_low;
        self.lcg_high = self.lcg_high.wrapping_add(old.wrapping_add((self.lcg_low < self.lcg_adder) as u64));

        old = self.history ^ (self.history >> 32);
        self.history = tmp ^ (tmp >> 32);
        tmp.wrapping_add(old)
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
