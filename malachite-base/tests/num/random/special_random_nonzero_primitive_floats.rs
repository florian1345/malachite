use itertools::Itertools;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::float::NiceFloat;
use malachite_base::num::random::special_random_nonzero_primitive_floats;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::test_util::stats::common_values_map::common_values_map;
use malachite_base::test_util::stats::median;
use malachite_base::test_util::stats::moments::CheckedToF64;
use std::panic::catch_unwind;

fn special_random_nonzero_primitive_floats_helper<T: CheckedToF64 + PrimitiveFloat>(
    mean_exponent_numerator: u64,
    mean_exponent_denominator: u64,
    mean_precision_numerator: u64,
    mean_precision_denominator: u64,
    mean_special_p_numerator: u64,
    mean_special_p_denominator: u64,
    expected_values: &[T],
    expected_common_values: &[(T, usize)],
    expected_median: (T, Option<T>),
) {
    let xs = special_random_nonzero_primitive_floats::<T>(
        EXAMPLE_SEED,
        mean_exponent_numerator,
        mean_exponent_denominator,
        mean_precision_numerator,
        mean_precision_denominator,
        mean_special_p_numerator,
        mean_special_p_denominator,
    );
    let actual_values = xs.clone().take(50).map(NiceFloat).collect_vec();
    let actual_common_values = common_values_map(1000000, 20, xs.clone().map(NiceFloat));
    let actual_median = median(xs.map(NiceFloat).take(1000000));
    let (lo, hi) = expected_median;
    assert_eq!(
        (
            actual_values,
            actual_common_values.as_slice(),
            actual_median,
        ),
        (
            expected_values.iter().cloned().map(NiceFloat).collect_vec(),
            expected_common_values
                .iter()
                .map(|&(x, freq)| (NiceFloat(x), freq))
                .collect_vec()
                .as_slice(),
            (NiceFloat(lo), hi.map(NiceFloat)),
        )
    );
}

#[test]
fn test_special_random_nonzero_primitive_floats() {
    // f32, mean abs of exponent = 1/64, mean precision = 65/64, mean special P = 1/4
    let values = &[
        f32::POSITIVE_INFINITY,
        1.0,
        1.0,
        f32::NEGATIVE_INFINITY,
        1.0,
        -1.0,
        f32::POSITIVE_INFINITY,
        -1.0,
        f32::POSITIVE_INFINITY,
        f32::NEGATIVE_INFINITY,
        -1.0,
        f32::NEGATIVE_INFINITY,
        -1.0,
        1.0,
        1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        1.0,
        -1.0,
        -1.0,
        1.0,
        0.5,
        1.0,
        -1.0,
        1.0,
        -1.0,
        1.0,
        1.0,
        f32::POSITIVE_INFINITY,
        -1.5,
        f32::NEGATIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        -1.0,
        1.0,
        -1.0,
        -1.0,
        -1.0,
        f32::POSITIVE_INFINITY,
        1.0,
        -1.0,
        -0.5,
        -1.0,
        f32::NEGATIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        1.0,
    ];
    let common_values = &[
        (1.0, 358244),
        (-1.0, 357926),
        (f32::POSITIVE_INFINITY, 125637),
        (f32::NEGATIVE_INFINITY, 124572),
        (2.0, 5538),
        (1.5, 5500),
        (0.5, 5497),
        (-1.5, 5454),
        (-2.0, 5379),
        (-0.5, 5357),
        (0.75, 102),
        (3.0, 98),
        (-4.0, 95),
        (-0.25, 91),
        (-0.75, 87),
        (-3.0, 86),
        (0.25, 79),
        (4.0, 75),
        (-1.25, 48),
        (1.75, 44),
    ];
    let sample_median = (0.5, None);
    special_random_nonzero_primitive_floats_helper::<f32>(
        1,
        64,
        65,
        64,
        1,
        4,
        values,
        common_values,
        sample_median,
    );

    // f32, mean abs of exponent = 1, mean precision = 2, mean special P = 1/10
    let values = &[
        1.0,
        1.25,
        3.0,
        f32::POSITIVE_INFINITY,
        -1.0,
        -1.0,
        -2.0,
        -3.5,
        1.0,
        2.0,
        -1.5,
        -2.5,
        -2.0,
        f32::NEGATIVE_INFINITY,
        -6.5,
        -1.0,
        -1.0,
        f32::POSITIVE_INFINITY,
        3.0,
        -0.21875,
        -1.0,
        0.25,
        1.5,
        5.25,
        -4.0,
        7.0,
        -0.5,
        0.1875,
        1.25,
        f32::POSITIVE_INFINITY,
        -0.1875,
        -7.5,
        f32::NEGATIVE_INFINITY,
        0.75,
        -7.0,
        -6.0,
        -3.0,
        0.234375,
        -2.0,
        -0.875,
        -0.75,
        6.0,
        -24.0,
        24.0,
        -2.0,
        1.5,
        f32::NEGATIVE_INFINITY,
        -1.25,
        14.0,
        5.0,
    ];
    let common_values = &[
        (1.0, 74789),
        (-1.0, 74702),
        (f32::POSITIVE_INFINITY, 50351),
        (f32::NEGATIVE_INFINITY, 49873),
        (1.5, 38119),
        (-0.5, 37713),
        (2.0, 37640),
        (-1.5, 37613),
        (-2.0, 37333),
        (0.5, 37027),
        (0.75, 19050),
        (4.0, 18892),
        (0.25, 18875),
        (-3.0, 18866),
        (3.0, 18821),
        (-0.75, 18725),
        (-4.0, 18663),
        (-0.25, 18537),
        (0.125, 9445),
        (-0.375, 9395),
    ];
    let sample_median = (0.0048828125, None);
    special_random_nonzero_primitive_floats_helper::<f32>(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        common_values,
        sample_median,
    );

    // f32, mean abs of exponent = 10, mean precision = 10, mean special P = 1/100
    let values = &[
        0.65625,
        0.0000014255784,
        0.013183594,
        -0.8125,
        -74240.0,
        -0.0078125,
        -0.03060913,
        3.331552,
        4.75,
        -0.000038146973,
        -0.3125,
        -27136.0,
        -59392.0,
        -1.75,
        -41.1875,
        0.30940247,
        -0.0009765625,
        -1536.0,
        0.2109375,
        0.0014648438,
        2.1129381e-8,
        -0.037109375,
        5242880.0,
        -0.21386719,
        134.21094,
        4.184082,
        -1561370.0,
        -2.1420419e-7,
        0.38085938,
        -0.007003784,
        -37748736.0,
        -6448.0,
        28.25,
        -6.703125,
        -4.483364,
        -3.1757812,
        0.000003915804,
        -0.020751953,
        0.00011110306,
        -0.000053405256,
        0.00019985437,
        -35.40625,
        0.005859375,
        0.0078125,
        28.25,
        30.0,
        -0.20776367,
        -144.0,
        -0.109375,
        -6144.0,
    ];
    let common_values = &[
        (f32::POSITIVE_INFINITY, 5098),
        (f32::NEGATIVE_INFINITY, 4891),
        (1.0, 2559),
        (-1.0, 2528),
        (0.5, 2362),
        (-2.0, 2312),
        (-1.5, 2306),
        (2.0, 2304),
        (1.5, 2275),
        (-0.5, 2243),
        (-3.0, 2204),
        (-4.0, 2163),
        (-0.25, 2129),
        (0.75, 2103),
        (3.0, 2081),
        (0.25, 2070),
        (-0.75, 2047),
        (4.0, 2038),
        (-6.0, 1943),
        (-8.0, 1918),
    ];
    let sample_median = (2.1684043e-19, None);
    special_random_nonzero_primitive_floats_helper::<f32>(
        10,
        1,
        10,
        1,
        1,
        100,
        values,
        common_values,
        sample_median,
    );

    // f64, mean abs of exponent = 1/64, mean precision = 65/64, mean special P = 1/4
    let values = &[
        f64::POSITIVE_INFINITY,
        1.0,
        1.0,
        f64::NEGATIVE_INFINITY,
        1.0,
        -1.0,
        f64::POSITIVE_INFINITY,
        -1.0,
        f64::POSITIVE_INFINITY,
        f64::NEGATIVE_INFINITY,
        -1.0,
        f64::NEGATIVE_INFINITY,
        -1.0,
        1.0,
        1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        -1.0,
        1.0,
        -1.0,
        -1.0,
        1.0,
        0.5,
        1.0,
        -1.0,
        1.0,
        -1.0,
        1.0,
        1.0,
        f64::POSITIVE_INFINITY,
        -1.5,
        f64::NEGATIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        -1.0,
        1.0,
        -1.0,
        -1.0,
        -1.0,
        f64::POSITIVE_INFINITY,
        1.0,
        -1.0,
        -0.5,
        -1.0,
        f64::NEGATIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        1.0,
    ];
    let common_values = &[
        (1.0, 358244),
        (-1.0, 357926),
        (f64::POSITIVE_INFINITY, 125637),
        (f64::NEGATIVE_INFINITY, 124572),
        (2.0, 5538),
        (1.5, 5500),
        (0.5, 5497),
        (-1.5, 5454),
        (-2.0, 5379),
        (-0.5, 5357),
        (0.75, 102),
        (3.0, 98),
        (-4.0, 95),
        (-0.25, 91),
        (-0.75, 87),
        (-3.0, 86),
        (0.25, 79),
        (4.0, 75),
        (-1.25, 48),
        (1.75, 44),
    ];
    let sample_median = (0.5, None);
    special_random_nonzero_primitive_floats_helper::<f64>(
        1,
        64,
        65,
        64,
        1,
        4,
        values,
        common_values,
        sample_median,
    );

    // f64, mean abs of exponent = 1, mean precision = 2, mean special P = 1/10
    let values = &[
        1.0,
        1.25,
        3.0,
        f64::POSITIVE_INFINITY,
        -1.0,
        -1.0,
        -2.0,
        -3.5,
        1.0,
        2.0,
        -1.5,
        -2.5,
        -2.0,
        f64::NEGATIVE_INFINITY,
        -6.5,
        -1.0,
        -1.0,
        f64::POSITIVE_INFINITY,
        3.0,
        -0.21875,
        -1.0,
        0.25,
        1.5,
        5.25,
        -4.0,
        7.0,
        -0.5,
        0.1875,
        1.25,
        f64::POSITIVE_INFINITY,
        -0.1875,
        -7.5,
        f64::NEGATIVE_INFINITY,
        0.75,
        -7.0,
        -6.0,
        -3.0,
        0.234375,
        -2.0,
        -0.875,
        -0.75,
        6.0,
        -24.0,
        24.0,
        -2.0,
        1.5,
        f64::NEGATIVE_INFINITY,
        -1.25,
        14.0,
        5.0,
    ];
    let common_values = &[
        (1.0, 74789),
        (-1.0, 74702),
        (f64::POSITIVE_INFINITY, 50351),
        (f64::NEGATIVE_INFINITY, 49873),
        (1.5, 38119),
        (-0.5, 37713),
        (2.0, 37640),
        (-1.5, 37613),
        (-2.0, 37333),
        (0.5, 37027),
        (0.75, 19050),
        (4.0, 18892),
        (0.25, 18875),
        (-3.0, 18866),
        (3.0, 18821),
        (-0.75, 18725),
        (-4.0, 18663),
        (-0.25, 18537),
        (0.125, 9445),
        (-0.375, 9395),
    ];
    let sample_median = (0.0048828125, None);
    special_random_nonzero_primitive_floats_helper::<f64>(
        1,
        1,
        2,
        1,
        1,
        10,
        values,
        common_values,
        sample_median,
    );

    // f64, mean abs of exponent = 10, mean precision = 10, mean special P = 1/100
    let values = &[
        0.7709910366684198,
        1.2504315236583352e-6,
        0.00830078125,
        -0.8125,
        -85504.0,
        -0.0078125,
        -0.018890380859375,
        2.5721821784973145,
        5.75,
        -0.00003814697265625,
        -0.4375,
        -24064.0,
        -43008.0,
        -1.75,
        -54.6875,
        0.4641265869140625,
        -0.0014760522753931582,
        -1536.0,
        0.1484375,
        0.00146484375,
        1.9383151084184647e-8,
        -0.060546875,
        7340032.0,
        -0.1982421875,
        203.0546875,
        4.57177734375,
        -1555162.0,
        -2.0675361156463623e-7,
        0.279296875,
        -0.0045928955078125,
        -46137344.0,
        -5712.0,
        17.75,
        -5.265625,
        -7.966220855712891,
        -2.99609375,
        5.397188942879438e-6,
        -0.017333984375,
        0.00011491775512695312,
        -0.00005845972555107437,
        0.00020831823348999023,
        -46.78125,
        0.005859375,
        0.0078125,
        27.25,
        30.0,
        -0.175537109375,
        -208.0,
        -0.109375,
        -6144.0,
    ];
    let common_values = &[
        (f64::POSITIVE_INFINITY, 5098),
        (f64::NEGATIVE_INFINITY, 4891),
        (1.0, 2396),
        (-1.0, 2336),
        (-2.0, 2200),
        (-1.5, 2169),
        (0.5, 2116),
        (2.0, 2108),
        (-0.5, 2101),
        (1.5, 2085),
        (-3.0, 2000),
        (4.0, 1993),
        (3.0, 1969),
        (-0.25, 1955),
        (0.75, 1946),
        (0.25, 1917),
        (-4.0, 1882),
        (-0.75, 1863),
        (8.0, 1826),
        (-6.0, 1782),
    ];
    let sample_median = (2.1519930816179568e-19, Some(2.168404344971009e-19));
    special_random_nonzero_primitive_floats_helper::<f64>(
        10,
        1,
        10,
        1,
        1,
        100,
        values,
        common_values,
        sample_median,
    );
}

fn special_random_nonzero_primitive_floats_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(special_random_nonzero_primitive_floats::<T>(
        EXAMPLE_SEED,
        0,
        1,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_nonzero_primitive_floats::<T>(
        EXAMPLE_SEED,
        1,
        0,
        10,
        1,
        1,
        10
    ));
    assert_panic!(special_random_nonzero_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        1,
        1,
        10
    ));
    assert_panic!(special_random_nonzero_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        1,
        0,
        1,
        10
    ));
    assert_panic!(special_random_nonzero_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        10,
        1,
        1,
        0
    ));
    assert_panic!(special_random_nonzero_primitive_floats::<T>(
        EXAMPLE_SEED,
        10,
        1,
        10,
        1,
        2,
        1
    ));
}

#[test]
fn special_random_nonzero_primitive_floats_fail() {
    apply_fn_to_primitive_floats!(special_random_nonzero_primitive_floats_fail_helper);
}
