//! Integration tests exercising the public API of `cssesc`.

use cssesc::{cssesc, cssesc_with, Options, Quotes};

#[test]
fn string_values() {
    assert_eq!(cssesc("Hello world!"), "Hello world!");
    assert_eq!(cssesc("a\nb"), "a\\A b");
    assert_eq!(cssesc("emoji 😀"), "emoji \\1F600");
}

#[test]
fn class_name_identifiers() {
    let id = Options::new().is_identifier(true);
    assert_eq!(cssesc_with("my-class", &id), "my-class");
    assert_eq!(cssesc_with("2cool", &id), "\\32 cool");
    assert_eq!(cssesc_with("a:hover", &id), "a\\:hover");
    assert_eq!(cssesc_with("--custom-prop", &id), "\\--custom-prop");
}

#[test]
fn quote_choices() {
    assert_eq!(cssesc("it's"), "it\\'s");
    assert_eq!(cssesc_with("it's", &Options::new().quotes(Quotes::Double)), "it's");
    assert_eq!(cssesc_with("say \"hi\"", &Options::new().quotes(Quotes::Double).wrap(true)), "\"say \\\"hi\\\"\"");
}

#[test]
fn escape_everything() {
    assert_eq!(cssesc_with("a1", &Options::new().escape_everything(true)), "\\61\\31");
}
