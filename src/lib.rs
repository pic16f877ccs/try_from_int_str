#![forbid(unsafe_code)]
#![no_std]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(rustdoc::broken_intra_doc_links)]
//! A generic trait for converting from str or integer to type integer.
//!
//! # Examples
//! Usage:
//!
//! ```
//! # use try_from_int_str::TryFromIntStr;
//! assert_eq!(<u32>::try_from_int_str("2023"), Ok(2023u32));
//! assert_eq!(<u64>::try_from_int_str(<u64>::MAX as u128), Ok(u64::MAX));
//! ```
use core::convert::Infallible;
use core::num::ParseIntError;
use core::num::TryFromIntError;

/// A generic trait for converting from str or integer to type integer.
pub trait TryFromIntStr<T>: Sized {
    /// The type returned in the event of a conversion error.
    type IntStrErr;

    /// Conversion from str or integer to type integer.
    fn try_from_int_str(var: T) -> Result<Self, Self::IntStrErr>;
}

macro_rules! try_from_int {
    ( $into_type:ty; $($from_type:ty),+ ) => {
        $(
            impl TryFromIntStr<$from_type> for $into_type {
                type IntStrErr = Infallible;

                #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), " losslessly.")]
                #[inline]
                fn try_from_int_str(var: $from_type) -> Result<Self, Self::IntStrErr> {
                        Ok(var as Self)
                }
            }
        )+
    };
}

try_from_int! { i16; i8 }
try_from_int! { u16; u8 }
try_from_int! { i32; i8, i16 }
try_from_int! { u32; u8, u16 }
try_from_int! { i64; i8, i16, i32, isize }
try_from_int! { u64; u8, u16, u32, usize }
try_from_int! { isize; i8, i16, i32, i64 }
try_from_int! { usize; u8, u16, u32, u64 }
try_from_int! { i128; i8, i16, i32, i64, isize }
try_from_int! { u128; u8, u16, u32, u64, usize }

macro_rules! try_from_int_into {
    ( $into_type:ty; $($from_type:ty),+ ) => {
        impl TryFromIntStr<$into_type> for $into_type {
            type IntStrErr = Infallible;

            #[doc = concat!("Converts ", stringify!($into_type), " to ", stringify!($into_type), " losslessly.")]
            #[inline]
            fn try_from_int_str(var: Self) -> Result<Self, Self::IntStrErr> {
                Ok(var)
            }
        }

        $(
            impl TryFromIntStr<$from_type> for $into_type {
                type IntStrErr = TryFromIntError;

                #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_from_int_str(var: $from_type) -> Result<Self, Self::IntStrErr> {
                    match var.try_into() {
                        Ok(ok) => Ok(ok),
                        Err(err) => Err(err.into()),
                    }
                }
            }
        )+
    };
}

try_from_int_into! { i8; u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
try_from_int_into! { u8; i8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
try_from_int_into! { i16; u8, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
try_from_int_into! { u16; i8, i16, i32, u32, i64, u64, isize, usize, i128, u128 }
try_from_int_into! { i32; u8, u16, u32, i64, u64, isize, usize, i128, u128 }
try_from_int_into! { u32; i8, i16, i32, i64, u64, isize, usize, i128, u128 }
try_from_int_into! { i64; u8, u16, u32, u64, usize, i128, u128 }
try_from_int_into! { u64; i8, i16, i32, i64, isize, i128, u128 }
try_from_int_into! { isize; u8, u16, u32, u64, usize, i128, u128 }
try_from_int_into! { usize; i8, i16, i32, i64, isize, i128, u128 }
try_from_int_into! { i128; u8, u16, u32, u64, usize, u128 }
try_from_int_into! { u128; i8, i16, i32, i64, isize, i128 }

macro_rules! try_from_str {
    ( $($into_type:ty),+ ) => {
        $(
            impl TryFromIntStr<&str> for $into_type {
                type IntStrErr = ParseIntError;

                #[doc = concat!("Converts &str to ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_from_int_str(var: &str) -> Result<Self, Self::IntStrErr> {
                    match var.parse() {
                        Ok(ok) => Ok(ok),
                        Err(err) => Err(err),
                    }
                }
            }
        )+
    }
}

try_from_str! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
