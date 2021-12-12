use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Square, ModPowerOf2SquareAssign, Parity, ShrRound, Square, WrappingSquare,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf};
use malachite_base::rounding_modes::RoundingMode;
use natural::arithmetic::add::limbs_slice_add_same_length_in_place_left;
use natural::arithmetic::add_mul::limbs_slice_add_mul_limb_same_length_in_place_left;
use natural::arithmetic::mod_power_of_2::limbs_vec_mod_power_of_2_in_place;
use natural::arithmetic::mul::fft::limbs_mul_greater_to_out_fft;
use natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
use natural::arithmetic::mul::limbs_mul_greater_to_out_basecase;
use natural::arithmetic::mul::mul_low::{
    limbs_mul_low_same_length, limbs_mul_low_same_length_basecase,
};
use natural::arithmetic::mul::toom::{TUNE_PROGRAM_BUILD, WANT_FAT_BINARY};
use natural::arithmetic::shl::{limbs_shl_to_out, limbs_slice_shl_in_place};
use natural::arithmetic::square::{
    limbs_square, limbs_square_diagonal, limbs_square_to_out, limbs_square_to_out_basecase,
    SQR_FFT_THRESHOLD,
};
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{
    DoubleLimb, Limb, MULLO_BASECASE_THRESHOLD, MULLO_DC_THRESHOLD, SQRLO_DC_THRESHOLD,
    SQR_TOOM2_THRESHOLD, SQR_TOOM3_THRESHOLD, SQR_TOOM4_THRESHOLD, SQR_TOOM8_THRESHOLD,
};

/// This is MPN_SQRLO_DIAGONAL from mpn/generic/sqrlo_basecase.c, GMP 6.2.1.
fn limbs_square_low_diagonal(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let half_n = n >> 1;
    limbs_square_diagonal(out, &xs[..half_n]);
    if n.odd() {
        out[n - 1] = xs[half_n].wrapping_square();
    }
}

/// This is MPN_SQRLO_DIAG_ADDLSH1 from mpn/generic/sqrlo_basecase.c, GMP 6.2.1.
#[doc(hidden)]
pub fn limbs_square_diagonal_shl_add(out: &mut [Limb], scratch: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    assert_eq!(scratch.len(), n - 1);
    assert_eq!(out.len(), n);
    limbs_square_low_diagonal(out, xs);
    limbs_slice_shl_in_place(scratch, 1);
    limbs_slice_add_same_length_in_place_left(&mut out[1..], scratch);
}

//TODO tune
pub const SQRLO_DC_THRESHOLD_LIMIT: usize = 500;

//TODO tune
const SQRLO_BASECASE_ALLOC: usize = if SQRLO_DC_THRESHOLD_LIMIT < 2 {
    1
} else {
    SQRLO_DC_THRESHOLD_LIMIT - 1
};

/// TODO complexity
///
/// This is mpn_sqrlo_basecase from mpn/generic/sqrlo_basecase.c, GMP 6.2.1.
#[doc(hidden)]
pub fn limbs_square_low_basecase(out: &mut [Limb], xs: &[Limb]) {
    let n = xs.len();
    let out = &mut out[..n];
    assert_ne!(n, 0);
    let xs_0 = xs[0];
    match n {
        1 => out[0] = xs_0.wrapping_square(),
        2 => {
            let (p_hi, p_lo) = DoubleLimb::from(xs_0).square().split_in_half();
            out[0] = p_lo;
            out[1] = (xs_0.wrapping_mul(xs[1]) << 1).wrapping_add(p_hi);
        }
        _ => {
            let scratch = &mut [0; SQRLO_BASECASE_ALLOC];
            // must fit n - 1 limbs in scratch
            assert!(n <= SQRLO_DC_THRESHOLD_LIMIT);
            let scratch = &mut scratch[..n - 1];
            limbs_mul_limb_to_out(scratch, &xs[1..], xs_0);
            for i in 1.. {
                let two_i = i << 1;
                if two_i >= n - 1 {
                    break;
                }
                limbs_slice_add_mul_limb_same_length_in_place_left(
                    &mut scratch[two_i..],
                    &xs[i + 1..n - i],
                    xs[i],
                );
            }
            limbs_square_diagonal_shl_add(out, scratch, xs);
        }
    }
}

//TODO tune
const SQRLO_BASECASE_THRESHOLD: usize = 8;

//TODO tune
/// This is MAYBE_range_basecase from mpn/generic/sqrlo.c, GMP 6.2.1. Investigate changes from
/// 6.1.2?
const MAYBE_RANGE_BASECASE_MOD_SQUARE: bool = TUNE_PROGRAM_BUILD
    || WANT_FAT_BINARY
    || (if SQRLO_DC_THRESHOLD == 0 {
        SQRLO_BASECASE_THRESHOLD
    } else {
        SQRLO_DC_THRESHOLD
    }) < SQR_TOOM2_THRESHOLD * 36 / (36 - 11);

//TODO tune
/// This is MAYBE_range_toom22 from mpn/generic/sqrlo.c, GMP 6.2.1. Investigate changes from 6.1.2?
const MAYBE_RANGE_TOOM22_MOD_SQUARE: bool = TUNE_PROGRAM_BUILD
    || WANT_FAT_BINARY
    || (if SQRLO_DC_THRESHOLD == 0 {
        SQRLO_BASECASE_THRESHOLD
    } else {
        SQRLO_DC_THRESHOLD
    }) < SQR_TOOM3_THRESHOLD * 36 / (36 - 11);

/// This is mpn_sqrlo_itch from mpn/generic/sqrlo.c, GMP 6.2.1. Investigate changes from 6.1.2?
#[doc(hidden)]
pub const fn limbs_square_low_scratch_len(len: usize) -> usize {
    len << 1
}

/// Requires a scratch space of 2 * `xs.len()` limbs at `scratch`.
///
/// TODO complexity
///
/// This is mpn_dc_sqrlo from mpn/generic/sqrlo.c, GMP 6.2.1. Investigate changes from 6.1.2?
#[allow(clippy::absurd_extreme_comparisons)]
#[doc(hidden)]
pub fn limbs_square_low_divide_and_conquer(out: &mut [Limb], xs: &[Limb], scratch: &mut [Limb]) {
    let len = xs.len();
    let out = &mut out[..len];
    assert!(len > 1);
    // We need a fractional approximation of the value 0 < a <= 1/2, giving the minimum in the
    // function k = (1 - a) ^ e / (1 - 2 * a ^ e).
    let len_small = if MAYBE_RANGE_BASECASE_MOD_SQUARE && len < SQR_TOOM2_THRESHOLD * 36 / (36 - 11)
    {
        len >> 1
    } else if MAYBE_RANGE_TOOM22_MOD_SQUARE && len < SQR_TOOM3_THRESHOLD * 36 / (36 - 11) {
        len * 11 / 36 // n1 ~= n*(1-.694...)
    } else if len < SQR_TOOM4_THRESHOLD * 40 / (40 - 9) {
        len * 9 / 40 // n1 ~= n*(1-.775...)
    } else if len < SQR_TOOM8_THRESHOLD * 10 / 9 {
        len * 7 / 39 // n1 ~= n*(1-.821...)
    } else {
        len / 10 // n1 ~= n*(1-.899...) [TOOM88]
    };
    let len_big = len - len_small;
    // x0 ^ 2
    let (xs_lo, xs_hi) = xs.split_at(len_big);
    limbs_square_to_out(scratch, xs_lo);
    let xs_lo = &xs_lo[..len_small];
    let (out_lo, out_hi) = out.split_at_mut(len_big);
    let (scratch_lo, scratch_hi) = scratch.split_at_mut(len);
    out_lo.copy_from_slice(&scratch_lo[..len_big]);
    // x1 * x0 * 2^(n2 GMP_NUMB_BITS)
    if len_small < MULLO_BASECASE_THRESHOLD {
        limbs_mul_greater_to_out_basecase(scratch_hi, xs_hi, xs_lo);
    } else if len_small < MULLO_DC_THRESHOLD {
        limbs_mul_low_same_length_basecase(scratch_hi, xs_hi, xs_lo);
    } else {
        limbs_mul_low_same_length(scratch_hi, xs_hi, xs_lo);
    }
    limbs_shl_to_out(out_hi, &scratch_hi[..len_small], 1);
    limbs_slice_add_same_length_in_place_left(out_hi, &scratch_lo[len_big..]);
}

//TODO tune
// must be at least SQRLO_BASECASE_THRESHOLD
const SQRLO_BASECASE_THRESHOLD_LIMIT: usize = 8;

//TODO tune
const SQRLO_SQR_THRESHOLD: usize = 6440;

//TODO tune
const SQR_BASECASE_ALLOC: usize = if SQRLO_BASECASE_THRESHOLD_LIMIT == 0 {
    1
} else {
    SQRLO_BASECASE_THRESHOLD_LIMIT << 1
};

/// Square an n-limb number and return the lowest n limbs of the result.
///
/// //TODO complexity
///
/// This is mpn_sqrlo from mpn/generic/sqrlo.c, GMP 6.2.1. Investigate changes from 6.1.2?
#[doc(hidden)]
pub fn limbs_square_low(out: &mut [Limb], xs: &[Limb]) {
    assert!(SQRLO_BASECASE_THRESHOLD_LIMIT >= SQRLO_BASECASE_THRESHOLD);
    let len = xs.len();
    assert_ne!(len, 0);
    let out = &mut out[..len];
    if len < SQRLO_BASECASE_THRESHOLD {
        // Allocate workspace of fixed size on stack: fast!
        let scratch = &mut [0; SQR_BASECASE_ALLOC];
        limbs_square_to_out_basecase(scratch, xs);
        out.copy_from_slice(&scratch[..len]);
    } else if len < SQRLO_DC_THRESHOLD {
        limbs_square_low_basecase(out, xs);
    } else {
        let mut scratch = vec![0; limbs_square_low_scratch_len(len)];
        if len < SQRLO_SQR_THRESHOLD {
            limbs_square_low_divide_and_conquer(out, xs, &mut scratch);
        } else {
            // For really large operands, use plain mpn_mul_n but throw away upper n limbs of the
            // result.
            if !TUNE_PROGRAM_BUILD && SQRLO_SQR_THRESHOLD > SQR_FFT_THRESHOLD {
                limbs_mul_greater_to_out_fft(&mut scratch, xs, xs);
            } else {
                limbs_square_to_out(&mut scratch, xs);
            }
            out.copy_from_slice(&scratch[..len]);
        }
    }
}

/// Interpreting a `Vec<Limb>` as the limbs (in ascending order) of a `Natural`, returns a `Vec` of
/// the limbs of the square of the `Natural` mod 2<sup>`pow`</sup>. Assumes the input is already
/// reduced mod 2<sup>`pow`</sup>. The input `Vec` may be mutated. The input may not be empty or
/// have trailing zeros.
///
/// TODO complexity
///
/// # Panics
/// Panics if the input is empty. May panic if the input has trailing zeros.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_2_square::limbs_mod_power_of_2_square;
///
/// assert_eq!(limbs_mod_power_of_2_square(&mut vec![25], 5), &[17]);
/// assert_eq!(limbs_mod_power_of_2_square(&mut vec![123, 456], 42), &[15129, 560]);
/// ```
#[doc(hidden)]
pub fn limbs_mod_power_of_2_square(xs: &mut Vec<Limb>, pow: u64) -> Vec<Limb> {
    let len = xs.len();
    assert_ne!(len, 0);
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling));
    if max_len > len << 1 {
        return limbs_square(xs);
    }
    // Should really be max_len / sqrt(2); 0.75 * max_len is close enough
    let limit = max_len.checked_mul(3).unwrap() >> 2;
    let mut square = if len >= limit {
        if len != max_len {
            xs.resize(max_len, 0);
        }
        let mut square_limbs = vec![0; max_len];
        limbs_square_low(&mut square_limbs, xs);
        square_limbs
    } else {
        limbs_square(xs)
    };
    limbs_vec_mod_power_of_2_in_place(&mut square, pow);
    square
}

/// Interpreting a slice of `Limb` as the limbs (in ascending order) of a `Natural`, returns a `Vec`
/// of the limbs of the square of the `Natural` mod 2<sup>`pow`</sup>. Assumes the input is already
/// reduced mod 2<sup>`pow`</sup>. The input may not be empty or have trailing zeros.
///
/// TODO complexity
///
/// # Panics
/// Panics if the input is empty. May panic if the input has trailing zeros.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::mod_power_of_2_square::*;
///
/// assert_eq!(limbs_mod_power_of_2_square_ref(&[25], 5), &[17]);
/// assert_eq!(limbs_mod_power_of_2_square_ref(&[123, 456], 42), &[15129, 560]);
/// ```
#[doc(hidden)]
pub fn limbs_mod_power_of_2_square_ref(xs: &[Limb], pow: u64) -> Vec<Limb> {
    let len = xs.len();
    assert_ne!(len, 0);
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, RoundingMode::Ceiling));
    if max_len > len << 1 {
        return limbs_square(xs);
    }
    // Should really be max_len / sqrt(2); 0.75 * max_len is close enough
    let limit = max_len.checked_mul(3).unwrap() >> 2;
    let mut square = if len >= limit {
        let mut xs_adjusted_vec;
        let xs_adjusted = if len == max_len {
            xs
        } else {
            xs_adjusted_vec = vec![0; max_len];
            xs_adjusted_vec[..len].copy_from_slice(xs);
            &xs_adjusted_vec
        };
        let mut square = vec![0; max_len];
        limbs_square_low(&mut square, xs_adjusted);
        square
    } else {
        limbs_square(xs)
    };
    limbs_vec_mod_power_of_2_in_place(&mut square, pow);
    square
}

impl ModPowerOf2Square for Natural {
    type Output = Natural;

    /// Computes `self.square()` mod 2<sup>`pow`</sup>, taking `self` by value. Assumes the input is
    /// already reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_2_square(2), 0);
    /// assert_eq!(Natural::from(5u32).mod_power_of_2_square(3), 1);
    /// assert_eq!(
    ///     Natural::from_str("12345678987654321").unwrap().mod_power_of_2_square(64).to_string(),
    ///     "16556040056090124897"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_square(mut self, pow: u64) -> Natural {
        self.mod_power_of_2_square_assign(pow);
        self
    }
}

impl<'a> ModPowerOf2Square for &'a Natural {
    type Output = Natural;

    /// Computes `self.square()` mod 2<sup>`pow`</sup>, taking `self` by reference. Assumes the
    /// input is already reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&Natural::ZERO).mod_power_of_2_square(2), 0);
    /// assert_eq!((&Natural::from(5u32)).mod_power_of_2_square(3), 1);
    /// assert_eq!(
    ///     (&Natural::from_str("12345678987654321").unwrap())
    ///         .mod_power_of_2_square(64).to_string(),
    ///     "16556040056090124897"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_square(self, pow: u64) -> Natural {
        match self {
            &natural_zero!() => Natural::ZERO,
            Natural(Small(x)) if pow <= Limb::WIDTH => Natural(Small(x.mod_power_of_2_square(pow))),
            Natural(Small(x)) => {
                let x_double = DoubleLimb::from(*x);
                Natural::from(if pow <= Limb::WIDTH << 1 {
                    x_double.mod_power_of_2_square(pow)
                } else {
                    x_double.square()
                })
            }
            Natural(Large(ref xs)) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_square_ref(xs, pow))
            }
        }
    }
}

impl ModPowerOf2SquareAssign for Natural {
    /// Replaces `self` with `self.square()` mod 2<sup>`pow`</sup>. Assumes the input is already
    /// reduced mod 2<sup>`pow`</sup>.
    ///
    /// TODO complexity
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// let mut n = Natural::ZERO;
    /// n.mod_power_of_2_square_assign(2);
    /// assert_eq!(n, 0);
    ///
    /// let mut n = Natural::from(5u32);
    /// n.mod_power_of_2_square_assign(3);
    /// assert_eq!(n, 1);
    ///
    /// let mut n = Natural::from_str("12345678987654321").unwrap();
    /// n.mod_power_of_2_square_assign(64);
    /// assert_eq!(n.to_string(), "16556040056090124897");
    /// ```
    #[inline]
    fn mod_power_of_2_square_assign(&mut self, pow: u64) {
        match self {
            natural_zero!() => {}
            Natural(Small(ref mut x)) if pow <= Limb::WIDTH => x.mod_power_of_2_square_assign(pow),
            Natural(Small(x)) => {
                let x_double = DoubleLimb::from(*x);
                *self = Natural::from(if pow <= Limb::WIDTH << 1 {
                    x_double.mod_power_of_2_square(pow)
                } else {
                    x_double.square()
                })
            }
            Natural(Large(ref mut xs)) => {
                *xs = limbs_mod_power_of_2_square(xs, pow);
                self.trim();
            }
        }
    }
}
