use malachite_base::traits::{NegativeOne, One, Two, Zero};
use natural::Natural::{self, Small};

/// An integer.
///
/// Any `Integer` whose absolute value is small enough to fit into an `u32` is represented inline.
/// Only integers outside this range incur the costs of heap-allocation.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Integer {
    pub(crate) sign: bool, // must be true if abs is zero
    pub(crate) abs: Natural,
}

impl Integer {
    /// Returns true iff `self` is valid. To be valid, can only be Large when its absolute value
    /// is at least 2^(31). All Integers used outside this crate are valid, but temporary Integers
    /// used inside may not be.
    pub fn is_valid(&self) -> bool {
        self.abs.is_valid() && (self.sign || self.abs != 0)
    }
}

/// Creates a default `Integer` equal to 0.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert_eq!(Integer::default().to_string(), "0");
/// ```
impl Default for Integer {
    fn default() -> Integer {
        Integer {
            sign: true,
            abs: Natural::zero(),
        }
    }
}

/// The constant 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
impl Zero for Integer {
    fn zero() -> Integer {
        Integer {
            sign: true,
            abs: Small(0),
        }
    }
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
impl One for Integer {
    fn one() -> Integer {
        Integer {
            sign: true,
            abs: Small(1),
        }
    }
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
impl Two for Integer {
    fn two() -> Integer {
        Integer {
            sign: true,
            abs: Small(2),
        }
    }
}

/// The constant -1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
impl NegativeOne for Integer {
    fn negative_one() -> Integer {
        Integer {
            sign: false,
            abs: Small(1),
        }
    }
}

pub mod arithmetic {
    pub mod abs;
    pub mod add;
    pub mod add_i32;
    pub mod add_u32;
    pub mod add_mul;
    pub mod add_mul_i32;
    pub mod add_mul_u32;
    pub mod divisible_by_power_of_2;
    pub mod even_odd;
    pub mod mul;
    pub mod mul_i32;
    pub mod mul_u32;
    pub mod neg;
    pub mod shl_u32;
    pub mod sub;
    pub mod sub_i32;
    pub mod sub_u32;
    pub mod sub_mul;
    pub mod sub_mul_i32;
    pub mod sub_mul_u32;
}
pub mod comparison {
    pub mod ord;
    pub mod ord_abs;
    pub mod partial_eq_i32;
    pub mod partial_eq_natural;
    pub mod partial_eq_u32;
    pub mod partial_ord_abs_i32;
    pub mod partial_ord_abs_natural;
    pub mod partial_ord_abs_u32;
    pub mod partial_ord_i32;
    pub mod partial_ord_natural;
    pub mod partial_ord_u32;
    pub mod sign;
}
pub mod conversion;
pub mod logic;
