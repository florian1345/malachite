use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::test_util::generators::integer_pair_gen;
use malachite_q::test_util::arithmetic::mul::mul_naive;
use malachite_q::test_util::common::{
    bigrational_to_rational, rational_to_bigrational, rational_to_rug_rational,
    rug_rational_to_rational,
};
use malachite_q::test_util::generators::{rational_gen, rational_pair_gen, rational_triple_gen};
use malachite_q::Rational;
use num::BigRational;
use std::str::FromStr;

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let u = Rational::from_str(s).unwrap();
        let v = Rational::from_str(t).unwrap();

        let mut n = u.clone();
        n *= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n *= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() * v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u * v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() * &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u * &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigRational::from_str(s).unwrap() * BigRational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Rational::from_str(s).unwrap() * rug::Rational::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("0", "-123", "0");
    test("123", "0", "0");
    test("-123", "0", "0");
    test("1", "123", "123");
    test("1", "-123", "-123");
    test("-1", "123", "-123");
    test("-1", "-123", "123");
    test("123", "1", "123");
    test("123", "-1", "-123");
    test("-123", "1", "-123");
    test("-123", "-1", "123");
    test("123", "456", "56088");
    test("123", "-456", "-56088");
    test("-123", "456", "-56088");
    test("-123", "-456", "56088");
    test("22/7", "3/2", "33/7");
    test("22/7", "-3/2", "-33/7");
    test("-22/7", "3/2", "-33/7");
    test("-22/7", "-3/2", "33/7");
    test("4/5", "5/4", "1");
    test("4/5", "-5/4", "-1");
    test("-4/5", "5/4", "-1");
    test("-4/5", "-5/4", "1");
}

#[test]
fn mul_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * &y;
        let product_ref_val = &x * y.clone();
        let product = &x * &y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);
        let mut mut_x = x.clone();
        mut_x *= &y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        let mut mut_x = rational_to_rug_rational(&x);
        mut_x *= rational_to_rug_rational(&y);
        assert_eq!(rug_rational_to_rational(&mut_x), product);

        assert_eq!(
            bigrational_to_rational(&(rational_to_bigrational(&x) * rational_to_bigrational(&y))),
            product
        );
        assert_eq!(
            rug_rational_to_rational(
                &(rational_to_rug_rational(&x) * rational_to_rug_rational(&y))
            ),
            product
        );
        assert_eq!(mul_naive(x.clone(), y.clone()), product);
        assert_eq!(&y * &x, product);
        if x != 0u32 {
            assert_eq!(&product / &x, y);
        }
        if y != 0u32 {
            assert_eq!(&product / &y, x);
            assert_eq!(&x / (&y).reciprocal(), product);
        }
        if product != 0u32 {
            assert_eq!(
                (&x).reciprocal() * (&y).reciprocal(),
                (&product).reciprocal()
            )
        }
        assert_eq!(-&x * &y, -&product);
        assert_eq!(x * -y, -product);
    });

    rational_gen().test_properties(|ref x| {
        assert_eq!(x * Rational::ZERO, 0);
        assert_eq!(Rational::ZERO * x, 0);
        assert_eq!(x * Rational::ONE, *x);
        assert_eq!(Rational::ONE * x, *x);
        assert_eq!(x * Rational::NEGATIVE_ONE, -x);
        assert_eq!(Rational::NEGATIVE_ONE * x, -x);
        //TODO assert_eq!(x * x, x.square());
    });

    rational_triple_gen().test_properties(|(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(&x * &y, Rational::from(x) * Rational::from(y));
    });
}
