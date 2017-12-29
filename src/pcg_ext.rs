// PCG extension scheme. Unused and untested, but maybe a good starting point for someone.

let kdd: bool = true;
const MCG: bool = true;

let table_pow2: usize = 6; // 2^6=64 bits
let advance_pow2: usize = 32; // 32 words of state
let stypebits: usize = 64; // size of state words of the base generator
let table_size: usize = 1 << table_pow2;

const TICK_LIMIT_POW2: usize = 64;




fn get_extended_value(self) -> u32 {
    let mut state = self.state;
    // If we want k-dimensional equidistribution (kdd) and the base RNG is MCG:
    if kdd && MCG {
        state >>= 2; // The low order bits of an MCG are constant, so drop them.
    }

    let index;
    if kdd {
        table_mask = (1 << table_pow2) - 1;
        index = state & table_mask;
    } else {
        table_shift: usize = stypebits - table_pow2;
        index = state >> table_shift;
    }

    let may_tick = (advance_pow2 < stypebits) && (advance_pow2 < TICK_LIMIT_POW2);
    if may_tick {
        let tick;
        if kdd {
            tick_mask = (1 << advance_pow2) - 1;
            tick = (state & tick_mask) == 0;
        } else {
            tick_shift = stypebits - advance_pow2;
            tick = (state >> tick_shift) == 0;
        };
        if tick { advance_table(); }
    }

    let may_tock = stypebits < TICK_LIMIT_POW2;
    if may_tock {
        tock = state == 0;
        if tock { advance_table(); }
    }
    data[index]
}

rhs = get_extended_value(self);
lhs = self.next_u32();
lhs ^ rhs

fn advance_table(self) {
    let mut carry = false;
    for i in 0..table_size {
        if carry {
            carry = insideout_external_step(&mut data[i], i + 1);
        }
        let carry2 = insideout_external_step(&mut data[i], i + 1);
        carry |= carry2;
    }
}

fn insideout_external_step(randval: &mut u32, i: u32) -> bool {
    // Recover LCG/MCG state of the value in the extension array, step the base
    // RNG, and apply the output function again
    state = rxs_m_xs_unoutput(randval);
    state = state.wrapping_mul(MULTIPLIER)
                 .wrapping_add(INCREMENT)
                 .wrapping_add(i * 2); // FIXME: why?
    result = self.rxs_m_xs_output(state);

    randval = result;
    if MCG { state & 3 != 0 } else { false }
}


macro_rules! rxs_m_xs {
    ($fnn:ident, $ty:ty, $bits:expr, $opbits:expr /* log2($bits) */,
     $multiplier:expr, $unmultiplier:expr) => {
        fn output(mut internal: $ty) => $ty {
            let mask = (1 << $opbits) - 1;

            let rshift = (internal >> ($bits - $opbits)) & mask;
            internal ^= internal >> ($opbits + rshift);
            internal *= $multiplier;
            result = internal;
            result ^= result >> ((2 * bits + 2) / 3);
            result
        }

        fn unoutput(mut internal: $ty) => $ty {
            let mask = (1 << $opbits) - 1;

            internal = internal.unxorshift((2 * bits + 2) / 3);
            internal *= $unmultiplier;

            let rshift = (internal >> ($bits - $opbits)) & mask;
            internal = internal.unxorshift($opbits + rshift);
            internal
        }
}

rxs_m_xs!(rxs_m_xs_32, u32, 32, 5, 277803737, 2897767785);
rxs_m_xs!(rxs_m_xs_64, u64, 64, 6, 12605985483714917081, 15009553638781119849);

trait UnXorShift {
    fn unxorshift(self, shift: usize) -> Self;
}

macro_rules! unxorshift_impl {
    ($ty:ty, $bits:expr) => {

        impl UnXorShift for $ty {
            fn unxorshift(self, shift: usize) -> Self {
                fn unxorshift_inner(self, bits: usize, shift: usize) -> Self {
                    if (2 * shift >= bits) {
                        return self ^ (self >> shift);
                    }
                    let lowmask1 = (1 << (bits - shift*2)) - 1;
                    let highmask1 = ~lowmask1;
                    let mut top1 = self;
                    let bottom1 = self & lowmask1;
                    top1 ^= top1 >> shift;
                    top1 &= highmask1;21
                    let x = top1 | bottom1;
                    let lowmask2 = (1 << (bits - shift)) - 1;
                    let mut bottom2 = x & lowmask2;
                    bottom2 = unxorshift_inner(bottom2, bits - shift, shift);
                    bottom2 &= lowmask1;
                    top1 | bottom2
                }
                unxorshift_inner(self, $bits, shift)
            }
        }
    }
}

unxorshift_impl!(u32, 32);
unxorshift_impl!(u64, 64);
