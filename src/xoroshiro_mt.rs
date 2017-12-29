// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Xorshift* random number generators

use rand_core::{Rng, SeedableRng, Error, impls, le};

#[derive(Clone)]
pub struct XoroshiroMt32of128Rng {
    s0: u64,
    s1: u64,
}

impl SeedableRng for XoroshiroMt32of128Rng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);

        if seed_u64.iter().all(|&x| x == 0) {
            seed_u64 = [0x0DD_B1A5E5_BAD_5EED, 0x0DD_B1A5E5_BAD_5EED];
        }

        Self { s0: seed_u64[0], s1: seed_u64[1] }
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

impl SeedableRng for XoroshiroMt64of128Rng {
    type Seed = [u8; 16];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u64 = [0u64; 2];
        le::read_u64_into(&seed, &mut seed_u64);

        if seed_u64.iter().all(|&x| x == 0) {
            seed_u64 = [0x0DD_B1A5E5_BAD_5EED, 0x0DD_B1A5E5_BAD_5EED];
        }

        Self { s0: seed_u64[0], s1: seed_u64[1] }
    }
}

impl Rng for XoroshiroMt64of128Rng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let mult = (s0 as u32 as u64).wrapping_mul(3857418925 as u64);

        s1 ^= s0;
        self.s0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.s1 = s1.rotate_left(36); // c
        (mult >> 16) as u32
    }

    #[inline]
    #[cfg(feature = "i128_support")]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let mult = s0 as u128 * 2685821657736338717 as u128;

        s1 ^= s0;
        self.s0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.s1 = s1.rotate_left(36); // c

        (mult >> 32) as u64
    }

    #[inline]
    #[cfg(not(feature = "i128_support"))]
    fn next_u64(&mut self) -> u64 {
        let s0 = self.s0;
        let mut s1 = self.s1;
        let (high, low) = s0.wmul(2685821657736338717);

        s1 ^= s0;
        self.s0 = s0.rotate_left(55) ^ s1 ^ (s1 << 14); // a, b
        self.s1 = s1.rotate_left(36); // c

        high << 32 | low >> 32
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



trait WideningMultiply<RHS = Self> {
    type Output;

    fn wmul(self, x: RHS) -> Self::Output;
}

macro_rules! wmul_impl {
    ($ty:ty, $wide:ident, $shift:expr) => {
        impl WideningMultiply for $ty {
            type Output = ($ty, $ty);

            #[inline(always)]
            fn wmul(self, x: $ty) -> Self::Output {
                let tmp = (self as $wide) * (x as $wide);
                ((tmp >> $shift) as $ty, tmp as $ty)
            }
        }
    }
}

wmul_impl! { u32, u64, 32 }
#[cfg(feature = "i128_support")]
wmul_impl! { u64, u128, 64 }

#[cfg(any(target_pointer_width = "32", not(feature = "i128_support")))]
impl WideningMultiply for u64 {
    type Output = (u64, u64);

    // This code is a translation of the __mulddi3 function in LLVM's
    // compiler-rt. It is an optimised variant of the common method
    // `(a + b) * (c + d) = ac + ad + bc + bd`.
    //
    // For some reason LLVM can optimise the C version very well, but keeps
    // shuffeling registers in this Rust translation.
    #[inline(always)]
    fn wmul(self, b: u64) -> Self::Output {
        const LOWER_MASK: u64 = !0u64 >> 32;
        let mut low = (self & LOWER_MASK).wrapping_mul(b & LOWER_MASK);
        let mut t = low >> 32;
        low &= LOWER_MASK;
        t += (self >> 32).wrapping_mul(b & LOWER_MASK);
        low += (t & LOWER_MASK) << 32;
        let mut high = (t >> 32) as i64;
        t = low >> 32;
        low &= LOWER_MASK;
        t += (b >> 32).wrapping_mul(self & LOWER_MASK);
        low += (t & LOWER_MASK) << 32;
        high += (t >> 32) as i64;
        high += (self >> 32).wrapping_mul(b >> 32) as i64;

        (high as u64, low)
    }
}
