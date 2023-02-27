use paste::paste;
use try_from_int_str::TryFromIntStr;

macro_rules! try_int_from_into {
    ( $from_type:ty; $($($into_type:ty),*; $($and_into_type:ty),*; $type:ty),* )=> {
        paste! {
                #[test]
                fn [<$from_type _try_from_int_$from_type _min>]() {
                   assert_eq!(<$from_type>::try_from_int_str(<$from_type>::MIN), Ok(<$from_type>::MIN));
                }

                #[test]
                fn [<$from_type _try_from_int_$from_type _max>]() {
                   assert_eq!(<$from_type>::try_from_int_str(<$from_type>::MAX), Ok(<$from_type>::MAX));
                }
        }

        $(
            $( paste! {
                    #[test]
                    fn [<$into_type _try_from_int_$from_type _min>]() {
                       assert_eq!(<$into_type>::try_from_int_str(<$from_type>::MIN), Ok(<$from_type>::MIN as $into_type));
                    }

                    #[test]
                    fn [<$into_type _try_from_int_$from_type _max>]() {
                       assert_eq!(<$into_type>::try_from_int_str(<$from_type>::MAX), Ok(<$from_type>::MAX as $into_type));
                    }
                }
            )*

            $( paste! {
                    #[test]
                    fn [<$and_into_type _try_from_int_$from_type _min>]() {
                       assert_eq!(<$and_into_type>::try_from_int_str(<$type>::MIN as $from_type), Ok(<$type>::MIN as $and_into_type));
                    }

                    #[test]
                    fn [<$and_into_type _try_from_int_$from_type _max>]() {
                       assert_eq!(<$and_into_type>::try_from_int_str(<$from_type>::MAX), Ok(<$from_type>::MAX as $and_into_type));
                    }
                }
            )*
        )*
    };
}

try_int_from_into! { i8; i16, i32, i64, isize, i128; u8, u16, u32, u64, usize, u128; u8 }
try_int_from_into! { i16; i32, i64, isize, i128; u16, u32, u64, usize, u128; u16 }
try_int_from_into! { i32; i64, isize, i128; u32, u64, usize, u128; u32 }
try_int_from_into! { i64; isize, i128; u64, usize, u128; u64 }
try_int_from_into! { isize; i64, i128; u64, usize, u128; usize }
try_int_from_into! { i128; ; u128; u128 }

try_int_from_into! { u8; u16, u32, u64, usize, u128; i16, i32, i64, isize, i128; u8 }
try_int_from_into! { u16; u32, u64, usize, u128; i32, i64, isize, i128; u16 }
try_int_from_into! { u32; u64, usize, u128; i64, isize, i128; u32 }
try_int_from_into! { u64; usize, u128; i128; u64 }
try_int_from_into! { usize; u64, u128; i128; usize }
try_int_from_into! { u128; }

macro_rules! try_int_into_from {
    ( $into_type:ty; $($from_type:ty),*; $($and_from_type:ty),*; $type:ty )=> {
        $( paste! {
                #[test]
                fn [<$into_type _try_from_int_$from_type _min>]() {
                   assert_eq!(<$into_type>::try_from_int_str(<$into_type>::MIN as $from_type), Ok(<$into_type>::MIN));
                }

                #[test]
                fn [<$into_type _try_from_int_$from_type _max>]() {
                   assert_eq!(<$into_type>::try_from_int_str(<$into_type>::MAX as $from_type), Ok(<$into_type>::MAX));
                }
            }
        )*

        $( paste! {
                #[test]
                fn [<$into_type _try_from_int_$and_from_type _min>]() {
                   assert_eq!(<$into_type>::try_from_int_str(<$type>::MIN as $and_from_type), Ok(<$type>::MIN as $into_type));
                }

                #[test]
                fn [<$into_type _try_from_int_$and_from_type _max>]() {
                   assert_eq!(<$into_type>::try_from_int_str(<$into_type>::MAX as $and_from_type), Ok(<$into_type>::MAX));
                }
            }
        )*
    };
}

try_int_into_from! { i8; i16, i32, i64, isize, i128; u8, u16, u32, u64, usize, u128; u8 }
try_int_into_from! { i16; i32, i64, isize, i128; u16, u32, u64, usize, u128; u16 }
try_int_into_from! { i32; i64, isize, i128; u32, u64, usize, u128; u32 }
try_int_into_from! { i64; i128; u64, usize, u128; u64}
try_int_into_from! { isize; i128; u64, usize, u128; usize}
try_int_into_from! { i128; ; u128; u128 }

try_int_into_from! { u8; u16, u32, u64, usize, u128; i16, i32, i64, isize, i128; u8 }
try_int_into_from! { u16; u32, u64, usize, u128; i32, i64, isize, i128; u16 }
try_int_into_from! { u32; u64, usize, u128; i64, isize, i128; u32 }
try_int_into_from! { u64; u128; i128; u64 }
try_int_into_from! { usize; u128; i128; usize }

macro_rules! try_str_from_into {
    ( $min_from:expr, $max_from:expr; $($into_type:ty),*; $($and_into_type:ty),*; $type:ty )=> {
        $( paste! {
                #[test]
                fn [<$into_type _try_from_str_$type _min>]() {
                   assert_eq!(<$into_type>::try_from_int_str($min_from), Ok(<$type>::MIN as $into_type));
                }

                #[test]
                fn [<$into_type _try_from_str_$type _max>]() {
                   assert_eq!(<$into_type>::try_from_int_str($max_from), Ok(<$type>::MAX as $into_type));
                }
            }
        )*

        $( paste! {
                #[test]
                fn [<$and_into_type _try_from_str_$type _zero>]() {
                   assert_eq!(<$and_into_type>::try_from_int_str("0"), Ok(0));
                }

                #[test]
                fn [<$and_into_type _try_from_str_$type _max>]() {
                   assert_eq!(<$and_into_type>::try_from_int_str($max_from), Ok(<$type>::MAX as $and_into_type));
                }
            }
        )*

    };
}

try_str_from_into! { "-128", "127"; i8, i16, i32, i64, isize, i128; u8, u16, u32, u64, usize, u128; i8 }
try_str_from_into! { "0", "255"; i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128; ; u8 }
try_str_from_into! { "-32768", "32767"; i16, i32, i64, isize, i128; u16, u32, u64, usize, u128; i16 }
try_str_from_into! { "0", "65535"; u16, i32, u32, i64, u64, isize, usize, i128, u128; ; u16 }
try_str_from_into! { "-2147483648", "2147483647"; i32, i64, isize, i128; u32, u64, usize, u128; i32 }
try_str_from_into! { "0", "4294967295"; i64, u64, isize, usize, i128, u128; ; u32 }
try_str_from_into! { "-9223372036854775808", "9223372036854775807"; i64, isize, i128; u64, usize, u128; i64 }
try_str_from_into! { "-9223372036854775808", "9223372036854775807"; i64, isize, i128; u64, usize, u128; isize }
try_str_from_into! { "0", "18446744073709551615"; u64, usize, i128, u128; ; u64 }
try_str_from_into! { "0", "18446744073709551615"; u64, usize, i128, u128; ; usize }
try_str_from_into! { "-170141183460469231731687303715884105728", "170141183460469231731687303715884105727"; i128; u128; i128 }
try_str_from_into! { "0", "340282366920938463463374607431768211455"; u128; ; u128 }

macro_rules! try_str_from_into_err {
    ( $from_str:expr; $($into_type:ty),* )=> {
        $( paste! {
                #[test]
                fn [<$into_type _try_from_str_$from_str _err>]() {
                   assert_eq!(<$into_type>::try_from_int_str($from_str).unwrap_err().to_string(),
                   $from_str.parse::<$into_type>().unwrap_err().to_string());
                }
            }
        )*

    };
}

try_str_from_into_err! { "340282366920938463463374607431768211456"; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128 }
try_str_from_into_err! { "-340282366920938463463374607431768211456"; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128 }
try_str_from_into_err! { ""; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128 }
try_str_from_into_err! { "rust"; i8, i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128 }

macro_rules! try_int_from_err {
    ( $into_type:ty; $($from_type:ty),* ) => {
        $(
            paste! {
                #[test]
                fn [<$into_type _try_from_int_str_$from_type _err>]() {
                    assert_eq!(<$into_type>::try_from_int_str((<$into_type>::MAX as $from_type) + 1).unwrap_err().to_string(),
                    <$into_type>::try_from((<$into_type>::MAX as $from_type) + 1).unwrap_err().to_string())
                }
            }
        )*
    }
}

try_int_from_err! { i8; i16, i32, i64, isize, i128, u8, u16, u32, u64, usize, u128 }
try_int_from_err! { i16; i32, i64, isize, i128, u16, u32, u64, usize, u128 }
try_int_from_err! { i32; i64, isize, i128, u32, u64, usize, u128 }
try_int_from_err! { i64; i128, u64, usize, u128 }
try_int_from_err! { isize; i128, u64, usize, u128 }
try_int_from_err! { i128; u128 }

try_int_from_err! { u8; i16, i32, i64, isize, i128, u16, u32, u64, usize, u128 }
try_int_from_err! { u16; i32, i64, isize, i128, u32, u64, usize, u128 }
try_int_from_err! { u32; i64, isize, i128, u64, usize, u128 }
try_int_from_err! { u64; i128, u128 }
try_int_from_err! { usize; i128, u128 }
