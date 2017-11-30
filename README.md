small-rngs
====

A collection of small pseudorandom number generators in Rust.

The intention of this repository is to collect small PRNGs, to make it easier
to comparing them.

This requires rustc nightly to build, because some RNGs need support for `u128`.

Note: not all implementations of RNGs are verified to be correct yet.

## Currently implemented RNGs
Various lesser-known PRNGs:
- `GjRng`: A small chaotic RNG by Geronimo Jones.
- `Jsf32Rng`, `Jsf64Rng`: A small random number generator designed by
  Bob Jenkins.
- `MswsRng`: Middle Square Weyl Sequence RNG.
- `Sapparot32Rng`, `Sapparot64Rng`: The Sapparoth-2 RNG by Ilya Levin.
- `Sfc32Rng`, `Sfc64Rng`: A small chaotic RNG combined with a counter, designed
  by Chris Doty-Humphrey.
- `Velox3bRng`: A small random number generator designed by Elias Yarrkov.

Xorshift family:
- `Xorshift128_32Rng`: An Xorshift random number generator (128/32-bit variant).
- `Xorshift128_64Rng`: An Xorshift random number generator (128/64-bit variant).
- `Xorshift128PlusRng`: The Xorshift128+ random number generator.
- `Xoroshiro128PlusRng`: The Xoroshiro128+ random number generator.
- `Xoroshiro64PlusRng`: A 32-bit variant of Xoroshiro128+,
  with just 64 bits of state.
- `XorshiftMultWT32Rng`
- `XorshiftMultWT64Rng`

PCG family:
- `PcgXsh64LcgRng`: A PCG random number generator (XSH 64/32 RR (LCG) variant).
- `PcgXsl64LcgRng`: A PCG random number generator (XSL 64/32 RR (LCG) variant).
- `PcgXsl128McgRng`: A PCG random number generator (XSL 128/64 RR (MCG) variant).

## Benchmarks

Result of `cargo bench`:
```rust
test gen_u32_gj                  ... bench:       3,333 ns/iter (+/- 3) = 1200 MB/s
test gen_u32_jsf32               ... bench:       2,229 ns/iter (+/- 1) = 1794 MB/s
test gen_u32_jsf64               ... bench:       2,312 ns/iter (+/- 4) = 1730 MB/s
test gen_u32_kiss32              ... bench:       3,015 ns/iter (+/- 1) = 1326 MB/s
test gen_u32_kiss64              ... bench:       3,023 ns/iter (+/- 4) = 1323 MB/s
test gen_u32_msws                ... bench:         971 ns/iter (+/- 2) = 4119 MB/s
test gen_u32_pcg_xsh_64_lcg      ... bench:       1,041 ns/iter (+/- 2) = 3842 MB/s
test gen_u32_pcg_xsl_64_lcg      ... bench:         976 ns/iter (+/- 0) = 4098 MB/s
test gen_u32_pcg_xsl_128_mcg     ... bench:       1,461 ns/iter (+/- 1) = 2737 MB/s
test gen_u32_sapparoth_32        ... bench:       2,544 ns/iter (+/- 4) = 1572 MB/s
test gen_u32_sapparoth_64        ... bench:       2,545 ns/iter (+/- 10) = 1571 MB/s
test gen_u32_sfc_32              ... bench:         980 ns/iter (+/- 4) = 4081 MB/s
test gen_u32_sfc_64              ... bench:       1,221 ns/iter (+/- 1) = 3276 MB/s
test gen_u32_velox               ... bench:       2,007 ns/iter (+/- 35) = 1993 MB/s
test gen_u32_xoroshiro_128_plus  ... bench:       1,196 ns/iter (+/- 1) = 3344 MB/s
test gen_u32_xoroshiro_64_plus   ... bench:       1,097 ns/iter (+/- 1) = 3646 MB/s
test gen_u32_xoroshiro_mt_32of128... bench:       1,060 ns/iter (+/- 9) = 3773 MB/s
test gen_u32_xoroshiro_mt_64of128... bench:       1,082 ns/iter (+/- 109) = 3696 MB/s
test gen_u32_xorshift_128_32     ... bench:       1,082 ns/iter (+/- 2) = 3696 MB/s
test gen_u32_xorshift_128_64     ... bench:         977 ns/iter (+/- 3) = 4094 MB/s
test gen_u32_xorshift_128_plus   ... bench:       1,186 ns/iter (+/- 3) = 3372 MB/s
test gen_u32_xorshift_mt_32      ... bench:       1,136 ns/iter (+/- 22) = 3521 MB/s
test gen_u32_xorshift_mt_64      ... bench:       1,280 ns/iter (+/- 4) = 3125 MB/s
test gen_u32_xsm32               ... bench:       2,511 ns/iter (+/- 8) = 1592 MB/s
test gen_u32_xsm64               ... bench:       2,423 ns/iter (+/- 8) = 1650 MB/s
test gen_u64_gj                  ... bench:       3,333 ns/iter (+/- 3) = 2400 MB/s
test gen_u64_kiss32              ... bench:       5,168 ns/iter (+/- 15) = 1547 MB/s
test gen_u64_kiss64              ... bench:       3,019 ns/iter (+/- 6) = 2649 MB/s
test gen_u64_jsf32               ... bench:       3,078 ns/iter (+/- 3) = 2599 MB/s
test gen_u64_jsf64               ... bench:       2,311 ns/iter (+/- 5) = 3461 MB/s
test gen_u64_msws                ... bench:         971 ns/iter (+/- 1) = 8238 MB/s
test gen_u64_pcg_xsh_64_lcg      ... bench:       2,738 ns/iter (+/- 10) = 2921 MB/s
test gen_u64_pcg_xsl_64_lcg      ... bench:       2,555 ns/iter (+/- 16) = 3131 MB/s
test gen_u64_pcg_xsl_128_mcg     ... bench:       1,468 ns/iter (+/- 6) = 5449 MB/s
test gen_u64_sapparoth_32        ... bench:       4,008 ns/iter (+/- 13) = 1996 MB/s
test gen_u64_sapparoth_64        ... bench:       2,546 ns/iter (+/- 5) = 3142 MB/s
test gen_u64_sfc_32              ... bench:       3,218 ns/iter (+/- 9) = 2486 MB/s
test gen_u64_sfc_64              ... bench:         980 ns/iter (+/- 3) = 8163 MB/s
test gen_u64_velox               ... bench:       3,667 ns/iter (+/- 35) = 2181 MB/s
test gen_u64_xoroshiro_128_plus  ... bench:       1,101 ns/iter (+/- 3) = 7266 MB/s
test gen_u64_xoroshiro_64_plus   ... bench:       3,624 ns/iter (+/- 25) = 2207 MB/s
test gen_u64_xoroshiro_mt_32of128... bench:       3,565 ns/iter (+/- 16) = 2244 MB/s
test gen_u64_xoroshiro_mt_64of128... bench:       1,094 ns/iter (+/- 12) = 7312 MB/s
test gen_u64_xorshift_128_32     ... bench:       2,638 ns/iter (+/- 23) = 3032 MB/s
test gen_u64_xorshift_128_64     ... bench:         979 ns/iter (+/- 3) = 8171 MB/s
test gen_u64_xorshift_128_plus   ... bench:       1,186 ns/iter (+/- 3) = 6745 MB/s
test gen_u64_xorshift_mt_32      ... bench:       3,411 ns/iter (+/- 29) = 2345 MB/s
test gen_u64_xorshift_mt_64      ... bench:       1,271 ns/iter (+/- 30) = 6294 MB/s
test gen_u64_xsm32               ... bench:       3,901 ns/iter (+/- 17) = 2050 MB/s
test gen_u64_xsm64               ... bench:       2,423 ns/iter (+/- 8) = 3301 MB/s
```

Result of `cargo bench --target i686-unknown-linux-musl`:
```rust
test gen_u32_gj                  ... bench:       8,787 ns/iter (+/- 118) = 455 MB/s
test gen_u32_jsf32               ... bench:       2,680 ns/iter (+/- 34) = 1492 MB/s
test gen_u32_jsf64               ... bench:       7,595 ns/iter (+/- 104) = 526 MB/s
test gen_u32_kiss32              ... bench:       3,668 ns/iter (+/- 66) = 1090 MB/s
test gen_u32_kiss64              ... bench:       9,084 ns/iter (+/- 124) = 440 MB/s
test gen_u32_msws                ... bench:       1,919 ns/iter (+/- 67) = 2084 MB/s
test gen_u32_pcg_xsh_64_lcg      ... bench:       1,999 ns/iter (+/- 11) = 2001 MB/s
test gen_u32_pcg_xsl_64_lcg      ... bench:       1,949 ns/iter (+/- 23) = 2052 MB/s
test gen_u32_pcg_xsl_128_mcg     ... bench:      12,736 ns/iter (+/- 121) = 314 MB/s
test gen_u32_sapparoth_32        ... bench:       2,615 ns/iter (+/- 27) = 1529 MB/s
test gen_u32_sapparoth_64        ... bench:       8,442 ns/iter (+/- 40) = 473 MB/s
test gen_u32_sfc_32              ... bench:       2,434 ns/iter (+/- 7) = 1643 MB/s
test gen_u32_sfc_64              ... bench:       4,125 ns/iter (+/- 14) = 969 MB/s
test gen_u32_velox               ... bench:       2,934 ns/iter (+/- 7) = 1363 MB/s
test gen_u32_xoroshiro_128_plus  ... bench:       2,442 ns/iter (+/- 21) = 1638 MB/s
test gen_u32_xoroshiro_64_plus   ... bench:       1,223 ns/iter (+/- 14) = 3270 MB/s
test gen_u32_xoroshiro_mt_32of128... bench:       3,658 ns/iter (+/- 52) = 1093 MB/s
test gen_u32_xoroshiro_mt_64of128... bench:       3,661 ns/iter (+/- 44) = 1092 MB/s
test gen_u32_xorshift_128_32     ... bench:       1,475 ns/iter (+/- 60) = 2711 MB/s
test gen_u32_xorshift_128_64     ... bench:       2,567 ns/iter (+/- 59) = 1558 MB/s
test gen_u32_xorshift_128_plus   ... bench:       2,444 ns/iter (+/- 17) = 1636 MB/s
test gen_u32_xorshift_mt_32      ... bench:       1,734 ns/iter (+/- 12) = 2306 MB/s
test gen_u32_xorshift_mt_64      ... bench:       3,482 ns/iter (+/- 13) = 1148 MB/s
test gen_u32_xsm32               ... bench:       3,189 ns/iter (+/- 157) = 1254 MB/s
test gen_u32_xsm64               ... bench:       6,007 ns/iter (+/- 283) = 665 MB/s
test gen_u64_gj                  ... bench:       8,812 ns/iter (+/- 125) = 907 MB/s
test gen_u64_jsf32               ... bench:       3,838 ns/iter (+/- 63) = 2084 MB/s
test gen_u64_jsf64               ... bench:       7,661 ns/iter (+/- 73) = 1044 MB/s
test gen_u64_kiss32              ... bench:       8,763 ns/iter (+/- 98) = 912 MB/s
test gen_u64_kiss64              ... bench:       8,918 ns/iter (+/- 135) = 897 MB/s
test gen_u64_msws                ... bench:       1,995 ns/iter (+/- 72) = 4010 MB/s
test gen_u64_pcg_xsh_64_lcg      ... bench:       4,421 ns/iter (+/- 24) = 1809 MB/s
test gen_u64_pcg_xsl_64_lcg      ... bench:       3,885 ns/iter (+/- 37) = 2059 MB/s
test gen_u64_pcg_xsl_128_mcg     ... bench:      13,802 ns/iter (+/- 254) = 579 MB/s
test gen_u64_sapparoth_32        ... bench:       4,360 ns/iter (+/- 82) = 1834 MB/s
test gen_u64_sapparoth_64        ... bench:       8,491 ns/iter (+/- 207) = 942 MB/s
test gen_u64_sfc_32              ... bench:       3,568 ns/iter (+/- 15) = 2242 MB/s
test gen_u64_sfc_64              ... bench:       4,914 ns/iter (+/- 17) = 1628 MB/s
test gen_u64_velox               ... bench:       5,282 ns/iter (+/- 82) = 1514 MB/s
test gen_u64_xoroshiro_128_plus  ... bench:       2,933 ns/iter (+/- 11) = 2727 MB/s
test gen_u64_xoroshiro_64_plus   ... bench:       3,850 ns/iter (+/- 65) = 2077 MB/s
test gen_u64_xoroshiro_mt_32of128... bench:       6,174 ns/iter (+/- 16) = 1295 MB/s
test gen_u64_xoroshiro_mt_64of128... bench:      11,884 ns/iter (+/- 83) = 673 MB/s
test gen_u64_xorshift_128_32     ... bench:       4,591 ns/iter (+/- 59) = 1742 MB/s
test gen_u64_xorshift_128_64     ... bench:       2,638 ns/iter (+/- 91) = 3032 MB/s
test gen_u64_xorshift_128_plus   ... bench:       3,014 ns/iter (+/- 5) = 2654 MB/s
test gen_u64_xorshift_mt_32      ... bench:       3,985 ns/iter (+/- 52) = 2007 MB/s
test gen_u64_xorshift_mt_64      ... bench:      11,624 ns/iter (+/- 25) = 688 MB/s
test gen_u64_xsm32               ... bench:       4,346 ns/iter (+/- 190) = 1840 MB/s
test gen_u64_xsm64               ... bench:       6,282 ns/iter (+/- 311) = 1273 MB/s
```
