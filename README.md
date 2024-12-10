# AoC 2024

[![AoC](https://img.shields.io/badge/AoC-2024-blue)](https://adventofcode.com/2024)
[![Rust CI](https://github.com/Jokymon/AoC-2024/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Jokymon/AoC-2024/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/Jokymon/AoC-2024/blob/master/LICENSE)

## Puzzle Inputs

Save the input for a given puzzle under `./input_data/day<xy>/input.txt` (e.g. for Day3, save it as `./input_data/day3/input.txt`)

## Usage

To get the answers for a day, you can run the corresponding executable using `cargo`. For example to get the results of day,
you would run `cargo run --bin day1`. The executables automatically search for the necessary test input in the correct
subdirectory in `./input_data`.

To run the unittests for a day, you can run the corresponding test cases with cargo. For example to get the test results
for day 1, you would run `cargo test --bin day1`.

To create the Rust-skeleton for a new day, you can use the `new_day` executable. For example when you want to create the
skeleton for day 12, you would run `cargo run --bin new_day day12`. This creates a new subdirectory and a source file in
the `./src` tree with the name `day12` and creates a new `[[bin]]`-entry in the `Cargo.toml` file.

## Observations and experiences

In this section I loosely collect my experiences, good, bad and key take aways from trying to solve AoC 2024 with Rust.

### Take aways

Learnings for the future for further AoC participations or general Rust coding

 * use `.lines()` instead of `.split('\n')` because it more clearly describes the
   intent and also handles different types of line endings
 * yep, definitly i64 is likely a good choice for default numbers type in AoC ;-)

### Cool, happy

 * Nice integration in VSCode with type annotations that are automatically added in 
   (you can even double click to explicitly insert them)
 * Tests can be run from inside the code either individually or by test module
 * Lots (most) of the datatypes can easily be printed with the `Debug` trait using
   the `{:?}` pattern.
 * Lots of helpful iterator functions
 * itertools often was a very helpful package, e.x. for `combinations()` on day 8
 * Adding a trait to `Vec<&str>` to make it feel like a 2-D map of characters with my
   new functions like `char_at(x, y)` and `has_position(x, y)` felt really easy.

### Unsure, unhappy

 * Enums are not by default comparable and need to derive `PartialEq`
 * Need to explicitly think about copy/clone/move and add a corresponding `derive`
 * Just like SaltBae you throw in some `&`, `*` in all sorts of places
 * For `::<some_type>()` it feels similar but less so
 * Strings are not easily indexable because of UTF-8 problems. For AoC we have to
   use `.as_bytes()` and then convert back again from `u8` to `char`
 * *list*`.len()` returns a `usize` which is unsigned, but often I needed a signed
   type like `i32`. In C/C++ that "looked" simple with the implicit conversion. In
   Rust I have to `.... as i32` in a couple of places.
 * Understanding the functions from the documentation is often very hard.
   The hover text shows the traits which are sometimes so abstract that
   the actual functionality is barely understandable or you only get
   documentation for a `struct` that is returned and that would actually
   most importantly implement an `Iterator` trait.
 * Handling of strings is hard because of special UTF-8 handling