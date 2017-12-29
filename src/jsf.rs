// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Bob Jenkins small fast pseudorandom number generator.

use rand_core::{Rng, SeedableRng, Error, impls, le};

/// A small random number generator designed by Bob Jenkins.
///
/// - Author: Bob Jenkins
/// - License: Public domain
/// - Source: http://burtleburtle.net/bob/rand/smallprng.html
/// - Period: 2<sup>64</sup>
/// - State: 128 bits
/// - Word size: 32 bits
/// - Seed size: 32 bits (very small!)
/// - Passes BigCrush and PractRand
#[derive(Clone)]
pub struct Jsf32Rng {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl SeedableRng for Jsf32Rng {
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 1];
        le::read_u32_into(&seed, &mut seed_u32);

        let mut state = Self { a: 0xf1ea5eed, // fleaseed
                               b: seed_u32[0],
                               c: seed_u32[0],
                               d: seed_u32[0]};
        for _ in 0..20 {
            state.next_u32();
        }
        state
    }
}

impl Rng for Jsf32Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let e = self.a.wrapping_sub(self.b.rotate_left(27));
        self.a = self.b ^ self.c.rotate_left(17);
        self.b = self.c.wrapping_add(self.d);
        self.c = self.d.wrapping_add(e);
        self.d = e.wrapping_add(self.a);
        self.d
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




/// A small random number generator designed by Bob Jenkins (64-bit variant).
///
/// - Author: Bob Jenkins
/// - License: Public domain
/// - Source: http://burtleburtle.net/bob/rand/smallprng.html
/// - Period: 2<sup>64</sup>
/// - State: 256 bits
/// - Word size: 64 bits
/// - Seed size: 64 bits (very small!)
/// - Passes BigCrush and PractRand
#[derive(Clone)]
pub struct Jsf64Rng {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl SeedableRng for Jsf64Rng {
    type Seed = [u8; 8];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 1];
        le::read_u64_into(&seed, &mut seed_u64);

        let mut state = Self { a: 0xf1ea5eed, // fleaseed
                               b: seed_u64[0],
                               c: seed_u64[0],
                               d: seed_u64[0]};
        for _ in 0..20 {
            state.next_u64();
        }
        state
    }
}

impl Rng for Jsf64Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let e = self.a.wrapping_sub(self.b.rotate_left(7));
        self.a = self.b ^ self.c.rotate_left(31);
        self.b = self.c.wrapping_add(self.d.rotate_right(27));
        self.c = self.d.wrapping_add(e);
        self.d = e.wrapping_add(self.a);
        self.d
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
