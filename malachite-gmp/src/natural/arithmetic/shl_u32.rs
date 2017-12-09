use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Shl, ShlAssign};

/// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO << 10u32).to_string(), "0");
///     assert_eq!((Natural::from(123u32) << 2u32).to_string(), "492");
///     assert_eq!((Natural::from(123u32) << 100u32).to_string(),
///         "155921023828072216384094494261248");
/// }
/// ```
impl Shl<u32> for Natural {
    type Output = Natural;

    fn shl(mut self, other: u32) -> Natural {
        self <<= other;
        self
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2), taking the `Natural` by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO << 10u32).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) << 2u32).to_string(), "492");
///     assert_eq!((&Natural::from(123u32) << 100u32).to_string(),
///         "155921023828072216384094494261248");
/// }
/// ```
impl<'a> Shl<u32> for &'a Natural {
    type Output = Natural;

    fn shl(self, other: u32) -> Natural {
        if other == 0 || self == &0 {
            return self.clone();
        }
        match *self {
            Small(small) if other <= small.leading_zeros() => Small(small << other),
            Small(small) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut result, small.into());
                gmp::mpz_mul_2exp(&mut result, &result, other.into());
                Large(result)
            },
            Large(ref large) => unsafe {
                let mut result: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut result);
                gmp::mpz_mul_2exp(&mut result, large, other.into());
                Large(result)
            },
        }
    }
}

/// Shifts a `Natural` left (multiplies it by a power of 2) in place.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::One;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ONE;
///     x <<= 1;
///     x <<= 2;
///     x <<= 3;
///     x <<= 4;
///     assert_eq!(x.to_string(), "1024");
/// }
/// ```
impl ShlAssign<u32> for Natural {
    fn shl_assign(&mut self, other: u32) {
        if other == 0 || *self == 0 {
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            large,
            {
                if other <= small.leading_zeros() {
                    Some(*small << other)
                } else {
                    None
                }
            },
            {
                unsafe { gmp::mpz_mul_2exp(large, large, other.into()) }
            }
        );
    }
}
