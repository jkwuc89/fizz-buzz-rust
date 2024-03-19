# Fizz Buzz Exercize for Pinnacle Solutions

## Introduction
Choose a language I do not know and implement the solution to
the [Fizz Buzz Test](https://wiki.c2.com/?FizzBuzzTest).

## Language
I chose [Rust](https://www.rust-lang.org) to implement the solution because I've seen quite
a bit of coverage of Rust online and at conferences and it is used to implement
the YJIT Ruby Compiler.

## Learning Approach
* To get started, I used [Getting Started](https://www.rust-lang.org/learn/get-started) to 
  install Rust and verify that everything is working correctly.
* I used `cargo new fizz-buzz-rust` to create a new project for my implementation and ran
  `cargo run` to verify the ability to run the new project. After doing this, I imported the project
  into IntelliJ IDEA (my preferred IDE) to complete the implementation.
* When encountering something I was not familiar with, I used Google to search for `Rust <concept>`.
  Here is the list of some of the Rust searches I did:
  * `rust command line arguments`
  * `rust constants`
  * `rust convert i32 to String`
  * `rust edition` to understand `edition` inside `Cargo.toml`.
  * `rust iterate over range`
  * `rust lang tests`
  * `rust let mut` to understand difference between mutable and immutable variables
  * `rust println format`
  * `rust return new formatted string from a function`
  * `rust single quote vs double quote`
  * `rust switch case`
  * `rust use expression in match`
* I used [Try Rust without installing](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
  to try things out without building an application.

## Implementation approach
Starting after the commit labeled "Initial commit after creating project and importing into IntelliJ IDEA",
each of the following are in their own commit.

* For doing 1 through 100, I chose to iterate over an inclusive range, using a simple if/then/else.
* I refactored `fizz_buzz_using_if_then_else` to return a `String` and added unit tests for this function.
* To learn how to use `match`, I added `fizz_buzz_using_match`. Adding the tests via copy/paste would
  violate DRY so I added the test helper `verify_fizz_buzz_function` which takes the function under test
  as an argument.
* To learn how to use command line arguments, I added support for providing the `range_end` value
  on the command line.

## Executing instructions
* Run `cargo run` to run with the default end value of 100.
* Run `cargo run -- <value>` to run with another end value, replacing `<value>` with the end value.
* Run `cargo test` to run all of the tests.
