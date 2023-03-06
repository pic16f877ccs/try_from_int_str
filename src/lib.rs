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
//! assert_eq!(<u32>::try_from_int_str(true), Ok(1u32));
//! assert_eq!(<u32>::try_from_int_str("2023"), Ok(2023u32));
//! assert!(<u8>::try_from_int_str("2023").is_err());
//! assert_eq!(<u64>::try_from_int_str(<u64>::MAX as u128), Ok(u64::MAX));
//! assert_eq!(<u64>::try_from_int_str(u128::MAX).unwrap_err().to_string(),
//! "out of range integral type conversion attempted");
//! ```
#[doc = include_str!("../README.md")]
use core::fmt::{self, Display};
use core::num::ParseIntError;
use core::num::TryFromIntError;
use core::str::FromStr;

/// An error which can be returned when parsing an integer or string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryFromIntStrErr {
    int_str_error: IntStrError,
}

impl TryFromIntStrErr {
    /// Returns the enum error variant when converting a integer or string.
    pub fn multi_err(&self) -> &IntStrError {
        &self.int_str_error
    }
}

impl Display for TryFromIntStrErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.int_str_error {
            IntStrError::ErrorStr(parse_int_error) => {
                write!(f, "{parse_int_error}")
            }
            IntStrError::ErrorInt(try_from_int_error) => {
                write!(f, "{try_from_int_error}")
            }
        }
    }
}

impl From<ParseIntError> for TryFromIntStrErr {
    fn from(err: ParseIntError) -> Self {
        Self {
            int_str_error: IntStrError::ErrorStr(err),
        }
    }
}

impl From<TryFromIntError> for TryFromIntStrErr {
    fn from(err: TryFromIntError) -> Self {
        Self {
            int_str_error: IntStrError::ErrorInt(err),
        }
    }
}

/// Enum to store the various types of errors that can cause parsing an integer or string to fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntStrError {
    /// Core library [`ParseIntError`] structure.
    ErrorStr(ParseIntError),
    /// Core library [`TryFromIntError`] structure.
    ErrorInt(TryFromIntError),
}

/// A generic trait for converting from str or integer to type integer.
pub trait TryFromIntStr<T>: Sized {
    /// Conversion from str or integer to type integer.
    fn try_from_int_str(var: T) -> Result<Self, TryFromIntStrErr>;
}

macro_rules! try_from_int {
    ( $into_type:ty; $($from_type:ty),+ ) => {
        $(
            impl TryFromIntStr<$from_type> for $into_type {

                #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), " losslessly.")]
                #[inline]
                fn try_from_int_str(var: $from_type) -> Result<Self, TryFromIntStrErr> {
                    Ok(var as Self).map_err(|err| TryFromIntStrErr { int_str_error: IntStrError::ErrorInt(err) } )
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

            #[doc = concat!("Converts ", stringify!($into_type), " to ", stringify!($into_type), " losslessly.")]
            #[inline]
            fn try_from_int_str(var: Self) -> Result<Self, TryFromIntStrErr> {
                Ok(var).map_err(|err| TryFromIntStrErr { int_str_error: IntStrError::ErrorInt(err) } )
            }
        }

        $(
            impl TryFromIntStr<$from_type> for $into_type {

                #[doc = concat!("Converts ", stringify!($from_type), " to ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_from_int_str(var: $from_type) -> Result<Self, TryFromIntStrErr> {
                    <Self>::try_from(var).map_err(|err| TryFromIntStrErr { int_str_error: IntStrError::ErrorInt(err.into()) } )
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

                #[doc = concat!("Converts &str to ", stringify!($into_type), ". Conversion can fail.")]
                #[inline]
                fn try_from_int_str(var: &str) -> Result<Self, TryFromIntStrErr> {
                    FromStr::from_str(var).map_err(|err| TryFromIntStrErr{ int_str_error: IntStrError::ErrorStr(err) } )
                }
            }
        )+
    }
}

try_from_str! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }

macro_rules! try_from_bool {
    ( $($into_type:ty),+ ) => {
        $(
            impl TryFromIntStr<bool> for $into_type {

                #[doc = concat!("Converts bool to ", stringify!($into_type), " losslessly.")]
                #[inline]
                fn try_from_int_str(var: bool) -> Result<Self, TryFromIntStrErr> {
                    Ok(var as Self)
                }
            }
        )+
    }
}

try_from_bool! { i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, i128, u128 }
