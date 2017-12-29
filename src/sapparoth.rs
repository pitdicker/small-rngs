// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A fast pseudorandom number generator by Ilya Levin.

use rand_core::{Rng, SeedableRng, Error, impls, le};

/// The Sapparot-2 random number generator by Ilya Levin (32-bit version).
///
/// - Author: Ilya Levin
/// - License: unknown
/// - Source: ["Sapparot-2"](http://www.literatecode.com/get/sapparot2.pdf)
/// - Period: ?
/// - State: 96 bits
/// - Word size: 32 bits
//  - Seed size: 96 bits
#[derive(Clone)]
pub struct Sapparot32Rng {
    a: u32,
    b: u32,
    c: u32,
}

impl SeedableRng for Sapparot32Rng {
    type Seed = [u8; 12];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 3];
        le::read_u32_into(&seed, &mut seed_u32);
        Self { a: seed_u32[0],
               b: seed_u32[1],
               c: seed_u32[2] }
    }
}

impl Rng for Sapparot32Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        const PHI: u32 = 0x9e3779b9;

        self.c = self.c.wrapping_add(self.a);
        self.c = self.c.rotate_left(self.b >> 27);
        self.b = self.b.wrapping_add((self.a << 1).wrapping_add(1))
                 ^ self.b.rotate_left(5);
        self.a = self.a.wrapping_add(PHI).rotate_left(7);
        let m = self.a;
        self.a = self.b;
        self.b = m;

        self.c ^ self.b ^ self.a
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



/// The Sapparot-2 random number generator by Ilya Levin (64-bit version).
///
/// - Author: Ilya Levin
/// - License: unknown
/// - Source: ["Sapparot-2"](http://www.literatecode.com/get/sapparot2.pdf)
/// - Period: ?
/// - State: 192 bits
/// - Word size: 64 bits
//  - Seed size: 192 bits
#[derive(Clone)]
pub struct Sapparot64Rng {
    a: u64,
    b: u64,
    c: u64,
}

impl SeedableRng for Sapparot64Rng {
    type Seed = [u8; 24];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 3];
        le::read_u64_into(&seed, &mut seed_u64);
        Self { a: seed_u64[0],
               b: seed_u64[1],
               c: seed_u64[2] }
    }
}

impl Rng for Sapparot64Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        const PHI: u64 = 0x9e3779b97f4a7c55;

        self.c = self.c.wrapping_add(self.a);
        self.c = self.c.rotate_left((self.b >> 58) as u32);
        self.b = self.b.wrapping_add((self.a << 1).wrapping_add(1))
                 ^ self.b.rotate_left(5);
        self.a = self.a.wrapping_add(PHI).rotate_left(13);
        let m = self.a;
        self.a = self.b;
        self.b = m;

        self.c ^ self.b ^ self.a
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
