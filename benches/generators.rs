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

gen_uint!(gen_u32_ci, u32, CiRng);
gen_uint!(gen_u32_gj, u32, GjRng);
gen_uint!(gen_u32_jsf32, u32, Jsf32Rng);
gen_uint!(gen_u32_jsf64, u32, Jsf64Rng);
gen_uint!(gen_u32_kiss32, u32, Kiss32Rng);
gen_uint!(gen_u32_kiss64, u32, Kiss64Rng);
gen_uint!(gen_u32_msws, u32, MswsRng);
gen_uint!(gen_u32_mwp, u32, MwpRng);
gen_uint!(gen_u32_pcg_xsh_64_lcg, u32, PcgXsh64LcgRng);
gen_uint!(gen_u32_pcg_xsl_64_lcg, u32, PcgXsl64LcgRng);
gen_uint!(gen_u32_pcg_xsl_128_mcg, u32, PcgXsl128McgRng);
gen_uint!(gen_u32_sapparoth_32, u32, Sapparot32Rng);
gen_uint!(gen_u32_sapparoth_64, u32, Sapparot64Rng);
gen_uint!(gen_u32_sfc_32, u32, Sfc32Rng);
gen_uint!(gen_u32_sfc_64, u32, Sfc64Rng);
gen_uint!(gen_u32_velox, u32, Velox3bRng);
gen_uint!(gen_u32_xorshift_128_32, u32, Xorshift128_32Rng);
gen_uint!(gen_u32_xorshift_128_64, u32, Xorshift128_64Rng);
gen_uint!(gen_u32_xorshift_128_plus, u32, Xorshift128PlusRng);
gen_uint!(gen_u32_xorshift_mt_32, u32, XorshiftMt32Rng);
gen_uint!(gen_u32_xorshift_mt_64, u32, XorshiftMt64Rng);
gen_uint!(gen_u32_xoroshiro_128_plus, u32, Xoroshiro128PlusRng);
gen_uint!(gen_u32_xoroshiro_64_plus, u32, Xoroshiro64PlusRng);
gen_uint!(gen_u32_xoroshiro_mt_64of128, u32, XoroshiroMt64of128Rng);
gen_uint!(gen_u32_xoroshiro_mt_32of128, u32, XoroshiroMt32of128Rng);
gen_uint!(gen_u32_xsm32, u32, Xsm32Rng);
gen_uint!(gen_u32_xsm64, u32, Xsm64Rng);

gen_uint!(gen_u64_ci, u64, CiRng);
gen_uint!(gen_u64_gj, u64, GjRng);
gen_uint!(gen_u64_jsf32, u64, Jsf32Rng);
gen_uint!(gen_u64_jsf64, u64, Jsf64Rng);
gen_uint!(gen_u64_kiss32, u64, Kiss32Rng);
gen_uint!(gen_u64_kiss64, u64, Kiss64Rng);
gen_uint!(gen_u64_msws, u64, MswsRng);
gen_uint!(gen_u64_mwp, u64, MwpRng);
gen_uint!(gen_u64_sapparoth_32, u64, Sapparot32Rng);
gen_uint!(gen_u64_sapparoth_64, u64, Sapparot64Rng);
gen_uint!(gen_u64_sfc_32, u64, Sfc32Rng);
gen_uint!(gen_u64_sfc_64, u64, Sfc64Rng);
gen_uint!(gen_u64_pcg_xsh_64_lcg, u64, PcgXsh64LcgRng);
gen_uint!(gen_u64_pcg_xsl_64_lcg, u64, PcgXsl64LcgRng);
gen_uint!(gen_u64_pcg_xsl_128_mcg, u64, PcgXsl128McgRng);
gen_uint!(gen_u64_velox, u64, Velox3bRng);
gen_uint!(gen_u64_xorshift_128_32, u64, Xorshift128_32Rng);
gen_uint!(gen_u64_xorshift_128_64, u64, Xorshift128_64Rng);
gen_uint!(gen_u64_xorshift_128_plus, u64, Xorshift128PlusRng);
gen_uint!(gen_u64_xorshift_mt_32, u64, XorshiftMt32Rng);
gen_uint!(gen_u64_xorshift_mt_64, u64, XorshiftMt64Rng);
gen_uint!(gen_u64_xoroshiro_128_plus, u64, Xoroshiro128PlusRng);
gen_uint!(gen_u64_xoroshiro_64_plus, u64, Xoroshiro64PlusRng);
gen_uint!(gen_u64_xoroshiro_mt_64of128, u64, XoroshiroMt64of128Rng);
gen_uint!(gen_u64_xoroshiro_mt_32of128, u64, XoroshiroMt32of128Rng);
gen_uint!(gen_u64_xsm32, u64, Xsm32Rng);
gen_uint!(gen_u64_xsm64, u64, Xsm64Rng);
