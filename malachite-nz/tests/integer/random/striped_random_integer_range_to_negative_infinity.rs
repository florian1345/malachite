use itertools::Itertools;
use malachite_base::num::float::NiceFloat;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::{moment_stats, MomentStats};
use malachite_nz::integer::random::striped_random_integer_range_to_negative_infinity;
use malachite_nz::integer::Integer;
use std::str::FromStr;

fn striped_random_integer_range_to_negative_infinity_helper(
    a: &str,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_bits_numerator: u64,
    mean_bits_denominator: u64,
    expected_values: &[&str],
    expected_common_values: &[(&str, usize)],
    expected_sample_median: (&str, Option<&str>),
    expected_sample_moment_stats: MomentStats,
) {
    let a = Integer::from_str(a).unwrap();
    let xs = striped_random_integer_range_to_negative_infinity(
        EXAMPLE_SEED,
        a,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_bits_numerator,
        mean_bits_denominator,
    );
    let actual_values = xs
        .clone()
        .map(|x| Integer::to_string(&x))
        .take(20)
        .collect_vec();
    let actual_values = actual_values.iter().map(String::as_str).collect_vec();
    let actual_common_values = common_values_map(1000000, 10, xs.clone())
        .into_iter()
        .map(|(x, freq)| (x.to_string(), freq))
        .collect_vec();
    let actual_common_values = actual_common_values
        .iter()
        .map(|(x, freq)| (x.as_str(), *freq))
        .collect_vec();
    let (median_lo, median_hi) = median(xs.clone().take(1000000));
    let (median_lo, median_hi) = (
        median_lo.to_string(),
        median_hi.map(|x| Integer::to_string(&x)),
    );
    let actual_sample_median = (median_lo.as_str(), median_hi.as_deref());
    let actual_sample_moment_stats = moment_stats(xs.take(1000000).map(|x| f64::from(&x)));
    assert_eq!(
        (
            actual_values.as_slice(),
            actual_common_values.as_slice(),
            actual_sample_median,
            actual_sample_moment_stats
        ),
        (
            expected_values,
            expected_common_values,
            expected_sample_median,
            expected_sample_moment_stats
        )
    );
}

#[test]
fn test_striped_random_integer_range_to_negative_infinity() {
    let values = &[
        "-2",
        "-4",
        "-128",
        "-1124203576",
        "-4",
        "-15",
        "-32",
        "-751",
        "-6400",
        "-8376024595",
        "-3",
        "-60",
        "-1",
        "-1",
        "-65045535",
        "-6",
        "0",
        "-7",
        "-73608",
        "-719661083353407616",
    ];
    let common_values = &[
        ("0", 90859),
        ("-1", 82901),
        ("-3", 37653),
        ("-2", 37438),
        ("-7", 25786),
        ("-4", 25681),
        ("-8", 17394),
        ("-15", 17328),
        ("-16", 12055),
        ("-31", 11982),
    ];
    let sample_median = ("-71", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-8.068551928510147e34),
        standard_deviation: NiceFloat(6.914607365781463e37),
        skewness: NiceFloat(-958.8924868378492),
        excess_kurtosis: NiceFloat(939262.8054862365),
    };
    striped_random_integer_range_to_negative_infinity_helper(
        "0",
        4,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "4",
        "2",
        "-1024",
        "-270182996582056056",
        "-263128",
        "798",
        "-15",
        "-16744652",
        "479",
        "68",
        "11",
        "-1072708599",
        "-10",
        "0",
        "240",
        "0",
        "-34521100",
        "-4385034796639",
        "-60",
        "-7",
    ];
    let common_values = &[
        ("0", 58400),
        ("1", 53191),
        ("-1", 53081),
        ("-3", 24269),
        ("3", 24150),
        ("2", 24142),
        ("-2", 23864),
        ("7", 16659),
        ("4", 16517),
        ("-4", 16397),
    ];
    let sample_median = ("-3", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-3.9240865426395e50),
        standard_deviation: NiceFloat(3.92408654263964e53),
        skewness: NiceFloat(-999.9984999993759),
        excess_kurtosis: NiceFloat(999995.0000009785),
    };
    striped_random_integer_range_to_negative_infinity_helper(
        "1000",
        4,
        1,
        10,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );

    let values = &[
        "-8192", "-8312", "-15614", "-1984", "-1568", "-1009", "-1791", "-260174", "-9855",
        "-98176", "-1519", "-2591", "-3616", "-8176", "-1796", "-8167", "-262616", "-4069",
        "-12062", "-1072",
    ];
    let common_values = &[
        ("-1023", 31892),
        ("-1007", 28274),
        ("-1000", 28157),
        ("-1008", 10740),
        ("-1022", 10689),
        ("-1016", 10551),
        ("-1020", 10510),
        ("-1001", 9498),
        ("-1004", 9330),
        ("-1006", 9302),
    ];
    let sample_median = ("-4159", None);
    let sample_moment_stats = MomentStats {
        mean: NiceFloat(-54201665904379.41),
        standard_deviation: NiceFloat(2.6365321443458296e16),
        skewness: NiceFloat(-608.9470987335318),
        excess_kurtosis: NiceFloat(388228.1677811064),
    };
    striped_random_integer_range_to_negative_infinity_helper(
        "-1000",
        4,
        1,
        14,
        1,
        values,
        common_values,
        sample_median,
        sample_moment_stats,
    );
}

#[test]
#[should_panic]
fn striped_random_integer_range_to_negative_infinity_fail_1() {
    striped_random_integer_range_to_negative_infinity(
        EXAMPLE_SEED,
        Integer::from(100),
        1,
        0,
        10,
        1,
    );
}

#[test]
#[should_panic]
fn striped_random_integer_range_to_negative_infinity_fail_2() {
    striped_random_integer_range_to_negative_infinity(
        EXAMPLE_SEED,
        Integer::from(100),
        1,
        1,
        10,
        1,
    );
}

#[test]
#[should_panic]
fn striped_random_integer_range_to_negative_infinity_fail_3() {
    striped_random_integer_range_to_negative_infinity(EXAMPLE_SEED, Integer::from(100), 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_integer_range_to_negative_infinity_fail_4() {
    striped_random_integer_range_to_negative_infinity(
        EXAMPLE_SEED,
        Integer::from(-100),
        4,
        1,
        2,
        3,
    );
}
