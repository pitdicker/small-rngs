// Copyright 2017 Paul Dicker.
// See the COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The Velox 3b random number generator.

use rand_core::{Rng, SeedableRng, Error, impls, le};

/// A small random number generator designed by Elias Yarrkov.
///
/// - Author: Elias Yarrkov
/// - License: Public domain
/// - Source: http://cipherdev.org/v3b.c
/// - Period: at least 2<sup>128</sup>
/// - State: 256 bit
/// - Word size: 32-bit
//  - Seed size: 32 bit (may be improved to 128 bits)
#[derive(Clone)]
pub struct Velox3bRng {
    v: [u32; 4],
    ctr: [u32; 4],
    pos: usize,
}

impl Velox3bRng {
    fn update(&mut self) {
        self.v[0] = (self.v[0].wrapping_add(self.v[3])).rotate_left(21);
        self.v[1] = self.v[1].rotate_left(12).wrapping_add(self.v[2]);
        self.v[2] = self.v[2] ^ self.v[0];
        self.v[3] = self.v[3] ^ self.v[1];

        self.v[0] = (self.v[0].wrapping_add(self.v[3])).rotate_left(19);
        self.v[1] = self.v[1].rotate_left(24).wrapping_add(self.v[2]);
        self.v[2] = self.v[2] ^ self.v[0];
        self.v[3] = self.v[3] ^ self.v[1];

        self.v[0] = (self.v[0].wrapping_add(self.v[3])).rotate_left(7);
        self.v[1] = self.v[1].rotate_left(12).wrapping_add(self.v[2]);
        self.v[2] = self.v[2] ^ self.v[0];
        self.v[3] = self.v[3] ^ self.v[1];

        self.v[0] = (self.v[0].wrapping_add(self.v[3])).rotate_left(27);
        self.v[1] = self.v[1].rotate_left(17).wrapping_add(self.v[2]);
        self.v[2] = self.v[2] ^ self.v[0];
        self.v[3] = self.v[3] ^ self.v[1];

        for i in 0..4 {
            self.v[i] += self.ctr[i];
        }

        // increase counter by 1
        for i in 0..4 {
            if { self.ctr[i] = self.ctr[i].wrapping_add(1); self.ctr[i] != 0 } {
                break
            };
        }

        self.pos = 4;
    }
}

impl SeedableRng for Velox3bRng {
    type Seed = [u8; 4];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut seed_u32 = [0u32; 1];
        le::read_u32_into(&seed, &mut seed_u32);

        let mut state = Velox3bRng {
            v: [seed_u32[0], 0x3c6ef372, 0xdaa66d2b, 0x78dde6e4],
            ctr: [0x9e3779b9, 0x3c6ef372, 0xdaa66d2b, 0x78dde6e4],
            // 1*0x9e3779b9, 2*0x9e3779b9, 3*0x9e3779b9, 4*0x9e3779b9
            pos: 0};

        for _ in 0..16 {
            state.next_u32();
        }
        state
    }
}

impl Rng for Velox3bRng {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        if self.pos == 0 {
            self.update()
        }
        self.pos -= 1;
        self.v[self.pos]
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
