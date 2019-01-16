use common::test_properties;
use malachite_base::num::{NegativeOne, One, SubMul, SubMulAssign, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use std::str::FromStr;

#[test]
fn test_sub_mul() {
    let test = |i, j, k, out| {
        let mut a = Integer::from_str(i).unwrap();
        a.sub_mul_assign(Integer::from_str(j).unwrap(), Integer::from_str(k).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(i).unwrap();
        a.sub_mul_assign(
            Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(i).unwrap();
        a.sub_mul_assign(
            &Integer::from_str(j).unwrap(),
            Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let mut a = Integer::from_str(i).unwrap();
        a.sub_mul_assign(
            &Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(i)
            .unwrap()
            .sub_mul(Integer::from_str(j).unwrap(), Integer::from_str(k).unwrap());
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(i).unwrap().sub_mul(
            Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(i).unwrap().sub_mul(
            &Integer::from_str(j).unwrap(),
            Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = Integer::from_str(i).unwrap().sub_mul(
            &Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());

        let a = (&Integer::from_str(i).unwrap()).sub_mul(
            &Integer::from_str(j).unwrap(),
            &Integer::from_str(k).unwrap(),
        );
        assert_eq!(a.to_string(), out);
        assert!(a.is_valid());
    };
    test("0", "0", "0", "0");
    test("0", "0", "123", "0");
    test("123", "0", "5", "123");
    test("123", "-5", "1", "128");
    test("123", "-5", "100", "623");
    test("10", "-3", "4", "22");
    test("1000000000000", "0", "123", "1000000000000");
    test("1000000000000", "-1", "123", "1000000000123");
    test("1000000000000", "-123", "1", "1000000000123");
    test("1000000000000", "-123", "100", "1000000012300");
    test("1000000000000", "-100", "123", "1000000012300");
    test("1000000000000", "-65536", "65536", "1004294967296");
    test("1000000000000", "-1000000000000", "0", "1000000000000");
    test("1000000000000", "-1000000000000", "1", "2000000000000");
    test("1000000000000", "-1000000000000", "100", "101000000000000");
    test("0", "-1000000000000", "100", "100000000000000");
    test(
        "1000000000000",
        "-65536",
        "1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "-1000000000000",
        "1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "-1000000000000",
        "1000000000000",
        "1000000000000000000000000",
    );

    test("123", "5", "-1", "128");
    test("123", "5", "-100", "623");
    test("10", "3", "-4", "22");
    test("1000000000000", "1", "-123", "1000000000123");
    test("1000000000000", "123", "-1", "1000000000123");
    test("1000000000000", "123", "-100", "1000000012300");
    test("1000000000000", "100", "-123", "1000000012300");
    test("1000000000000", "65536", "-65536", "1004294967296");
    test("1000000000000", "1000000000000", "-1", "2000000000000");
    test("1000000000000", "1000000000000", "-100", "101000000000000");
    test("0", "1000000000000", "-100", "100000000000000");
    test(
        "1000000000000",
        "65536",
        "-1000000000000",
        "65537000000000000",
    );
    test(
        "1000000000000",
        "1000000000000",
        "-1000000000000",
        "1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "-1000000000000",
        "1000000000000000000000000",
    );

    test("0", "0", "-123", "0");
    test("123", "0", "-5", "123");
    test("123", "5", "1", "118");
    test("123", "-5", "-1", "118");
    test("123", "5", "100", "-377");
    test("123", "-5", "-100", "-377");
    test("10", "3", "4", "-2");
    test("10", "-3", "-4", "-2");
    test("15", "3", "4", "3");
    test("15", "-3", "-4", "3");
    test("1000000000000", "0", "-123", "1000000000000");
    test("1000000000000", "1", "123", "999999999877");
    test("1000000000000", "-1", "-123", "999999999877");
    test("1000000000000", "123", "1", "999999999877");
    test("1000000000000", "-123", "-1", "999999999877");
    test("1000000000000", "123", "100", "999999987700");
    test("1000000000000", "-123", "-100", "999999987700");
    test("1000000000000", "100", "123", "999999987700");
    test("1000000000000", "-100", "-123", "999999987700");
    test("1000000000000", "65536", "65536", "995705032704");
    test("1000000000000", "-65536", "-65536", "995705032704");
    test("1000000000000", "1000000000000", "0", "1000000000000");
    test("1000000000000", "1000000000000", "1", "0");
    test("1000000000000", "-1000000000000", "-1", "0");
    test("1000000000000", "1000000000000", "100", "-99000000000000");
    test("1000000000000", "-1000000000000", "-100", "-99000000000000");
    test("0", "1000000000000", "100", "-100000000000000");
    test("4294967296", "1", "1", "4294967295");
    test("4294967296", "-1", "-1", "4294967295");
    test("3902609153", "88817093856604", "1", "-88813191247451");
    test("3902609153", "-88817093856604", "-1", "-88813191247451");

    test("-123", "0", "5", "-123");
    test("-123", "5", "1", "-128");
    test("-123", "5", "100", "-623");
    test("-10", "3", "4", "-22");
    test("-1000000000000", "0", "123", "-1000000000000");
    test("-1000000000000", "1", "123", "-1000000000123");
    test("-1000000000000", "123", "1", "-1000000000123");
    test("-1000000000000", "123", "100", "-1000000012300");
    test("-1000000000000", "100", "123", "-1000000012300");
    test("-1000000000000", "65536", "65536", "-1004294967296");
    test("-1000000000000", "1000000000000", "0", "-1000000000000");
    test("-1000000000000", "1000000000000", "1", "-2000000000000");
    test("-1000000000000", "1000000000000", "100", "-101000000000000");
    test(
        "-1000000000000",
        "65536",
        "1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "1000000000000",
        "1000000000000",
        "-1000000000001000000000000",
    );
    test(
        "0",
        "1000000000000",
        "1000000000000",
        "-1000000000000000000000000",
    );

    test("-123", "-5", "-1", "-128");
    test("-123", "-5", "-100", "-623");
    test("-10", "-3", "-4", "-22");
    test("-1000000000000", "-1", "-123", "-1000000000123");
    test("-1000000000000", "-123", "-1", "-1000000000123");
    test("-1000000000000", "-123", "-100", "-1000000012300");
    test("-1000000000000", "-100", "-123", "-1000000012300");
    test("-1000000000000", "-65536", "-65536", "-1004294967296");
    test("-1000000000000", "-1000000000000", "-1", "-2000000000000");
    test(
        "-1000000000000",
        "-1000000000000",
        "-100",
        "-101000000000000",
    );
    test(
        "-1000000000000",
        "-65536",
        "-1000000000000",
        "-65537000000000000",
    );
    test(
        "-1000000000000",
        "-1000000000000",
        "-1000000000000",
        "-1000000000001000000000000",
    );

    test("-123", "0", "-5", "-123");
    test("-123", "-5", "1", "-118");
    test("-123", "5", "-1", "-118");
    test("-123", "-5", "100", "377");
    test("-123", "5", "-100", "377");
    test("-10", "-3", "4", "2");
    test("-10", "3", "-4", "2");
    test("-15", "-3", "4", "-3");
    test("-15", "3", "-4", "-3");
    test("-1000000000000", "0", "-123", "-1000000000000");
    test("-1000000000000", "-1", "123", "-999999999877");
    test("-1000000000000", "1", "-123", "-999999999877");
    test("-1000000000000", "-123", "1", "-999999999877");
    test("-1000000000000", "123", "-1", "-999999999877");
    test("-1000000000000", "-123", "100", "-999999987700");
    test("-1000000000000", "123", "-100", "-999999987700");
    test("-1000000000000", "-100", "123", "-999999987700");
    test("-1000000000000", "100", "-123", "-999999987700");
    test("-1000000000000", "-65536", "65536", "-995705032704");
    test("-1000000000000", "65536", "-65536", "-995705032704");
    test("-1000000000000", "-1000000000000", "0", "-1000000000000");
    test("-1000000000000", "-1000000000000", "1", "0");
    test("-1000000000000", "1000000000000", "-1", "0");
    test("-1000000000000", "-1000000000000", "100", "99000000000000");
    test("-1000000000000", "1000000000000", "-100", "99000000000000");
    test("-4294967296", "-1", "1", "-4294967295");
    test("-4294967296", "1", "-1", "-4294967295");
    test("-3902609153", "-88817093856604", "1", "88813191247451");
    test("-3902609153", "88817093856604", "-1", "88813191247451");
    test(
        "1000000000000000000000000",
        "1000000000000",
        "1000000000000",
        "0",
    );
}

#[test]
fn sub_mul_properties() {
    test_properties(triples_of_integers, |&(ref a, ref b, ref c)| {
        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c.clone());
        assert!(mut_a.is_valid());
        let result = mut_a;

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b.clone(), c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b, c.clone());
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let mut mut_a = a.clone();
        mut_a.sub_mul_assign(b, c);
        assert!(mut_a.is_valid());
        assert_eq!(mut_a, result);

        let result_alt = a.clone().sub_mul(b.clone(), c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(b.clone(), c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(b, c.clone());
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.clone().sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        let result = a.sub_mul(b, c);
        assert!(result_alt.is_valid());
        assert_eq!(result_alt, result);

        assert_eq!(a - b * c, result);
        assert_eq!(a.sub_mul(c, b), result);
        assert_eq!(a.sub_mul(&(-b), &(-c)), result);
        assert_eq!((-a).sub_mul(&(-b), c), -&result);
        assert_eq!((-a).sub_mul(b, -c), -result);
    });

    test_properties(integers, |a| {
        assert_eq!(a.sub_mul(a, &Integer::ONE), 0 as Limb);
        assert_eq!(a.sub_mul(&(-a), &Integer::NEGATIVE_ONE), 0 as Limb);
    });

    test_properties(pairs_of_integers, |&(ref a, ref b)| {
        assert_eq!(a.sub_mul(&Integer::ZERO, b), *a);
        assert_eq!(a.sub_mul(&Integer::ONE, b), a - b);
        assert_eq!(Integer::ZERO.sub_mul(a, b), -a * b);
        assert_eq!(a.sub_mul(b, &Integer::ZERO), *a);
        assert_eq!(a.sub_mul(b, &Integer::ONE), a - b);
        assert_eq!((a * b).sub_mul(a, b), 0 as Limb);
        assert_eq!((a * b).sub_mul(-a, -b), 0 as Limb);
    });
}
