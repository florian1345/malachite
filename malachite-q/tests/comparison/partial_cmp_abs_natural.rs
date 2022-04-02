use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_pair_gen;
use malachite_q::test_util::generators::{
    rational_natural_natural_triple_gen, rational_natural_pair_gen,
    rational_rational_natural_triple_gen,
};
use malachite_q::Rational;
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_rational_natural() {
    let test = |s, t, cmp, lt: bool, gt: bool, le: bool, ge: bool| {
        let u = Rational::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u.partial_cmp_abs(&v), cmp);
        assert_eq!(v.partial_cmp_abs(&u).map(Ordering::reverse), cmp);
        assert_eq!(lt, u.lt_abs(&v));
        assert_eq!(gt, u.gt_abs(&v));
        assert_eq!(le, u.le_abs(&v));
        assert_eq!(ge, u.ge_abs(&v));
        assert_eq!(lt, v.gt_abs(&u));
        assert_eq!(gt, v.lt_abs(&u));
        assert_eq!(le, v.ge_abs(&u));
        assert_eq!(ge, v.le_abs(&u));
    };
    test("0", "0", Some(Ordering::Equal), false, false, true, true);
    test("0", "5", Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        "123",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test("123", "124", Some(Ordering::Less), true, false, true, false);
    test(
        "123",
        "122",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "1000000000000",
        "123",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "123",
        "1000000000000",
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
    test(
        "1000000000000",
        "1000000000000",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        "1000000000000",
        Some(Ordering::Equal),
        false,
        false,
        true,
        true,
    );
    test(
        "-1000000000000",
        "0",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );

    test(
        "99/100",
        "1",
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
    test(
        "101/100",
        "1",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "22/7",
        "3",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test("22/7", "4", Some(Ordering::Less), true, false, true, false);
    test(
        "-99/100",
        "1",
        Some(Ordering::Less),
        true,
        false,
        true,
        false,
    );
    test(
        "-101/100",
        "1",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test(
        "-22/7",
        "3",
        Some(Ordering::Greater),
        false,
        true,
        false,
        true,
    );
    test("-22/7", "4", Some(Ordering::Less), true, false, true, false);
}

#[test]
fn partial_cmp_abs_natural_properties() {
    rational_natural_pair_gen().test_properties(|(x, y)| {
        let cmp = x.partial_cmp_abs(&y);
        assert_eq!(x.cmp_abs(&Rational::from(&y)), cmp.unwrap());
        assert_eq!(y.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
    });

    rational_rational_natural_triple_gen().test_properties(|(x, z, y)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });

    rational_natural_natural_triple_gen().test_properties(|(y, x, z)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x < z);
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x > z);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x).partial_cmp_abs(&y), Some(x.cmp(&y)));
        assert_eq!(x.partial_cmp_abs(&Rational::from(&y)), Some(x.cmp(&y)));
    });
}
