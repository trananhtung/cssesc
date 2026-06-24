# cssesc

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

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

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/./commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
