//! # cssesc — escape a string for CSS
//!
//! Escape a string so it can be used as a CSS string or identifier (selector). A faithful
//! Rust port of the widely-used [`cssesc`](https://www.npmjs.com/package/cssesc) npm package
//! by Mathias Bynens.
//!
//! ```
//! use cssesc::cssesc;
//!
//! assert_eq!(cssesc("Lady Bird"), "Lady Bird");
//! assert_eq!(cssesc("café"), "caf\\E9");
//! assert_eq!(cssesc("'quotes'"), "\\'quotes\\'");
//! ```
//!
//! Use [`cssesc_with`] / [`Options`] to escape an identifier, choose the quote style, wrap
//! the output, or escape every character:
//!
//! ```
//! use cssesc::{cssesc_with, Options};
//!
//! assert_eq!(cssesc_with("foo.bar", &Options::new().is_identifier(true)), "foo\\.bar");
//! assert_eq!(cssesc_with("1up", &Options::new().is_identifier(true)), "\\31up");
//! ```
//!
//! **Zero dependencies** and `#![no_std]`.

#![no_std]
#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/cssesc/0.1.0")]
#![allow(clippy::format_push_string)]

extern crate alloc;

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// The quote style to escape for (and wrap with).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Quotes {
    /// Single quotes (`'`).
    Single,
    /// Double quotes (`"`).
    Double,
}

/// Options for [`cssesc_with`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Options {
    quotes: Option<Quotes>,
    wrap: bool,
    escape_everything: bool,
    is_identifier: bool,
}

impl Options {
    /// Default options (single quotes, escape as a CSS string, no wrapping).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Which quote character to escape (and wrap with). Defaults to [`Quotes::Single`].
    #[must_use]
    pub fn quotes(mut self, quotes: Quotes) -> Self {
        self.quotes = Some(quotes);
        self
    }

    /// Wrap the output in the quote character (ignored for identifiers).
    #[must_use]
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Escape every character, including printable ASCII.
    #[must_use]
    pub fn escape_everything(mut self, value: bool) -> Self {
        self.escape_everything = value;
        self
    }

    /// Escape the string as a CSS identifier (selector) rather than a string value.
    #[must_use]
    pub fn is_identifier(mut self, value: bool) -> Self {
        self.is_identifier = value;
        self
    }
}

/// Escape `string` as a CSS string using the default options.
///
/// ```
/// # use cssesc::cssesc;
/// assert_eq!(cssesc("a\tb"), "a\\9 b");
/// ```
#[must_use]
pub fn cssesc(string: &str) -> String {
    cssesc_with(string, &Options::new())
}

/// Escape `string` with the given [`Options`].
#[must_use]
pub fn cssesc_with(string: &str, options: &Options) -> String {
    let quote = match options.quotes.unwrap_or(Quotes::Single) {
        Quotes::Single => '\'',
        Quotes::Double => '"',
    };
    let is_identifier = options.is_identifier;

    let first_char = string.chars().next();
    let mut output = String::with_capacity(string.len());

    for character in string.chars() {
        let code_point = u32::from(character);
        if !(0x20..=0x7E).contains(&code_point) {
            // Not a printable ASCII character: emit a `\HEX ` escape.
            output.push_str(&hex_escape(code_point));
        } else if options.escape_everything {
            if any_single_escape(code_point) {
                output.push('\\');
                output.push(character);
            } else {
                output.push_str(&hex_escape(code_point));
            }
        } else if character == '\\'
            || (!is_identifier && is_active_quote(character, quote))
            || (is_identifier && single_escape(code_point))
        {
            output.push('\\');
            output.push(character);
        } else {
            output.push(character);
        }
    }

    if is_identifier {
        if starts_with_dash_then_dash_or_digit(&output) {
            output = format!("\\-{}", &output['-'.len_utf8()..]);
        } else if let Some(first) = first_char.filter(char::is_ascii_digit) {
            output = format!("\\3{first} {}", &output[first.len_utf8()..]);
        }
    }

    output = remove_excessive_spaces(&output);

    if !is_identifier && options.wrap {
        let mut wrapped = String::with_capacity(output.len() + 2);
        wrapped.push(quote);
        wrapped.push_str(&output);
        wrapped.push(quote);
        return wrapped;
    }
    output
}

/// `\HEX ` — a backslash, the uppercase hex code point, and a trailing space.
fn hex_escape(code_point: u32) -> String {
    format!("\\{code_point:X} ")
}

fn is_active_quote(character: char, quote: char) -> bool {
    (character == '"' && quote == '"') || (character == '\'' && quote == '\'')
}

/// Punctuation that may be backslash-escaped in `escapeEverything` mode
/// (the reference's `regexAnySingleEscape`).
fn any_single_escape(code_point: u32) -> bool {
    matches!(code_point,
        0x20..=0x2C | 0x2E | 0x2F | 0x3A..=0x40 | 0x5B..=0x5E | 0x60 | 0x7B..=0x7E)
}

/// Punctuation that is backslash-escaped in identifier mode (the reference's
/// `regexSingleEscape`); like [`any_single_escape`] but excludes the backslash.
fn single_escape(code_point: u32) -> bool {
    matches!(code_point,
        0x20..=0x2C | 0x2E | 0x2F | 0x3A..=0x40 | 0x5B | 0x5D | 0x5E | 0x60 | 0x7B..=0x7E)
}

/// `/^-[-\d]/` — a leading `-` followed by `-` or a digit.
fn starts_with_dash_then_dash_or_digit(output: &str) -> bool {
    let mut chars = output.chars();
    chars.next() == Some('-') && matches!(chars.next(), Some(c) if c == '-' || c.is_ascii_digit())
}

fn is_upper_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || (b'A'..=b'F').contains(&byte)
}

fn is_any_hex(byte: u8) -> bool {
    byte.is_ascii_hexdigit()
}

/// Remove redundant spaces after `\HEX` escapes not followed by a hex digit, matching the
/// reference's `regexExcessiveSpaces`. The output of the escaping pass is always ASCII.
fn remove_excessive_spaces(string: &str) -> String {
    let bytes = string.as_bytes();
    let n = bytes.len();
    let mut out: Vec<u8> = Vec::with_capacity(n);
    let mut index = 0;
    while index < n {
        if bytes[index] == b'\\' {
            // The greedy `(\\+)?(\\…)` means all leading backslashes are consumed, with the
            // last one beginning the `\HEX` escape.
            let mut backslashes = 0;
            while index + backslashes < n && bytes[index + backslashes] == b'\\' {
                backslashes += 1;
            }
            let hex_start = index + backslashes;
            let mut hex_len = 0;
            while hex_len < 6 && hex_start + hex_len < n && is_upper_hex(bytes[hex_start + hex_len])
            {
                hex_len += 1;
            }
            let space_at = hex_start + hex_len;
            if hex_len >= 1 && space_at < n && bytes[space_at] == b' ' {
                let after = space_at + 1;
                let lookahead_ok =
                    after >= n || !(is_any_hex(bytes[after]) || bytes[after] == b' ');
                if lookahead_ok {
                    out.extend_from_slice(&bytes[index..space_at]); // backslashes + hex
                                                                    // Group 1 (`backslashes - 1`) of odd length means the escape's backslash
                                                                    // is itself escaped, so the space must stay.
                    if (backslashes - 1) % 2 == 1 {
                        out.push(b' ');
                    }
                    index = after;
                    continue;
                }
            }
        }
        out.push(bytes[index]);
        index += 1;
    }
    String::from_utf8(out).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strings() {
        assert_eq!(cssesc("Ladybird"), "Ladybird");
        assert_eq!(cssesc("Lady Bird"), "Lady Bird");
        assert_eq!(cssesc("café"), "caf\\E9");
        assert_eq!(cssesc("\u{1D306}"), "\\1D306");
        assert_eq!(cssesc("a\tb"), "a\\9 b"); // space kept (next char is a hex digit)
        assert_eq!(cssesc("\0"), "\\0");
        assert_eq!(cssesc("a\\b"), "a\\\\b");
    }

    #[test]
    fn quotes_and_wrap() {
        assert_eq!(cssesc("'quote'"), "\\'quote\\'");
        assert_eq!(
            cssesc_with("'quote'", &Options::new().quotes(Quotes::Double)),
            "'quote'"
        );
        assert_eq!(
            cssesc_with("\"dq\"", &Options::new().quotes(Quotes::Double)),
            "\\\"dq\\\""
        );
        assert_eq!(cssesc_with("foo", &Options::new().wrap(true)), "'foo'");
    }

    #[test]
    fn escape_everything() {
        assert_eq!(
            cssesc_with("ab", &Options::new().escape_everything(true)),
            "\\61\\62"
        );
    }

    #[test]
    fn identifiers() {
        let id = Options::new().is_identifier(true);
        assert_eq!(cssesc_with("-foo", &id), "-foo");
        assert_eq!(cssesc_with("--foo", &id), "\\--foo");
        assert_eq!(cssesc_with("-1", &id), "\\-1");
        assert_eq!(cssesc_with("1abc", &id), "\\31 abc");
        assert_eq!(cssesc_with("0", &id), "\\30");
        assert_eq!(cssesc_with("foo.bar", &id), "foo\\.bar");
        assert_eq!(cssesc_with("hello world", &id), "hello\\ world");
    }
}
