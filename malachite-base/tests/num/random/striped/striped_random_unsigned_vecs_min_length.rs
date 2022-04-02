use itertools::Itertools;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::random::striped::striped_random_unsigned_vecs_min_length;
use malachite_base::random::EXAMPLE_SEED;
use malachite_base::strings::ToBinaryString;
use malachite_base::test_util::stats::common_values_map::common_values_map_debug;
use malachite_base::test_util::stats::median;

fn striped_random_unsigned_vecs_min_length_helper<T: PrimitiveUnsigned>(
    min_length: u64,
    mean_stripe_numerator: u64,
    mean_stripe_denominator: u64,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
    expected_values: &[&[&str]],
    expected_common_values: &[(&[&str], usize)],
    expected_median: (&[&str], Option<&[&str]>),
) {
    let xss = striped_random_unsigned_vecs_min_length::<T>(
        EXAMPLE_SEED,
        min_length,
        mean_stripe_numerator,
        mean_stripe_denominator,
        mean_length_numerator,
        mean_length_denominator,
    );
    let values = xss
        .clone()
        .take(20)
        .map(|xs| {
            xs.into_iter()
                .map(|x: T| x.to_binary_string())
                .collect_vec()
        })
        .collect_vec();
    let common_values = common_values_map_debug(1000000, 10, xss.clone())
        .into_iter()
        .map(|(xs, freq)| {
            (
                xs.into_iter()
                    .map(|x: T| x.to_binary_string())
                    .collect_vec(),
                freq,
            )
        })
        .collect_vec();
    let (median_lo, median_hi) = median(xss.take(1000000));
    let median_lo = median_lo
        .into_iter()
        .map(|x: T| x.to_binary_string())
        .collect_vec();
    let median_hi = median_hi.map(|xs| {
        xs.into_iter()
            .map(|x: T| x.to_binary_string())
            .collect_vec()
    });

    let values = values
        .iter()
        .map(|xs| xs.iter().map(String::as_str).collect_vec())
        .collect_vec();
    let common_values = common_values
        .iter()
        .map(|(xs, freq)| (xs.iter().map(String::as_str).collect_vec(), *freq))
        .collect_vec();
    let median_lo = median_lo.iter().map(String::as_str).collect_vec();
    let median_hi = median_hi
        .as_ref()
        .map(|xs| xs.iter().map(String::as_str).collect_vec());
    assert_eq!(
        (
            values.iter().map(Vec::as_slice).collect_vec().as_slice(),
            common_values
                .iter()
                .map(|(xs, f)| (xs.as_slice(), *f))
                .collect_vec()
                .as_slice(),
            (median_lo.as_slice(), median_hi.as_deref())
        ),
        (expected_values, expected_common_values, expected_median)
    );
}

#[test]
fn test_striped_random_unsigned_vecs() {
    striped_random_unsigned_vecs_min_length_helper::<u8>(
        0,
        2,
        1,
        4,
        1,
        &[
            &[],
            &[
                "11001100", "11000", "11001", "10111101", "10111110", "110000", "11010011", "11",
                "1110110", "11100011", "1", "1100101", "10111110", "11010111",
            ],
            &["101110", "1111000", "10110010", "11101110"],
            &["10110110", "11000010", "11111010", "1100110"],
            &["1000"],
            &[],
            &["10100000", "100101", "1000010", "1100110", "11000111"],
            &["111", "11100001"],
            &["1010110", "10101110", "10111000", "10111101"],
            &[],
            &["101011", "10", "1110101", "1110001", "11101111", "10001001"],
            &[],
            &[],
            &[
                "10000100", "11110101", "11011100", "10011111", "10001000", "11001111", "1111000",
                "11010111", "1101001", "111110", "1100100",
            ],
            &[
                "10101", "11011001", "10100000", "10100001", "1101100", "1101111", "10100011",
                "11110101",
            ],
            &[],
            &["10111111", "100111", "1111110"],
            &[],
            &["11000", "11110010", "11111", "1110011", "11110011"],
            &[
                "10001110", "10011", "1100101", "111100", "10110111", "1101110", "100001",
                "10000000", "10101100",
            ],
        ],
        &[
            (&[], 199913),
            (&["11010010"], 689),
            (&["11"], 688),
            (&["11110111"], 681),
            (&["10000110"], 673),
            (&["110010"], 672),
            (&["11101111"], 671),
            (&["1000100"], 670),
            (&["1111101"], 670),
            (&["11110010"], 670),
        ],
        (&["1100000"], None),
    );
    striped_random_unsigned_vecs_min_length_helper::<u8>(
        3,
        2,
        1,
        4,
        1,
        &[
            &["11001100", "11000", "11001", "10111101"],
            &["10000010", "10011110", "1011001", "11111000", "10011"],
            &["1110010", "11111000", "1101011", "110", "10100001"],
            &["1000110", "11111", "110110", "1000101", "1101100"],
            &["1010", "11101011", "10011011", "10001"],
            &["10111111", "10110100", "1111011", "110011", "1110001"],
            &["11110000", "111101", "1010110"],
            &["10100011", "10001110", "10000100"],
            &["1010110", "100", "11101010", "11100010"],
            &["10111101", "100111", "11110110"],
            &["101001", "10001100", "10000000"],
            &["1000100", "1111100", "11000110"],
            &["10001000", "1100010", "11001", "10111100"],
            &["10101100", "1101110", "11110010", "11100101", "110101", "1001", "11001001"],
            &["1001011", "1", "10000100"],
            &["11000100", "10001111", "100001", "11111111", "110001"],
            &["10010001", "11100001", "111000"],
            &[
                "10100010", "10001100", "11100111", "11010110", "101101", "100", "10010000",
                "10010101",
            ],
            &["1110110", "10110100", "1110110", "111001"],
            &["10100111", "1110010", "1101", "1011010"],
        ],
        &[
            (&["111100", "11", "1111"], 3),
            (&["10000111", "100011", "110"], 3),
            (&["10101011", "1001101", "1"], 3),
            (&["10011", "1111000", "111"], 3),
            (&["10", "10111", "11111000"], 3),
            (&["11010000", "1001010", "11"], 3),
            (&["101", "1001110", "1110111"], 3),
            (&["101", "1001111", "10111101"], 3),
            (&["111000", "1010011", "1010"], 3),
            (&["111011", "0", "1110101"], 3),
        ],
        (
            &["1111111", "11011110", "10110011"],
            Some(&["1111111", "11011110", "10111100"]),
        ),
    );
    striped_random_unsigned_vecs_min_length_helper::<u8>(
        0,
        10,
        1,
        4,
        1,
        &[
            &[],
            &[
                "0", "0", "111000", "0", "11111110", "10000001", "11111111", "11", "0", "0", "0",
                "11111111", "11111111", "11111",
            ],
            &["11110000", "11111111", "11111101", "1111111"],
            &["0", "0", "11100000", "1"],
            &["11111110"],
            &[],
            &["0", "0", "10011000", "11111111", "111"],
            &["11111111", "11111111"],
            &["0", "0", "0", "0"],
            &[],
            &["11111111", "11111110", "11111111", "11111111", "11111", "0"],
            &[],
            &[],
            &[
                "0", "0", "0", "0", "11111100", "111011", "0", "1111000", "111", "1101000",
                "11011111",
            ],
            &[
                "11111111", "11111111", "11111111", "11111", "11000000", "11", "11001000",
                "11111111",
            ],
            &[],
            &["1", "11100000", "11111111"],
            &[],
            &["1000", "0", "0", "11111111", "1111"],
            &["0", "10000000", "111", "10000000", "111111", "0", "0", "11111000", "11111111"],
        ],
        &[
            (&[], 199913),
            (&["0"], 38129),
            (&["11111111"], 38051),
            (&["0", "0"], 13204),
            (&["11111111", "11111111"], 13153),
            (&["0", "0", "0"], 4662),
            (&["11111111", "11111111", "11111111"], 4549),
            (&["1"], 4369),
            (&["11111100"], 4338),
            (&["11111"], 4311),
        ],
        (&["11100", "11111110", "111111", "0"], None),
    );
    striped_random_unsigned_vecs_min_length_helper::<u8>(
        3,
        10,
        1,
        4,
        1,
        &[
            &["0", "0", "111000", "0"],
            &["11111100", "11", "11111111", "111", "0"],
            &["0", "0", "11111100", "11111111", "1111111"],
            &["11000000", "11111111", "11110111", "11111111", "1"],
            &["0", "10000000", "111", "11111100"],
            &["11111111", "11111111", "11001111", "0", "11110000"],
            &["0", "0", "0"],
            &["11111111", "11111111", "11111111"],
            &["0", "10", "0", "0"],
            &["1111111", "0", "0"],
            &["11111111", "11111111", "11111111"],
            &["11100000", "11011111", "1"],
            &["10000000", "1110111", "10000000", "11110110"],
            &["100", "0", "0", "0", "11111110", "11000011", "1111111"],
            &["11111001", "11111", "0"],
            &["11111100", "1111111", "11111111", "11111111", "1111"],
            &["11111111", "1", "0"],
            &["11110000", "0", "11110000", "111", "0", "0", "11111111", "1111111"],
            &["0", "11110000", "10011111", "11111111"],
            &["10111111", "11", "11100000", "1111"],
        ],
        &[
            (&["0", "0", "0"], 22115),
            (&["11111111", "11111111", "11111111"], 21981),
            (&["0", "0", "0", "0"], 4887),
            (&["11111111", "11111111", "11111111", "11111111"], 4789),
            (&["0", "11000000", "11111111"], 2545),
            (&["11110000", "11111111", "11111111"], 2537),
            (&["11111111", "11111111", "1111"], 2536),
            (&["1111111", "0", "0"], 2524),
            (&["11111111", "111111", "0"], 2524),
            (&["10000000", "11111111", "11111111"], 2524),
        ],
        (
            &["10000000", "0", "0", "0", "100", "0", "0", "11111000", "11111111"],
            Some(&["10000000", "0", "0", "0", "100", "11111110", "11111111", "11111", "11100000"]),
        ),
    );
    striped_random_unsigned_vecs_min_length_helper::<u8>(
        0,
        11,
        10,
        4,
        1,
        &[
            &[],
            &[
                "1011010", "11010101", "1001010", "10110101", "11010110", "10101010", "10101010",
                "1101010", "10100101", "10101010", "10011010", "1010010", "1010101", "1010101",
            ],
            &["10101010", "1010110", "101011", "1010101"],
            &["1010100", "1010101", "1010101", "10101010"],
            &["1101010"],
            &[],
            &["1001010", "1010101", "1010101", "1010101", "1001001"],
            &["10101011", "10101010"],
            &["10101010", "10101101", "10101010", "1011010"],
            &[],
            &["10101011", "10101010", "10101010", "11010", "11010", "1010111"],
            &[],
            &[],
            &[
                "10101010", "1001011", "11010101", "1010010", "1010101", "10101010", "101010",
                "1010101", "10101001", "1101010", "1010101",
            ],
            &[
                "1010101", "1010101", "1010101", "10110101", "10100100", "10110100", "10101010",
                "10101010",
            ],
            &[],
            &["1010101", "10100101", "10101010"],
            &[],
            &["10101010", "1010100", "1101010", "10100101", "1001010"],
            &[
                "10101100", "10101010", "10101010", "10010101", "10101010", "10101101", "10101010",
                "1001010", "1010101",
            ],
        ],
        &[
            (&[], 199913),
            (&["1010101"], 41088),
            (&["10101010"], 40900),
            (&["1010101", "1010101"], 15274),
            (&["10101010", "10101010"], 15212),
            (&["10101010", "10101010", "10101010"], 5901),
            (&["1010101", "1010101", "1010101"], 5641),
            (&["10101001"], 4206),
            (&["10100101"], 4201),
            (&["10101101"], 4181),
        ],
        (&["1010101", "10110101"], None),
    );
    striped_random_unsigned_vecs_min_length_helper::<u8>(
        3,
        11,
        10,
        4,
        1,
        &[
            &["1011010", "11010101", "1001010", "10110101"],
            &["1010010", "10101010", "10101010", "101010", "10110101"],
            &["10101010", "1101010", "1001010", "1010101", "1010101"],
            &["10101010", "1011010", "10101101", "1010100", "10101001"],
            &["10101010", "10101010", "1010110", "11010101"],
            &["10010101", "10101010", "10101010", "10101010", "10010010"],
            &["1010110", "1010101", "1010101"],
            &["1011011", "1010101", "10110101"],
            &["1010110", "1010101", "1010101", "110101"],
            &["10010111", "10100011", "10101010"],
            &["11010001", "10101010", "10110100"],
            &["10101010", "1010010", "1010101"],
            &["1010010", "10010101", "10101010", "1010110"],
            &["10101010", "10101010", "10101010", "10101010", "10110100", "10110101", "1010100"],
            &["1010101", "1010101", "1010101"],
            &["1010100", "1010101", "10110101", "1011010", "10101001"],
            &["10101011", "110110", "1010101"],
            &[
                "10101010", "1001010", "10101101", "1001010", "10101010", "10101010", "1010110",
                "1010101",
            ],
            &["1011010", "1010101", "1010101", "10101011"],
            &["1010101", "1010101", "10101010", "11101010"],
        ],
        &[
            (&["10101010", "10101010", "10101010"], 27990),
            (&["1010101", "1010101", "1010101"], 27615),
            (&["10101010", "10101010", "10101010", "10101010"], 6458),
            (&["1010101", "1010101", "1010101", "1010101"], 6439),
            (&["10101010", "1011010", "1010101"], 2932),
            (&["10101010", "10101010", "101010"], 2904),
            (&["10100101", "10101010", "10101010"], 2904),
            (&["10110101", "10101010", "10101010"], 2878),
            (&["1010101", "1010101", "10010101"], 2877),
            (&["10101010", "10101010", "1001010"], 2864),
        ],
        (
            &["10001001", "10101010", "11010100", "1010010", "1010101"],
            Some(&["10001001", "10101010", "11010110", "10101010", "10101010"]),
        ),
    );
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_min_length_fail_1() {
    striped_random_unsigned_vecs_min_length::<u8>(EXAMPLE_SEED, 3, 1, 0, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_min_length_fail_2() {
    striped_random_unsigned_vecs_min_length::<u8>(EXAMPLE_SEED, 3, 2, 3, 4, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_min_length_fail_3() {
    striped_random_unsigned_vecs_min_length::<u8>(EXAMPLE_SEED, 3, 4, 1, 3, 1);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_min_length_fail_4() {
    striped_random_unsigned_vecs_min_length::<u8>(EXAMPLE_SEED, 1, 4, 1, 1, 0);
}

#[test]
#[should_panic]
fn striped_random_unsigned_vecs_min_length_fail_5() {
    striped_random_unsigned_vecs_min_length::<u8>(EXAMPLE_SEED, 0, 4, 1, u64::MAX, u64::MAX - 1);
}
