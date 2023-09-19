# esi-rust
ESI library for the Rust programming language

## Project Status
Pre-Pre-Alpha

This software is not yet in a usable state. Feel free to utilise code however please note that large sections of functionality are missing.

## Project Objectives
To implement a fast, safe library for interacting with the Eve Online ESI API in the Rust programming language.
The library should handle HTTP interaction, user authentication, and data formatting for output and use by other software.

## Installation
This library is not currently available on crates.io due to its in-development state.
To install and use, clone this git repository, install Rust and Cargo, and run 'cargo build' on the directory.
You should then be able to link to it through the file system.
More detailed documentation will be produced through the development process although I am attempting to document functions as I go.
Use 'cargo doc --no-deps --open' to view the documentation and use 'cargo test' to run the suite of unit tests.

## Distribution and Support
This software will be eventually made available through crates.io (the Rust package repository).
It should run on every platform with first-class Rust support, although the developer uses Linux so Windows and Mac bugs may take a bit longer to fix.
Users from those platforms are highly encouraged to submit their own patches as they will likely be able to fix them faster than me.

## Code of Conduct for users
Aside from the obvious of not using this library for anything that will get you (or me) banned from Eve, contributors to the project should follow the [Linux kernel Contributor Covenant](https://docs.kernel.org/process/code-of-conduct.html).

