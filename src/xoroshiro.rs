// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Xoroshiro+ random number generators

use rand_core::{Rng, SeedFromRng, Error, impls};

/// The Xoroshiro128+ random number generator.
///
/// - Author: David Blackman and Sebastiano Vigna
/// - License: Public domain
/// - Source: [xoroshiro128plus.c](http://xoroshiro.di.unimi.it/xoroshiro128plus.c)
/// - Period: 2<sup>128</sup> - 1
/// - State: 128 bits
/// - Word size: 64 bits
/// - Seed size: 128 bits. Will panic if seed is all zeros.
#[derive(Clone)]
pub struct Xoroshiro128PlusRng {
    s0: u64,
    s1: u64,
}

impl SeedFromRng for Xoroshiro128PlusRng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut tuple: (u64, u64);
        loop {
            tuple = (other.next_u64(), other.next_u64());
            if tuple != (0, 0) {
                break;
            }
        }
        let (s0, s1) = tuple;
        Ok(Xoroshiro128PlusRng{ s0: s0, s1: s1 })
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

impl SeedFromRng for Xoroshiro64PlusRng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        let mut tuple: (u32, u32);
        loop {
            tuple = (other.next_u32(), other.next_u32());
            if tuple != (0, 0) {
                break;
            }
        }
        let (s0, s1) = tuple;
        Ok(Xoroshiro64PlusRng{ s0: s0, s1: s1 })
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
