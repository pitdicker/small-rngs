// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! CIPRNG v3

use rand_core::{Rng, SeedableRng, Error, impls, le};
use core::fmt;

/// Chaotic Iterations PRNG
///
/// - Author: Jacques M. Bahi et al
/// - License: Public domain (?)
/// - Source: ["FPGA acceleration of a pseudorandom number generator based on
///            chaotic iterations"](https://ai2-s2-pdfs.s3.amazonaws.com/7582/3db79773d60451b758bbdb288566ec7f6cbe.pdf).
/// - Period: N/A / 2^64?
/// - State: 192 bits
/// - Word size: 64 bits
/// - Seed size: 192 bits
#[derive(Clone)]
pub struct CiRng {
    t1: u64,
    t2: u64,
    t4: u32,
    x: u32,
}

impl fmt::Debug for CiRng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CiRng {{}}")
    }
}

impl SeedableRng for CiRng {
    type Seed = [u8; 24];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 3];
        le::read_u64_into(&seed, &mut seed_u64);
        let mut seed_u32 = [0u32; 6];
        le::read_u32_into(&seed, &mut seed_u32);

        CiRng {
            t1: if seed_u64[0] != 0 { seed_u64[0] } else { 0x0DD_B1A5E5_BAD_5EED },
            t2: if seed_u64[1] != 0 { seed_u64[1] } else { 0x0DD_B1A5E5_BAD_5EED },
            t4: if seed_u32[4] != 0 { seed_u32[4] } else { 0xBAD_5EED },
            x: seed_u32[5],
        }
    }

    fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
        let mut t1 = 0;
        while t1 == 0 { t1 = rng.next_u64() }
        let mut t2 = 0;
        while t2 == 0 { t2 = rng.next_u64() }
        let mut t4 = 0;
        while t4 == 0 { t4 = rng.next_u32() }
        let x = rng.next_u32();

        Ok(CiRng { t1: t1, t2:t2, t4: t4, x: x })
    }
}

impl Rng for CiRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        // Xorshift 1
        self.t1 ^= self.t1 << 11;
        self.t1 ^= self.t1 >> 9;
        self.t1 ^= self.t1 << 34;

        // Xorshift 2
        self.t2 ^= self.t2 << 12;
        self.t2 ^= self.t2 >> 25;
        self.t2 ^= self.t2  << 27;

        // Blum Blum Shub
        // Note: the primes for the modulus are probably horribly chosen.
        self.t4 = ((self.t4 as u64) * (self.t4 as u64) % (80021*53653)) as u32;

        // Xorshift instead of Blum Blum Shub
//        self.t4 ^= self.t4 << 13;
//        self.t4 ^= self.t4 >> 17;
//        self.t4 ^= self.t4  << 5;


        // Use binary masking to imitate ifs (no branches is 3x faster)
        // if self.t4 & 1 != 0 { self.x ^= self.t1 as u32; }
        // if self.t4 & 2 != 0 { self.x ^= (self.t1 >> 32) as u32; }
        // if self.t4 & 4 != 0 { self.x ^= self.t2 as u32; }
        let apply = !((self.t4 >> 0) as u32 & 1).wrapping_sub(1);
        self.x ^= self.t1 as u32 & apply;
        let apply = !((self.t4 >> 1) as u32 & 1).wrapping_sub(1);
        self.x ^= (self.t1 >> 32) as u32 & apply;
        let apply = !((self.t4 >> 2) as u32 & 1).wrapping_sub(1);
        self.x ^= self.t2 as u32 & apply;

        self.x ^= (self.t2 >> 32) as u32;

        self.x
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
