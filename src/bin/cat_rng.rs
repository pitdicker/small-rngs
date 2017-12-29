// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! A small utility to concatenate the output of an RNG to stdout.

extern crate rand;
extern crate small_rngs;

use rand::{Rng, NewSeeded};
use small_rngs::*;
use std::collections::HashMap;
use std::env;
use std::io::{self, Write, Error};
use std::iter::Iterator;

fn print_usage(cmd: &String, names: Vec<String>) {
    println!("Usage: {} RNG
where RNG is one of: {:?}

This is a small tool to endlessly contatenate output from an RNG. It can for
example be used with PractRand: ./cat_rng jsf32 | RNG_test stdin -multithreaded",
        cmd, names);
}

type BR = Box<Rng>;

fn main() {
    let mut ctors: HashMap<&'static str,
            Box<Fn() -> Result<BR, ::rand::Error>>> = HashMap::new();
    ctors.insert("ci", Box::new(|| CiRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("gj", Box::new(|| GjRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("jsf32", Box::new(|| Jsf32Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("jsf64", Box::new(|| Jsf64Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("kiss32", Box::new(|| Kiss32Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("kiss64", Box::new(|| Kiss64Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("msws", Box::new(|| MswsRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("pcg_xsh_64_lcg", Box::new(|| PcgXsh64LcgRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("pcg_xsl_64_lcg", Box::new(|| PcgXsl64LcgRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("pcg_xsl_128_mcg", Box::new(|| PcgXsl128McgRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("sapparoth_32", Box::new(|| Sapparot32Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("sapparoth_64", Box::new(|| Sapparot64Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("sfc_32", Box::new(|| Sfc32Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("sfc_64", Box::new(|| Sfc64Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("velox", Box::new(|| Velox3bRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xorshift_128_32", Box::new(|| Xorshift128_32Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xorshift_128_64", Box::new(|| Xorshift128_64Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xorshift_128_plus", Box::new(|| Xorshift128PlusRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xorshift_mt_32", Box::new(|| XorshiftMt32Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xorshift_mt_64", Box::new(|| XorshiftMt64Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xoroshiro_128_plus", Box::new(|| Xoroshiro128PlusRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xoroshiro_64_plus", Box::new(|| Xoroshiro64PlusRng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xoroshiro_mt_64of128", Box::new(|| XoroshiroMt64of128Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xoroshiro_mt_32of128", Box::new(|| XoroshiroMt32of128Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xsm32", Box::new(|| Xsm32Rng::new().map(|rng| Box::new(rng) as BR)));
    ctors.insert("xsm64", Box::new(|| Xsm64Rng::new().map(|rng| Box::new(rng) as BR)));

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print_usage(&args[0], ctors.keys().map(|s| String::from(*s)).collect());
    } else {
        if let Some(ctor) = ctors.get(&*args[1]) {
            let rng = ctor().unwrap();
            cat_rng(rng).unwrap();
        } else {
            println!("Error: unknown RNG: {}", args[1]);
            println!();
            print_usage(&args[0], ctors.keys().map(|s| String::from(*s)).collect());
        }
    }
}

fn cat_rng(mut rng: Box<Rng>) -> Result<(), Error> {
    let mut buf =  [0u8; 32];
    let stdout = io::stdout();
    let mut lock = stdout.lock();

    loop {
        rng.fill_bytes(&mut buf);
        lock.write(&buf)?;
    }
}
