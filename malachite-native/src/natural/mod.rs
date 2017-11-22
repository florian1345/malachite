use malachite_base::traits::{One, Two, Zero};
use natural::Natural::*;

pub const LOG_LIMB_BITS: u32 = 5;
pub const LIMB_BITS: u32 = 1 << LOG_LIMB_BITS;
pub const LIMB_BITS_MASK: u32 = LIMB_BITS - 1;

/// A natural (non-negative) integer.
///
/// Any `Natural` small enough to fit into an `u32` is represented inline. Only naturals outside
/// this range incur the costs of heap-allocation.
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Natural {
    Small(u32),
    Large(Vec<u32>),
}

// Test s and return whether the operand is zero.
pub fn mpn_zero_p(s: &[u32]) -> bool {
    s.iter().all(|&x| x == 0)
}

// Zero r.
pub fn mpn_zero(r: &mut [u32]) {
    for x in r.iter_mut() {
        *x = 0;
    }
}

impl Natural {
    fn demote_if_small(&mut self) {
        let mut demoted_value = None;
        if let Large(ref limbs) = *self {
            let limb_count = limbs.len();
            match limb_count {
                0 => demoted_value = Some(0),
                1 => demoted_value = Some(limbs[0]),
                _ => {}
            }
        }
        if let Some(small) = demoted_value {
            *self = Small(small);
        }
    }

    pub(crate) fn promote_in_place(&mut self) -> &mut Vec<u32> {
        if let Small(x) = *self {
            let xs = vec![x];
            *self = Large(xs);
        }
        if let Large(ref mut xs) = *self {
            xs
        } else {
            unreachable!();
        }
    }

    pub(crate) fn trim(&mut self) {
        if let Large(ref mut xs) = *self {
            while !xs.is_empty() && xs.last().unwrap() == &0 {
                xs.pop();
            }
        }
        self.demote_if_small();
    }

    /// Returns true iff `self` is valid. To be valid, `self` can only be Large when it is at least
    /// 2^(32), and cannot have leading zero limbs. All Naturals used outside this crate are valid,
    /// but temporary Naturals used inside may not be.
    pub fn is_valid(&self) -> bool {
        match *self {
            Small(_) => true,
            Large(ref xs) => xs.len() > 1 && xs.last().unwrap() != &0,
        }
    }
}

/// Creates a default `Natural` equal to 0.
///
/// # Example
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!(Natural::default().to_string(), "0");
/// ```
impl Default for Natural {
    fn default() -> Natural {
        Small(0)
    }
}

/// The constant 0.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
impl Zero for Natural {
    fn zero() -> Natural {
        Small(0)
    }
}

/// The constant 1.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
impl One for Natural {
    fn one() -> Natural {
        Small(1)
    }
}

/// The constant 2.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
impl Two for Natural {
    fn two() -> Natural {
        Small(2)
    }
}

fn pad_left<T: Clone>(vec: &mut Vec<T>, pad_size: usize, value: T) {
    let old_len = vec.len();
    vec.resize(old_len + pad_size, value);
    for i in (0..old_len).rev() {
        vec.swap(i, i + pad_size);
    }
}

macro_rules! mutate_with_possible_promotion {
    ($n: ident, $small: ident, $large: ident, $process_small: expr, $process_large: expr) => {
        if let Small(ref mut $small) = *$n {
            if let Some(small_result) = $process_small {
                *$small = small_result;
                return;
            }
        }
        if let Small(x) = *$n {
            *$n = Large(vec![x]);
        }
        if let Large(ref mut $large) = *$n {
            $process_large
        }
    };
}

pub mod arithmetic;
pub mod conversion;
pub mod comparison {
    pub mod ord;
    pub mod partial_eq_u32;
    pub mod partial_ord_u32;
}
pub mod logic {
    pub mod assign_bit;
    pub mod clear_bit;
    pub mod flip_bit;
    pub mod from_limbs;
    pub mod get_bit;
    pub mod limb_count;
    pub mod limbs;
    pub mod not;
    pub mod set_bit;
    pub mod significant_bits;
    pub mod trailing_zeros;
}
