// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Xorshift* random number generators

use rand_core::{Rng, SeedFromRng, Error, impls};

#[derive(Clone)]
pub struct XorshiftMultWT32Rng {
    s0: u32,
    s1: u32,
}

impl SeedFromRng for XorshiftMultWT32Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut tuple: (u32, u32);
        loop {
            tuple = (other.next_u32(), other.next_u32());
            if tuple != (0, 0) {
                break;
            }
        }
        let (s0, s1) = tuple;
        Ok(XorshiftMultWT32Rng{ s0: s0, s1: s1 })
    }
}

impl Rng for XorshiftMultWT32Rng {
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
pub struct XorshiftMultWT64Rng {
    s0: u64,
    s1: u64,
}

impl XorshiftMultWT64Rng {
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

impl SeedFromRng for XorshiftMultWT64Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut tuple: (u64, u64);
        loop {
            tuple = (other.next_u64(), other.next_u64());
            if tuple != (0, 0) {
                break;
            }
        }
        let (s0, s1) = tuple;
        Ok(XorshiftMultWT64Rng{ s0: s0, s1: s1 })
    }
}

impl Rng for XorshiftMultWT64Rng {
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










