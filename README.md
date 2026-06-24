# cssesc

[![crates.io](https://img.shields.io/crates/v/cssesc.svg)](https://crates.io/crates/cssesc)
[![docs.rs](https://docs.rs/cssesc/badge.svg)](https://docs.rs/cssesc)
[![CI](https://github.com/trananhtung/cssesc/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/cssesc/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/cssesc.svg)](#license)

**Escape a string for use as a CSS string or identifier (selector).**

A faithful Rust port of the widely-used [`cssesc`](https://www.npmjs.com/package/cssesc) npm
package by Mathias Bynens.

- **Zero dependencies**, **`#![no_std]`**
- String or identifier mode, single/double quotes, wrapping, escape-everything
- Differential-tested against the reference `cssesc` implementation (60k cases, all options)

## Install

```toml
[dependencies]
cssesc = "0.1"
```

## Usage

```rust
use cssesc::{cssesc, cssesc_with, Options, Quotes};

assert_eq!(cssesc("Lady Bird"), "Lady Bird");
assert_eq!(cssesc("café"), "caf\\E9");
assert_eq!(cssesc("'quotes'"), "\\'quotes\\'");

// Identifier (selector) mode escapes more, and fixes leading digits/dashes:
let id = Options::new().is_identifier(true);
assert_eq!(cssesc_with("foo.bar", &id), "foo\\.bar");
assert_eq!(cssesc_with("1up", &id), "\\31up");

// Choose quotes and wrap:
assert_eq!(cssesc_with("hi", &Options::new().quotes(Quotes::Double).wrap(true)), "\"hi\"");
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
