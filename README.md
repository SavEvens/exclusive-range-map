# exclusive-range-map
[![Rust](https://github.com/SavEvens/exclusive-range-map/actions/workflows/rust-tests.yml/badge.svg)](https://github.com/SavEvens/exclusive-range-map/actions/workflows/rust-tests.yml) <br />
A small crate to map exclusive ranges of numbers to values while allowing quick point retrievals.
While existing solutions exist for fungible values such as rangemap, no such solution existed for exclusive, non-fungible ranges.
This crate allows for exclusive ranges to map to fungible, non-copyable values with minimal memory usage and insert/lookup time.

## Poject Status

This crate is pre-alpha and considered unfit for production use.
