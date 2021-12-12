use malachite_base::num::basic::traits::Zero;
use malachite_base_test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{SignedDoubleLimb, SignedLimb};
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_nz_test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use num::BigInt;
use std::str::FromStr;

#[test]
fn test_add() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n += v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n += &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigInt::from_str(s).unwrap() + BigInt::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() + rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
    test("0", "-123", "-123");
    test("123", "-123", "0");
    test("123", "-456", "-333");
    test("1000000000000", "-123", "999999999877");
    test("123", "-1000000000000", "-999999999877");
    test("12345678987654321", "-314159265358979", "12031519722295342");
}

#[test]
fn add_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + &y;
        let sum_ref_val = &x + y.clone();
        let sum = &x + &y;
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += &y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(&x);
        mut_x += integer_to_rug_integer(&y);
        assert_eq!(rug_integer_to_integer(&mut_x), sum);

        assert_eq!(
            bigint_to_integer(&(integer_to_bigint(&x) + integer_to_bigint(&y))),
            sum
        );
        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(&x) + integer_to_rug_integer(&y))),
            sum
        );
        assert_eq!(&y + &x, sum);
        assert_eq!(&sum - &x, y);
        assert_eq!(sum - y, x);
    });

    integer_gen().test_properties(|ref x| {
        assert_eq!(x + Integer::ZERO, *x);
        assert_eq!(Integer::ZERO + x, *x);
        assert_eq!(x + x, x << 1);
        assert_eq!(x + (-x), 0)
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x + &y) + &z, x + (y + z));
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(&x + &y, Integer::from(x) + Integer::from(y));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(
            Integer::from(SignedDoubleLimb::from(x) + SignedDoubleLimb::from(y)),
            Integer::from(x) + Integer::from(y)
        );
    });
}
