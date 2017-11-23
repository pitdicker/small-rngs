small-rngs
====

A collection of small pseudorandom number generators in Rust.

The intention of this repository is to collect small PRNGs, to make it easier
to comparing them.

This requires rustc nightly to build, because some RNGs need support for `u128`.

## Currently implemented RNGs
Various lesser-known PRNGs:
- `GjRng`: A small random number generator by Geronimo Jones.
- `Jsf32Rng`: A small random number generator designed by Bob Jenkins.
- `Jsf64Rng`: A small random number generator designed by Bob Jenkins
  (64-bit variant).
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
- `PcgXsh64LcgRng`: A PCG random number generator (XSH 64/32 (LCG) variant).
- `PcgXsl128LcgRng`: A PCG random number generator (XSL 128/64 (LCG) variant).
