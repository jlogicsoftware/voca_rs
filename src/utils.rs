//! Utility functions and properties.

/// A property that contains the library <a href="http://semver.org/">semantic version number</a>.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::VERSION;
/// // => "1.15.3"
/// ```
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The concatenation of the `ascii_lowercase` and `ascii_uppercase` constants described below. This value is not locale-dependent.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::ASCII_LETTERS;
/// // => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
/// ```
// TODO: make a combination of ASCII_LOWERCASE, ASCII_UPPERCASE #5
pub const ASCII_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The lowercase letters `"abcdefghijklmnopqrstuvwxyz"`. This value is not locale-dependent and will not change.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::ASCII_LOWERCASE;
/// // => "abcdefghijklmnopqrstuvwxyz"
/// ```
pub const ASCII_LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";

/// The uppercase letters `"ABCDEFGHIJKLMNOPQRSTUVWXYZ"`. This value is not locale-dependent and will not change.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::ASCII_UPPERCASE;
/// // => "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
/// ```
// TODO: make a variation of ASCII_LOWERCASE #6
pub const ASCII_UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// The string "0123456789".
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::DIGITS;
/// // => "0123456789"
/// ```
pub const DIGITS: &str = "0123456789";

/// The string `"0123456789abcdefABCDEF"`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::HEXDIGITS;
/// // => "0123456789abcdefABCDEF"
/// ```
// TODO: make a combination of slices of DIGITS, ASCII_LOWERCASE, ASCII_UPPERCASE #7
pub const HEXDIGITS: &str = "0123456789abcdefABCDEF";

/// The string `"01234567"`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::OCTDIGITS;
/// // => "01234567"
/// ```
// TODO: make a slice of DIGITS #8
pub const OCTDIGITS: &str = "01234567";

/// The string `!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::PUNCTUATION;
/// // => !"#$%&'()*+,-./:;<=>?@[\]^_`{|}~
/// ```
pub const PUNCTUATION: &str = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

/// String of ASCII characters which are considered printable. This is a combination of `digits`, `ascii_letters`, `punctuation`, and `whitespace`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::PRINTABLE;
/// // => 0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r
/// ```
// TODO: make a combination of DIGITS, ASCII_LETTERS, PUNCTUATION, WHITESPACE #9
pub const PRINTABLE: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~ \t\n\r";

/// The string `" \t\n\r"`.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::WHITESPACE;
/// // => " \t\n\r"
/// ```
pub const WHITESPACE: &str = " \t\n\r";

/// Takes any Unicode string and returns an ASCII transliteration
/// of that string.
///
/// Delegates to the [`unidecode`](https://crates.io/crates/unidecode) crate.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::unidecode("Æneid");
/// // => "AEneid"
/// ```
pub fn unidecode(s: &str) -> String {
    ::unidecode::unidecode(s)
}

/// Takes a single Unicode character and returns an ASCII
/// transliteration.
///
/// Delegates to the [`unidecode`](https://crates.io/crates/unidecode) crate.
///
/// # Example
///
/// ```
/// use voca_rs::*;
/// utils::unidecode_char('Æ');
/// // => "AE"
/// ```
#[inline]
pub fn unidecode_char(ch: char) -> &'static str {
    ::unidecode::unidecode_char(ch)
}
