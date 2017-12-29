#![feature(i128_type)]

extern crate rand_core;
extern crate core;

mod ciprng;
mod gj;
mod jsf;
mod kiss;
mod msws;
mod pcg;
mod sapparoth;
mod sfc;
mod velox;
mod xorshift;
mod xorshift_plus;
mod xorshift_mt;
mod xoroshiro;
mod xoroshiro_mt;
mod xsm;

pub use self::ciprng::CiRng;
pub use self::gj::GjRng;
pub use self::jsf::{Jsf32Rng, Jsf64Rng};
pub use self::kiss::{Kiss32Rng, Kiss64Rng};
pub use self::msws::MswsRng;
pub use self::pcg::{PcgXsh64LcgRng, PcgXsl64LcgRng, PcgXsl128McgRng};
pub use self::sapparoth::{Sapparot32Rng, Sapparot64Rng};
pub use self::sfc::{Sfc32Rng, Sfc64Rng};
pub use self::velox::Velox3bRng;
pub use self::xorshift::{Xorshift128_32Rng, Xorshift128_64Rng};
pub use self::xorshift_plus::Xorshift128PlusRng;
pub use self::xorshift_mt::{XorshiftMt32Rng, XorshiftMt64Rng};
pub use self::xoroshiro::{Xoroshiro128PlusRng, Xoroshiro64PlusRng};
pub use self::xoroshiro_mt::{XoroshiroMt32of128Rng, XoroshiroMt64of128Rng};
pub use self::xsm::{Xsm32Rng, Xsm64Rng};
