// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Geronimo Jones' random number generator.

use rand_core::{Rng, SeedFromRng, Error, impls};

/// A small random number generator by Geronimo Jones.
///
/// - Author: Geronimo Jones
/// - Source: Part of [`gjrand`](http://gjrand.sourceforge.net/boast.html).
/// - License: GPL v2 or v3
/// - Period: 2<sup>64</sup>
/// - State: 256 bits
/// - Word size: 64 bits
/// - Passes BigCrush and PractRand
/// - Based "on emperical methods with just a tiny amount of theory as a guide",
///   instead of a sound theoretical basis.
#[derive(Clone)]
pub struct GjRng {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
}

impl SeedFromRng for GjRng {
    fn from_rng<R: Rng>(mut other: R) -> Result<Self, Error> {
        Ok(GjRng{ a: other.next_u64(),
                      b: other.next_u64(),
                      c: other.next_u64(),
                      d: other.next_u64()})
    }
}

impl Rng for GjRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let mut a = self.a;
        let mut b = self.b;
        let mut c = self.c;
        let mut d = self.d;

        // Crank
        b = b.wrapping_add(c);
        a = a.rotate_left(32);
        c ^= b;

        d = d.wrapping_add(0x55aa96a5);

        a = a.wrapping_add(b);
        c = c.rotate_left(23);
        b ^= a;

        a = a.wrapping_add(c);
        b = b.rotate_left(19);
        c += a;

        b += d;

        self.a = a;
        self.b = b;
        self.c = c;
        self.d = d;

        a
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

