use crate::generators::common::{integer_nrm, natural_nrm};
use crate::generators::exhaustive::*;
use crate::generators::random::*;
use crate::generators::special_random::*;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::SaturatingFrom;
use malachite_base_test_util::generators::common::Generator;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::{BigInt, BigUint};

// -- Integer --

pub fn integer_gen() -> Generator<Integer> {
    Generator::new(
        &exhaustive_integer_gen,
        &random_integer_gen,
        &special_random_integer_gen,
    )
}

pub fn integer_gen_nrm() -> Generator<(BigInt, rug::Integer, Integer)> {
    Generator::new(
        &|| integer_nrm(exhaustive_integer_gen()),
        &|config| integer_nrm(random_integer_gen(config)),
        &|config| integer_nrm(special_random_integer_gen(config)),
    )
}

// -- Natural --

pub fn natural_gen() -> Generator<Natural> {
    Generator::new(
        &exhaustive_natural_gen,
        &random_natural_gen,
        &special_random_natural_gen,
    )
}

pub fn natural_gen_nrm() -> Generator<(BigUint, rug::Integer, Natural)> {
    Generator::new(
        &|| natural_nrm(exhaustive_natural_gen()),
        &|config| natural_nrm(random_natural_gen(config)),
        &|config| natural_nrm(special_random_natural_gen(config)),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// All `(Vec<T>, u64>)` where `T` is unsigned, the `Vec` has at least two elements, and the `u64` is
// greater than 1 and exactly convertible to `T` and the unsigned type `U`.
pub fn unsigned_vec_unsigned_pair_gen_var_1<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
) -> Generator<(Vec<T>, u64)>
where
    u64: SaturatingFrom<T> + SaturatingFrom<U>,
{
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_pair_gen_var_1::<T, U>,
        &random_primitive_int_vec_unsigned_pair_gen_var_1::<T, U>,
        &special_random_unsigned_vec_unsigned_pair_gen_var_1::<T, U>,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>) --

// All `(Vec<u8>, u64, Vec<Limb>)` that are valid inputs to `_limbs_to_digits_small_base`.
pub fn unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1() -> Generator<(Vec<u8>, u64, Vec<Limb>)>
{
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
        &random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1,
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

// All `(Vec<u8>, usize, Vec<Limb>, u64)` that are valid inputs to
// `_limbs_to_digits_small_base_basecase`.
pub fn unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1(
) -> Generator<(Vec<u8>, usize, Vec<Limb>, u64)> {
    Generator::new(
        &exhaustive_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
        &random_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
        &special_random_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1,
    )
}

pub mod common;
pub mod exhaustive;
pub mod random;
pub mod special_random;
