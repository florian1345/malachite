use malachite_base::num::arithmetic::traits::{SaturatingSub, SaturatingSubAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_pair_gen_var_27;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen};
use std::str::FromStr;

#[test]
fn test_saturating_sub_natural() {
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_assign(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n.saturating_sub_assign(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u)
            .unwrap()
            .saturating_sub(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).saturating_sub(Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&Natural::from_str(u).unwrap()).saturating_sub(&Natural::from_str(v).unwrap());
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("123", "1000000000000", "0");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("4294967295", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
    test("18446744073709551615", "18446744073709551616", "0");
    test("70734740290631708", "282942734368", "70734457347897340");
    test("282942734368", "70734740290631708", "0");
}

#[test]
fn saturating_sub_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let mut mut_x = x.clone();
        mut_x.saturating_sub_assign(&y);
        assert!(mut_x.is_valid());
        let diff = mut_x;

        let mut mut_x = x.clone();
        mut_x.saturating_sub_assign(y.clone());
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, diff);

        let diff_alt = x.clone().saturating_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = x.clone().saturating_sub(&y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = (&x).saturating_sub(y.clone());
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = (&x).saturating_sub(&y);
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let reverse_diff = (&y).saturating_sub(&x);
        if diff > 0 {
            assert_eq!(reverse_diff, 0);
        }
        if reverse_diff > 0 {
            assert_eq!(diff, 0);
        }

        assert!(diff <= x);
        assert!(diff + y >= x);
    });

    natural_gen().test_properties(|x| {
        assert_eq!((&x).saturating_sub(Natural::ZERO), x);
        assert_eq!((&x).saturating_sub(&x), 0);
        assert_eq!(Natural::ZERO.saturating_sub(x), 0);
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(
            x.saturating_sub(y),
            Natural::from(x).saturating_sub(Natural::from(y))
        );
    });
}
