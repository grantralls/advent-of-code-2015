# Advent of Code 2015

_Language: Rust, Author: Grant S. Ralls ([grantralls.net](https://grantralls.net) - dev@grantralls.net)_

_Not affiliated with The Rust Foundation or The Rustlang Team in any manner_

## Introduction

This advent of code is being worked on in 2023 as a fun programming exercise to work on learning Rust.

## Requirements

-   rustc
-   cargo
-   rustup

[Installing via rustup](https://www.rust-lang.org/tools/install) works in the majority of cases

## Project Structure

The root folder is a [Rust Virtual Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html). Essentially meaning one "package" that contains many individual packages. Each individual package is a days solution and named accordingly (day-one, day-two...).

### Files (per project/day)

-   src/main.rs - binary entry
-   src/lib.rs - library (code that can be imported from other packages)
-   src/any-other-file.rs - a module
-   Cargo.toml - The project's [manifest](https://doc.rust-lang.org/cargo/reference/manifest.html)

## Running a solution

1. Change directory into the desired day
2. Run `cargo run`

## Running tests

1. Change directory into the desired day
2. Run `cargo test`
