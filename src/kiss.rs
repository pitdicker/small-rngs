// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! KISS rondom number generators


use rand_core::{Rng, SeedableRng, Error, impls, le};
use core::fmt;
use core::num::Wrapping as Wr;

/// The KISS random number generator (32-bit variant).
///
/// - Author: George Marsaglia
/// - License: Public domain
/// - Source: ["Random numbers in C: Some suggestions"]
///           (http://www.ciphersbyritter.com/NEWS4/RANDC.HTM).
/// - Period: ~2<sup>123</sup>
/// - State: 128 bits
/// - Word size: 32 bits
/// - Seed size: 128 bits
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Kiss32Rng {
    z: Wr<u32>,
    w: Wr<u32>,
    jsr: Wr<u32>,
    jcong: Wr<u32>,
}

impl fmt::Debug for Kiss32Rng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kiss32Rng {{}}")
    }
}

impl SeedableRng for Kiss32Rng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 4];
        le::read_u32_into(&seed, &mut seed_u32);

        Self {
            z: Wr(seed_u32[0]),
            w: Wr(seed_u32[1]),
            jsr: Wr(if seed_u32[2] != 0 { seed_u32[2] } else { 0xBAD_5EED }),
            jcong: Wr(seed_u32[3]),
        }
    }

    fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
        let z = rng.next_u32();
        let w = rng.next_u32();
        let mut jsr = 0;
        while jsr == 0 { jsr = rng.next_u32() };
        let jcong = rng.next_u32();

        Ok(Kiss32Rng { z: Wr(z), w: Wr(w), jsr: Wr(jsr), jcong: Wr(jcong) })
    }
}

impl Rng for Kiss32Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        // Multiply-With-Carry (MWC)
        self.z = Wr(36969) * (self.z & Wr(65535)) + (self.z >> 16);
        self.w = Wr(18000) * (self.w & Wr(65535)) + (self.w >> 16);
//        let mwc = (self.z << 16) + self.w;

        // Congruential (CONG)
        self.jcong = Wr(69069) * self.jcong + Wr(1234567);

        // Xorshift (SH3)
        self.jsr ^= self.jsr << 13;
        self.jsr ^= self.jsr >> 17;
        self.jsr ^= self.jsr << 5;

        let mwc = (self.z << 16) + self.w;
        (mwc + self.jsr + self.jcong).0
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


/// The KISS random number generator (64-bit variant).
///
/// - Author: George Marsaglia
/// - License: Public domain
/// - Source: ["64-bit KISS RNGs"]
///           (https://www.thecodingforums.com/threads/64-bit-kiss-rngs.673657/).
/// - Period: ~2<sup>247</sup>
/// - State: 256 bits
/// - Word size: 64 bits
/// - Seed size: 256 bits
#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Kiss64Rng {
    c: Wr<u64>,
    x: Wr<u64>,
    y: Wr<u64>,
    z: Wr<u64>,
}

impl fmt::Debug for Kiss64Rng {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Kiss64Rng {{}}")
    }
}

impl SeedableRng for Kiss64Rng {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 4];
        le::read_u64_into(&seed, &mut seed_u64);

        Self {
            c: Wr(seed_u64[0]),
            x: Wr(seed_u64[1]),
            y: Wr(if seed_u64[2] != 0 { seed_u64[2] }
                  else { 0x0DD_B1A5E5_BAD_5EED }),
            z: Wr(seed_u64[3]),
        }
    }

    fn from_rng<R: Rng>(mut rng: R) -> Result<Self, Error> {
        let c = rng.next_u64();
        let x = rng.next_u64();
        let mut y = 0;
        while y == 0 { y = rng.next_u64() };
        let z = rng.next_u64();

        Ok(Kiss64Rng { c: Wr(c), x: Wr(x), y: Wr(y), z: Wr(z) })
    }
}

impl Rng for Kiss64Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        // Multiply-With-Carry (MWC)
        let t = (self.x << 58) + self.c;
        self.c = self.x >> 6;
        self.x += t;
        self.c += Wr((self.x < t) as u64);

        // Xorshift (SH3)
        self.y ^= self.y << 13;
        self.y ^= self.y >> 17;
        self.y ^= self.y << 43;

        // Congruential (CONG)
        self.z = Wr(6906969069) * self.z + Wr(1234567);

        (self.x + self.y + self.z).0
    }

    #[cfg(feature = "i128_support")]
    fn next_u128(&mut self) -> u128 {
        impls::next_u128_via_u64(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_u64(self, dest);
    }

    fn try_fill(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}
