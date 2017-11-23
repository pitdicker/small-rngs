#![feature(test)]

extern crate test;
extern crate rand;
extern crate small_rngs;

const RAND_BENCH_N: u64 = 1000;

use std::mem::size_of;
use test::{black_box, Bencher};

use rand::{NewSeeded, Sample};
use small_rngs::*;

macro_rules! gen_uint {
    ($fnn:ident, $ty:ty, $gen:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut rng = $gen::new().unwrap();
            b.iter(|| {
                for _ in 0..RAND_BENCH_N {
                    black_box(rng.gen::<$ty>());
                }
            });
            b.bytes = size_of::<$ty>() as u64 * RAND_BENCH_N;
        }
    }
}

gen_uint!(gen_u32_jsf32, u32, Jsf32Rng);
gen_uint!(gen_u32_jsf64, u32, Jsf64Rng);
gen_uint!(gen_u32_gj, u32, GjRng);
gen_uint!(gen_u32_velox, u32, Velox3bRng);
gen_uint!(gen_u32_pcg_xsh_64_lcg, u32, PcgXsh64LcgRng);
gen_uint!(gen_u32_pcg_xsl_128_lcg, u32, PcgXsl128LcgRng);
gen_uint!(gen_u32_sapparoth_32, u32, Sapparot32Rng);
gen_uint!(gen_u32_sapparoth_64, u32, Sapparot64Rng);
gen_uint!(gen_u32_sfc_32, u32, Sfc32Rng);
gen_uint!(gen_u32_sfc_64, u32, Sfc64Rng);
gen_uint!(gen_u32_xorshift_128_32, u32, Xorshift128_32Rng);
gen_uint!(gen_u32_xorshift_128_64, u32, Xorshift128_64Rng);
gen_uint!(gen_u32_xorshift_128_plus, u32, Xorshift128PlusRng);
gen_uint!(gen_u32_xorshift_mult_wt_32, u32, XorshiftMultWT32Rng);
gen_uint!(gen_u32_xorshift_mult_wt_64, u32, XorshiftMultWT64Rng);
gen_uint!(gen_u32_xoroshiro_128_plus, u32, Xoroshiro128PlusRng);
gen_uint!(gen_u32_xoroshiro_64_plus, u32, Xoroshiro64PlusRng);

gen_uint!(gen_u64_jsf32, u64, Jsf32Rng);
gen_uint!(gen_u64_jsf64, u64, Jsf64Rng);
gen_uint!(gen_u64_gj, u64, GjRng);
gen_uint!(gen_u64_velox, u64, Velox3bRng);
gen_uint!(gen_u64_sapparoth_32, u64, Sapparot32Rng);
gen_uint!(gen_u64_sapparoth_64, u64, Sapparot64Rng);
gen_uint!(gen_u64_sfc_32, u64, Sfc32Rng);
gen_uint!(gen_u64_sfc_64, u64, Sfc64Rng);
gen_uint!(gen_u64_pcg_xsh_64_lcg, u64, PcgXsh64LcgRng);
gen_uint!(gen_u64_pcg_xsl_128_lcg, u64, PcgXsl128LcgRng);
gen_uint!(gen_u64_xorshift_128_32, u64, Xorshift128_32Rng);
gen_uint!(gen_u64_xorshift_128_64, u64, Xorshift128_64Rng);
gen_uint!(gen_u64_xorshift_128_plus, u64, Xorshift128PlusRng);
gen_uint!(gen_u64_xorshift_mult_wt_32, u64, XorshiftMultWT32Rng);
gen_uint!(gen_u64_xorshift_mult_wt_64, u64, XorshiftMultWT64Rng);
gen_uint!(gen_u64_xoroshiro_128_plus, u64, Xoroshiro128PlusRng);
gen_uint!(gen_u64_xoroshiro_64_plus, u64, Xoroshiro64PlusRng);
