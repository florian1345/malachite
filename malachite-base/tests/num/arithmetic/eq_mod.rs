use malachite_base::num::basic::integers::PrimitiveInt;

#[test]
fn test_eq_mod() {
    fn test<T: PrimitiveInt>(x: T, y: T, m: T, out: bool) {
        assert_eq!(x.eq_mod(y, m), out);
    };
    test::<u8>(0, 0, 0, true);
    test::<u16>(0, 1, 0, false);
    test::<u32>(57, 57, 0, true);
    test::<u64>(57, 58, 0, false);
    test::<u128>(1_000_000_000_000, 57, 0, false);
    test::<usize>(0, 256, 256, true);
    test::<u16>(0, 256, 512, false);
    test::<u16>(13, 23, 10, true);
    test::<u32>(13, 24, 10, false);
    test::<u64>(13, 21, 1, true);
    test::<u128>(13, 21, 2, true);
    test::<usize>(13, 21, 4, true);
    test::<u8>(13, 21, 8, true);
    test::<u16>(13, 21, 16, false);
    test::<u32>(13, 21, 3, false);
    test::<u64>(1_000_000_000_001, 1, 4_096, true);
    test::<u128>(1_000_000_000_001, 1, 8_192, false);
    test::<u64>(12_345_678_987_654_321, 321, 1_000, true);
    test::<u64>(12_345_678_987_654_321, 322, 1_000, false);
    test::<u64>(1_234, 1_234, 1_000_000_000_000, true);
    test::<u64>(1_234, 1_235, 1_000_000_000_000, false);
    test::<u64>(1_000_000_001_234, 1_000_000_002_234, 1_000, true);
    test::<u64>(1_000_000_001_234, 1_000_000_002_235, 1_000, false);
    test::<u64>(1_000_000_001_234, 1_234, 1_000_000_000_000, true);
    test::<u64>(1_000_000_001_234, 1_235, 1_000_000_000_000, false);
    test::<u64>(
        1_000_000_001_234,
        5_000_000_001_234,
        1_000_000_000_000,
        true,
    );
    test::<u64>(
        1_000_000_001_234,
        5_000_000_001_235,
        1_000_000_000_000,
        false,
    );

    test::<i8>(0, -1, 0, false);
    test::<i16>(57, -57, 0, false);
    test::<i32>(57, -58, 0, false);
    test::<i64>(1_000_000_000_000, -57, 0, false);
    test::<i128>(0, -256, 256, true);
    test::<isize>(0, -256, 512, false);
    test::<i8>(13, -27, 10, true);
    test::<i16>(13, -28, 10, false);
    test::<i32>(29, -27, 1, true);
    test::<i64>(29, -27, 2, true);
    test::<i128>(29, -27, 4, true);
    test::<isize>(29, -27, 8, true);
    test::<i8>(29, -27, 16, false);
    test::<i16>(29, -27, 3, false);
    test::<i64>(999_999_999_999, -1, 4_096, true);
    test::<i64>(999_999_999_999, -1, 8_192, false);
    test::<i64>(12_345_678_987_654_321, -679, 1_000, true);
    test::<i64>(12_345_678_987_654_321, -680, 1_000, false);
    test::<i64>(1_000_000_001_234, -999_999_999_766, 1_000, true);
    test::<i64>(1_000_000_001_234, -999_999_999_767, 1_000, false);
    test::<i64>(1_000_000_001_234, -999_999_998_766, 1_000_000_000_000, true);
    test::<i64>(
        1_000_000_001_234,
        -999_999_998_767,
        1_000_000_000_000,
        false,
    );

    test::<i16>(-57, 57, 0, false);
    test::<i32>(-57, 58, 0, false);
    test::<i64>(-1_000_000_000_000, 57, 0, false);
    test::<i8>(-13, 27, 10, true);
    test::<i16>(-13, 28, 10, false);
    test::<i32>(-29, 27, 1, true);
    test::<i64>(-29, 27, 2, true);
    test::<i128>(-29, 27, 4, true);
    test::<isize>(-29, 27, 8, true);
    test::<i8>(-29, 27, 16, false);
    test::<i16>(-29, 27, 3, false);
    test::<i64>(-999_999_999_999, 1, 4_096, true);
    test::<i64>(-999_999_999_999, 1, 8_192, false);
    test::<i64>(-12_345_678_987_654_321, 679, 1_000, true);
    test::<i64>(-12_345_678_987_654_321, 680, 1_000, false);
    test::<i64>(-1_000_000_001_234, 999_999_999_766, 1_000, true);
    test::<i64>(-1_000_000_001_234, 999_999_999_767, 1_000, false);
    test::<i64>(-1_000_000_001_234, 999_999_998_766, 1_000_000_000_000, true);
    test::<i64>(
        -1_000_000_001_234,
        999_999_998_767,
        1_000_000_000_000,
        false,
    );

    test::<i32>(-57, -57, 0, true);
    test::<i64>(-57, -58, 0, false);
    test::<i128>(-1_000_000_000_000, -57, 0, false);
    test::<i16>(-13, -23, 10, true);
    test::<i32>(-13, -24, 10, false);
    test::<i64>(-13, -21, 1, true);
    test::<i128>(-13, -21, 2, true);
    test::<isize>(-13, -21, 4, true);
    test::<i8>(-13, -21, 8, true);
    test::<i16>(-13, -21, 16, false);
    test::<i32>(-13, -21, 3, false);
    test::<i64>(-1_000_000_000_001, -1, 4_096, true);
    test::<i128>(-1_000_000_000_001, -1, 8_192, false);
    test::<i64>(-12_345_678_987_654_321, -321, 1_000, true);
    test::<i64>(-12_345_678_987_654_321, -322, 1_000, false);
    test::<i64>(-1_234, -1_234, 1_000_000_000_000, true);
    test::<i64>(-1_234, -1_235, 1_000_000_000_000, false);
    test::<i64>(-1_000_000_001_234, -1_000_000_002_234, 1_000, true);
    test::<i64>(-1_000_000_001_234, -1_000_000_002_235, 1_000, false);
    test::<i64>(-1_000_000_001_234, -1_234, 1_000_000_000_000, true);
    test::<i64>(-1_000_000_001_234, -1_235, 1_000_000_000_000, false);
    test::<i64>(
        -1_000_000_001_234,
        -5_000_000_001_234,
        1_000_000_000_000,
        true,
    );
    test::<i64>(
        -1_000_000_001_234,
        -5_000_000_001_235,
        1_000_000_000_000,
        false,
    );

    test::<isize>(0, 256, -256, true);
    test::<i16>(0, 256, -512, false);
    test::<i16>(13, 23, -10, true);
    test::<i32>(13, 24, -10, false);
    test::<i64>(13, 21, -1, true);
    test::<i128>(13, 21, -2, true);
    test::<isize>(13, 21, -4, true);
    test::<i8>(13, 21, -8, true);
    test::<i16>(13, 21, -16, false);
    test::<i32>(13, 21, -3, false);
    test::<i64>(1_000_000_000_001, 1, -4_096, true);
    test::<i128>(1_000_000_000_001, 1, -8_192, false);
    test::<i64>(12_345_678_987_654_321, 321, -1_000, true);
    test::<i64>(12_345_678_987_654_321, 322, -1_000, false);
    test::<i64>(1_234, 1_234, -1_000_000_000_000, true);
    test::<i64>(1_234, 1_235, -1_000_000_000_000, false);
    test::<i64>(1_000_000_001_234, 1_000_000_002_234, -1_000, true);
    test::<i64>(1_000_000_001_234, 1_000_000_002_235, -1_000, false);
    test::<i64>(1_000_000_001_234, 1_234, -1_000_000_000_000, true);
    test::<i64>(1_000_000_001_234, 1_235, -1_000_000_000_000, false);
    test::<i64>(
        1_000_000_001_234,
        5_000_000_001_234,
        -1_000_000_000_000,
        true,
    );
    test::<i64>(
        1_000_000_001_234,
        5_000_000_001_235,
        -1_000_000_000_000,
        false,
    );

    test::<i128>(0, -256, -256, true);
    test::<isize>(0, -256, -512, false);
    test::<i8>(13, -27, -10, true);
    test::<i16>(13, -28, -10, false);
    test::<i32>(29, -27, -1, true);
    test::<i64>(29, -27, -2, true);
    test::<i128>(29, -27, -4, true);
    test::<isize>(29, -27, -8, true);
    test::<i8>(29, -27, -16, false);
    test::<i16>(29, -27, -3, false);
    test::<i64>(999_999_999_999, -1, -4_096, true);
    test::<i64>(999_999_999_999, -1, -8_192, false);
    test::<i64>(12_345_678_987_654_321, -679, -1_000, true);
    test::<i64>(12_345_678_987_654_321, -680, -1_000, false);
    test::<i64>(1_000_000_001_234, -999_999_999_766, -1_000, true);
    test::<i64>(1_000_000_001_234, -999_999_999_767, -1_000, false);
    test::<i64>(
        1_000_000_001_234,
        -999_999_998_766,
        -1_000_000_000_000,
        true,
    );
    test::<i64>(
        1_000_000_001_234,
        -999_999_998_767,
        -1_000_000_000_000,
        false,
    );

    test::<i8>(-13, 27, -10, true);
    test::<i16>(-13, 28, -10, false);
    test::<i32>(-29, 27, -1, true);
    test::<i64>(-29, 27, -2, true);
    test::<i128>(-29, 27, -4, true);
    test::<isize>(-29, 27, -8, true);
    test::<i8>(-29, 27, -16, false);
    test::<i16>(-29, 27, -3, false);
    test::<i64>(-999_999_999_999, 1, -4_096, true);
    test::<i64>(-999_999_999_999, 1, -8_192, false);
    test::<i64>(-12_345_678_987_654_321, 679, -1_000, true);
    test::<i64>(-12_345_678_987_654_321, 680, -1_000, false);
    test::<i64>(-1_000_000_001_234, 999_999_999_766, -1_000, true);
    test::<i64>(-1_000_000_001_234, 999_999_999_767, -1_000, false);
    test::<i64>(
        -1_000_000_001_234,
        999_999_998_766,
        -1_000_000_000_000,
        true,
    );
    test::<i64>(
        -1_000_000_001_234,
        999_999_998_767,
        -1_000_000_000_000,
        false,
    );

    test::<i16>(-13, -23, -10, true);
    test::<i32>(-13, -24, -10, false);
    test::<i64>(-13, -21, -1, true);
    test::<i128>(-13, -21, -2, true);
    test::<isize>(-13, -21, -4, true);
    test::<i8>(-13, -21, -8, true);
    test::<i16>(-13, -21, -16, false);
    test::<i32>(-13, -21, -3, false);
    test::<i64>(-1_000_000_000_001, -1, -4_096, true);
    test::<i128>(-1_000_000_000_001, -1, -8_192, false);
    test::<i64>(-12_345_678_987_654_321, -321, -1_000, true);
    test::<i64>(-12_345_678_987_654_321, -322, -1_000, false);
    test::<i64>(-1_234, -1_234, -1_000_000_000_000, true);
    test::<i64>(-1_234, -1_235, -1_000_000_000_000, false);
    test::<i64>(-1_000_000_001_234, -1_000_000_002_234, -1_000, true);
    test::<i64>(-1_000_000_001_234, -1_000_000_002_235, -1_000, false);
    test::<i64>(-1_000_000_001_234, -1_234, -1_000_000_000_000, true);
    test::<i64>(-1_000_000_001_234, -1_235, -1_000_000_000_000, false);
    test::<i64>(
        -1_000_000_001_234,
        -5_000_000_001_234,
        -1_000_000_000_000,
        true,
    );
    test::<i64>(
        -1_000_000_001_234,
        -5_000_000_001_235,
        -1_000_000_000_000,
        false,
    );
}
