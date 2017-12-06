# aoc-2017
Advent of code 2017

## Rust

Each directory (`dayXX`) is a Rust code repository.  
You can execute the code directly, for each directory, using `cargo run`.  
For example, to use the `day01` code: `cd day01 && cargo run -- 1111`.

Also, for each directory, you can execute the unit tests.  
For example, to test `day01`: `cd day01 && cargo test`.

Tests have been performed using `rustc 1.22.1 (05e2e1c41 2017-11-22)`.  
If you are working on another `rustc` version, you can override it for the current code repository using `rustup override set stable-2017-11-22`.
