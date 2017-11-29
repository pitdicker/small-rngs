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
pub struct XoroshiroMt32of128Rng {
    s0: u64,
    s1: u64,
}

impl SeedFromRng for XoroshiroMt32of128Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut tuple: (u64, u64);
        loop {
            tuple = (other.next_u64(), other.next_u64());
            if tuple != (0, 0) {
                break;
            }
        }
        let (s0, s1) = tuple;
        Ok(XoroshiroMt32of128Rng{ s0: s0, s1: s1 })
    }
}

impl Rng for XoroshiroMt32of128Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let mult = s0.wrapping_mul(2685821657736338717);

        s1 ^= s0;
        self.s0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.s1 = s1.rotate_left(36); // c
        (mult >> 16) as u32
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



#[derive(Clone)]
pub struct XoroshiroMt64of128Rng {
    s0: u64,
    s1: u64,
}

impl SeedFromRng for XoroshiroMt64of128Rng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut tuple: (u64, u64);
        loop {
            tuple = (other.next_u64(), other.next_u64());
            if tuple != (0, 0) {
                break;
            }
        }
        let (s0, s1) = tuple;
        Ok(XoroshiroMt64of128Rng{ s0: s0, s1: s1 })
    }
}

impl Rng for XoroshiroMt64of128Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let mult = s0.wrapping_mul(2685821657736338717);

        s1 ^= s0;
        self.s0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.s1 = s1.rotate_left(36); // c
        (mult >> 16) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let mult = s0 as u128 * 2685821657736338717 as u128;

        s1 ^= s0;
        self.s0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.s1 = s1.rotate_left(36); // c

        (mult >> 32) as u64
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
