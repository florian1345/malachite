use malachite_base::named::Named;
use malachite_base::num::basic::traits::{NegativeOne, One, Two, Zero};
use malachite_base::num::conversion::traits::FromStringBase;
use natural::InnerNatural::Small;
use natural::Natural;
use std::convert::TryFrom;
#[cfg(feature = "test_build")]
use std::str::FromStr;

/// An integer.
///
/// Any `Integer` whose absolute value is small enough to fit into a `Limb` is represented inline.
/// Only integers outside this range incur the costs of heap-allocation.
///
/// On a 64-bit system, an `Integer` takes up 40 bytes of space on the stack.
#[derive(Clone, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(
    feature = "serde",
    serde(try_from = "SerdeInteger", into = "SerdeInteger")
)]
pub struct Integer {
    // whether the `Integer` is non-negative
    pub(crate) sign: bool,
    pub(crate) abs: Natural,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
struct SerdeInteger(String);

impl From<Integer> for SerdeInteger {
    #[inline]
    fn from(x: Integer) -> SerdeInteger {
        SerdeInteger(format!("{:#x}", x))
    }
}

impl TryFrom<SerdeInteger> for Integer {
    type Error = String;

    #[inline]
    fn try_from(s: SerdeInteger) -> Result<Integer, String> {
        if s.0.starts_with('-') {
            if s.0.starts_with("-0x") {
                Ok(Integer::from_sign_and_abs(
                    false,
                    Natural::from_string_base(16, &s.0[3..])
                        .ok_or_else(|| format!("Unrecognized digits in {}", s.0))?,
                ))
            } else {
                Err(format!(
                    "String '{}' starts with '-' but not with '-0x'",
                    s.0
                ))
            }
        } else if s.0.starts_with("0x") {
            Ok(Integer::from(
                Natural::from_string_base(16, &s.0[2..])
                    .ok_or_else(|| format!("Unrecognized digits in {}", s.0))?,
            ))
        } else {
            Err(format!(
                "String '{}' does not start with '0x' or '-0x'",
                s.0
            ))
        }
    }
}

impl Integer {
    /// Returns true iff `self` is valid.
    ///
    /// To be valid, its absolute value must be valid, and if the absolute value is zero, the sign
    /// must be true. All `Integer`s must be valid.

    #[cfg(feature = "test_build")]
    pub fn is_valid(&self) -> bool {
        self.abs.is_valid() && (self.sign || self.abs != 0)
    }

    #[cfg(feature = "test_build")]
    pub fn trillion() -> Integer {
        Integer::from_str("1000000000000").unwrap()
    }

    /// Converts a sign and an `Natural` to an `Integer`, taking the `Natural` by value. The
    /// `Natural` becomes the `Integer`'s absolute value, and the sign indicates whether the
    /// `Integer` should be non-negative. If the `Natural` is zero, then the `Integer` will be
    /// non-negative regardless of the sign.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Integer::from_sign_and_abs(true, Natural::from(123u32)), 123);
    /// assert_eq!(Integer::from_sign_and_abs(false, Natural::from(123u32)), -123);
    /// ```
    pub fn from_sign_and_abs(sign: bool, abs: Natural) -> Integer {
        Integer {
            sign: sign || abs == 0,
            abs,
        }
    }

    /// Converts a sign and an `Natural` to an `Integer`, taking the `Natural` by reference. The
    /// `Natural` becomes the `Integer`'s absolute value, and the sign indicates whether the
    /// `Integer` should be non-negative. If the `Natural` is zero, then the `Integer` will be
    /// non-negative regardless of the sign.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `abs.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Integer::from_sign_and_abs_ref(true, &Natural::from(123u32)), 123);
    /// assert_eq!(Integer::from_sign_and_abs_ref(false, &Natural::from(123u32)), -123);
    /// ```
    pub fn from_sign_and_abs_ref(sign: bool, abs: &Natural) -> Integer {
        Integer {
            sign: sign || *abs == 0,
            abs: abs.clone(),
        }
    }
}

macro_rules! integer_zero {
    () => {
        Integer {
            sign: true,
            abs: natural_zero!(),
        }
    };
}

macro_rules! integer_one {
    () => {
        Integer {
            sign: true,
            abs: natural_one!(),
        }
    };
}

macro_rules! integer_two {
    () => {
        Integer {
            sign: true,
            abs: natural_two!(),
        }
    };
}

macro_rules! integer_negative_one {
    () => {
        Integer {
            sign: false,
            abs: natural_one!(),
        }
    };
}

/// The constant 0.
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl Zero for Integer {
    const ZERO: Integer = integer_zero!();
}

/// The constant 1.
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl One for Integer {
    const ONE: Integer = integer_one!();
}

/// The constant 2.
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl Two for Integer {
    const TWO: Integer = integer_two!();
}

/// The constant -1.
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl NegativeOne for Integer {
    const NEGATIVE_ONE: Integer = integer_negative_one!();
}

/// The constant -1.
///
/// # Worst-case complexity
/// Constant time and additional memory.
impl Default for Integer {
    /// The default value of an `Integer`, 0.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn default() -> Integer {
        Integer::ZERO
    }
}

// Implement `Named` for `Integer`.
impl_named!(Integer);

pub mod arithmetic {
    pub mod abs;
    pub mod add;
    pub mod add_mul;
    pub mod div;
    pub mod div_exact;
    pub mod div_mod;
    pub mod div_round;
    pub mod divisible_by;
    pub mod divisible_by_power_of_2;
    pub mod eq_mod;
    pub mod eq_mod_power_of_2;
    pub mod mod_op;
    pub mod mod_power_of_2;
    pub mod mul;
    pub mod neg;
    pub mod parity;
    pub mod pow;
    pub mod power_of_2;
    pub mod root;
    pub mod round_to_multiple;
    pub mod round_to_multiple_of_power_of_2;
    pub mod shl;
    pub mod shl_round;
    pub mod shr;
    pub mod shr_round;
    pub mod sign;
    pub mod sqrt;
    pub mod square;
    pub mod sub;
    pub mod sub_mul;
}
pub mod comparison;
pub mod conversion;
/// Iterators that generate `Integer`s without repetition.
pub mod exhaustive;
pub mod logic {
    pub mod and;
    pub mod bit_access;
    pub mod bit_block_access;
    pub mod bit_convertible;
    pub mod bit_iterable;
    pub mod bit_scan;
    pub mod checked_count_ones;
    pub mod checked_count_zeros;
    pub mod checked_hamming_distance;
    pub mod low_mask;
    pub mod not;
    pub mod or;
    pub mod significant_bits;
    pub mod trailing_zeros;
    pub mod xor;
}
/// Iterators that generate `Integer`s randomly.
pub mod random;
