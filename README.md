## try_from_int_str

Rust library for converting integers or str type.                                                                                                                     
## Description
A generic trait for converting from str or integer to type integer.

#### Add this to your Cargo.toml
```rust,ignore
[dependencies]
try_from_int_str = { git = "https://github.com/pic16f877ccs/try_from_int_str", version = "0.1.0" }
```
#### Or using cargo
```rust,ignore
cargo add try_from_int_str --git "https://github.com/pic16f877ccs/try_from_int_str"
 
```
#### Examples
Usage:

```
# use try_from_int_str::TryFromIntStr; 
assert_eq!(<u32>::try_from_int_str("2023"), Ok(2023u32));
assert_eq!(<u64>::try_from_int_str(<u64>::MAX as u128), Ok(u64::MAX));
```

## License
GNU General Public License v3.0
