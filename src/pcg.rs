// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! PCG random number generators

use rand_core::{Rng, SeedFromRng, Error, impls};

/// A PCG random number generator (XSH 64/32 (LCG) variant).
///
/// Permuted Congruential Generators, "xorshift high (bits), random rotation"
/// using an underlying Linear congruential generator
#[derive(Clone)]
pub struct PcgXsh64LcgRng {
    state: u64,
    increment: u64,
}

impl SeedFromRng for PcgXsh64LcgRng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        // We only have to make sure increment is odd.
        let mut ctx = PcgXsh64LcgRng { state: other.next_u64(),
                                       increment: other.next_u64() | 1 };
        // Prepare for the first round
        ctx.state = ctx.state.wrapping_mul(6364136223846793005)
                             .wrapping_add(ctx.increment);
        Ok(ctx)
    }
}

impl Rng for PcgXsh64LcgRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        // prepare the LCG for the next round
        self.state = state.wrapping_mul(6364136223846793005);

        // output function XSH RR: xorshift high (bits), followed by a random rotate
        // good for 64-bit state, 32-bit output
        const IN_BITS: u32 = 64;
        const OUT_BITS: u32 = 32;
        const OP_BITS: u32 = 5; // log2(OUT_BITS)

        const ROTATE: u32 = IN_BITS - OP_BITS; // 59
        const XSHIFT: u32 = (OUT_BITS + OP_BITS) / 2; // 18
        const SPARE: u32 = IN_BITS - OUT_BITS - OP_BITS; // 27

        let xsh = (((state >> XSHIFT) ^ state) >> SPARE) as u32;

        self.state = state.wrapping_add(self.increment);
        xsh.rotate_right((state >> ROTATE) as u32)
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



/// A PCG random number generator (XSL 64/32 (LCG) variant).
///
/// Permuted Congruential Generators, "xorshift low (bits), random rotation"
/// using an underlying Linear congruential generator
#[derive(Clone)]
pub struct PcgXsl64LcgRng {
    state: u64,
    increment: u64,
}

impl SeedFromRng for PcgXsl64LcgRng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        // We only have to make sure increment is odd.
        let mut ctx = PcgXsl64LcgRng { state: other.next_u64(),
                                       increment: other.next_u64() | 1 };
        // Prepare for the first round
        ctx.state = ctx.state.wrapping_mul(6364136223846793005)
                             .wrapping_add(ctx.increment);
        Ok(ctx)
    }
}

impl Rng for PcgXsl64LcgRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let state = self.state;
        // prepare the LCG for the next round
        self.state = state.wrapping_mul(6364136223846793005);

        // Output function XSL RR ("xorshift low (bits), random rotation"):
        const IN_BITS: u32 = 64;
        const OUT_BITS: u32 = 32;
        const SPARE_BITS: u32 = IN_BITS - OUT_BITS;
        const OP_BITS: u32 = 5; // log2(OUT_BITS)

        const XSHIFT: u32 = (SPARE_BITS + OUT_BITS) / 2; // 32
        const ROTATE: u32 = IN_BITS - OP_BITS; // 59

        let xsl = ((state >> XSHIFT) as u32) ^ (state as u32);

        self.state = state.wrapping_add(self.increment);
        xsl.rotate_right((state >> ROTATE) as u32)
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



/// A PCG random number generator (XSL 128/64 (LCG) variant).
///
/// Permuted Congruential Generators, "xorshift low (bits), random rotation"
/// using an underlying multiplicative congruential generator
#[derive(Clone)]
pub struct PcgXsl128McgRng {
    state: u128,
}

const MULTIPLIER: u128 = 2549297995355413924u128 << 64 | 4865540595714422341;

impl SeedFromRng for PcgXsl128McgRng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        // We only have to make sure increment is odd.
        let mut ctx = PcgXsl128McgRng { state: (other.next_u64() as u128) << 64 |
                                               (other.next_u64() as u128) };
        // Prepare for the first round
        ctx.state = ctx.state.wrapping_mul(MULTIPLIER);
        Ok(ctx)
    }
}

impl Rng for PcgXsl128McgRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let state = self.state;
        // prepare for the next round
        self.state = state.wrapping_mul(MULTIPLIER);

        // Output function XSL RR ("xorshift low (bits), random rotation"):
        // XSL uses xor folding of the high and the low u64. This minimizes the
        // amount of information about internal state that leaks out.
        const IN_BITS: u32 = 128;
        const OUT_BITS: u32 = 64;
        const SPARE_BITS: u32 = IN_BITS - OUT_BITS;
        const OP_BITS: u32 = 6; // log2(OUT_BITS)

        const XSHIFT: u32 = (SPARE_BITS + OUT_BITS) / 2; // 64
        const ROTATE: u32 = IN_BITS - OP_BITS; // 122

        let xsl = ((state >> XSHIFT) as u64) ^ (state as u64);
        xsl.rotate_right((state >> ROTATE) as u32)
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
