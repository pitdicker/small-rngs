#![feature(test)]

extern crate test;
extern crate core;
extern crate rand_core;
extern crate small_rngs;

const RAND_BENCH_N: u64 = 1000;

use core::mem::size_of_val;
use test::{black_box, Bencher};

use rand_core::{SeedableRng, Rng};
use small_rngs::*;

macro_rules! gen_uint {
    ($fnn:ident, $gen:ident, $rng:ident) => {
        #[bench]
        fn $fnn(b: &mut Bencher) {
            let mut master_rng =
                XoroshiroMt64of128Rng::from_seed([236, 186, 13, 169, 36, 22, 113, 213,
                                                  12, 21, 28, 253, 104, 247, 90, 186]);
            let mut rng = $rng::from_rng(&mut master_rng).unwrap();
            b.bytes = size_of_val(&rng.$gen()) as u64 * RAND_BENCH_N;
            b.iter(|| {
                for _ in 0..RAND_BENCH_N {
                    black_box(rng.$gen());
                }
            });
        }
    }
}

gen_uint!(gen_u32_ci, next_u32, CiRng);
gen_uint!(gen_u32_gj, next_u32, GjRng);
gen_uint!(gen_u32_jsf32, next_u32, Jsf32Rng);
gen_uint!(gen_u32_jsf64, next_u32, Jsf64Rng);
gen_uint!(gen_u32_kiss32, next_u32, Kiss32Rng);
gen_uint!(gen_u32_kiss64, next_u32, Kiss64Rng);
gen_uint!(gen_u32_msws, next_u32, MswsRng);
gen_uint!(gen_u32_mwp, next_u32, MwpRng);
gen_uint!(gen_u32_pcg_xsh_64_lcg, next_u32, PcgXsh64LcgRng);
gen_uint!(gen_u32_pcg_xsl_64_lcg, next_u32, PcgXsl64LcgRng);
gen_uint!(gen_u32_pcg_xsl_128_mcg, next_u32, PcgXsl128McgRng);
gen_uint!(gen_u32_sapparoth_32, next_u32, Sapparot32Rng);
gen_uint!(gen_u32_sapparoth_64, next_u32, Sapparot64Rng);
gen_uint!(gen_u32_sfc_32, next_u32, Sfc32Rng);
gen_uint!(gen_u32_sfc_64, next_u32, Sfc64Rng);
gen_uint!(gen_u32_velox, next_u32, Velox3bRng);
gen_uint!(gen_u32_xorshift_128_32, next_u32, Xorshift128_32Rng);
gen_uint!(gen_u32_xorshift_128_64, next_u32, Xorshift128_64Rng);
gen_uint!(gen_u32_xorshift_128_plus, next_u32, Xorshift128PlusRng);
gen_uint!(gen_u32_xorshift_mt_32, next_u32, XorshiftMt32Rng);
gen_uint!(gen_u32_xorshift_mt_64, next_u32, XorshiftMt64Rng);
gen_uint!(gen_u32_xoroshiro_128_plus, next_u32, Xoroshiro128PlusRng);
gen_uint!(gen_u32_xoroshiro_64_plus, next_u32, Xoroshiro64PlusRng);
gen_uint!(gen_u32_xoroshiro_mt_64of128, next_u32, XoroshiroMt64of128Rng);
gen_uint!(gen_u32_xoroshiro_mt_32of128, next_u32, XoroshiroMt32of128Rng);
gen_uint!(gen_u32_xsm32, next_u32, Xsm32Rng);
gen_uint!(gen_u32_xsm64, next_u32, Xsm64Rng);

gen_uint!(gen_u64_ci, next_u64, CiRng);
gen_uint!(gen_u64_gj, next_u64, GjRng);
gen_uint!(gen_u64_jsf32, next_u64, Jsf32Rng);
gen_uint!(gen_u64_jsf64, next_u64, Jsf64Rng);
gen_uint!(gen_u64_kiss32, next_u64, Kiss32Rng);
gen_uint!(gen_u64_kiss64, next_u64, Kiss64Rng);
gen_uint!(gen_u64_msws, next_u64, MswsRng);
gen_uint!(gen_u64_mwp, next_u64, MwpRng);
gen_uint!(gen_u64_sapparoth_32, next_u64, Sapparot32Rng);
gen_uint!(gen_u64_sapparoth_64, next_u64, Sapparot64Rng);
gen_uint!(gen_u64_sfc_32, next_u64, Sfc32Rng);
gen_uint!(gen_u64_sfc_64, next_u64, Sfc64Rng);
gen_uint!(gen_u64_pcg_xsh_64_lcg, next_u64, PcgXsh64LcgRng);
gen_uint!(gen_u64_pcg_xsl_64_lcg, next_u64, PcgXsl64LcgRng);
gen_uint!(gen_u64_pcg_xsl_128_mcg, next_u64, PcgXsl128McgRng);
gen_uint!(gen_u64_velox, next_u64, Velox3bRng);
gen_uint!(gen_u64_xorshift_128_32, next_u64, Xorshift128_32Rng);
gen_uint!(gen_u64_xorshift_128_64, next_u64, Xorshift128_64Rng);
gen_uint!(gen_u64_xorshift_128_plus, next_u64, Xorshift128PlusRng);
gen_uint!(gen_u64_xorshift_mt_32, next_u64, XorshiftMt32Rng);
gen_uint!(gen_u64_xorshift_mt_64, next_u64, XorshiftMt64Rng);
gen_uint!(gen_u64_xoroshiro_128_plus, next_u64, Xoroshiro128PlusRng);
gen_uint!(gen_u64_xoroshiro_64_plus, next_u64, Xoroshiro64PlusRng);
gen_uint!(gen_u64_xoroshiro_mt_64of128, next_u64, XoroshiroMt64of128Rng);
gen_uint!(gen_u64_xoroshiro_mt_32of128, next_u64, XoroshiroMt32of128Rng);
gen_uint!(gen_u64_xsm32, next_u64, Xsm32Rng);
gen_uint!(gen_u64_xsm64, next_u64, Xsm64Rng);
