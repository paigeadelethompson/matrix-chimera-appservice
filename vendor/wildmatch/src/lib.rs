//! Match strings against a simple wildcard pattern.
//! Tests a wildcard pattern `p` against an input string `s`. Returns true only when `p` matches the entirety of `s`.
//!
//! See also the example described on [wikipedia](https://en.wikipedia.org/wiki/Matching_wildcards) for matching wildcards.
//!
//! No escape characters are defined.
//!
//! - `?` matches exactly one occurrence of any character.
//! - `*` matches arbitrary many (including zero) occurrences of any character.
//!
//! Examples matching wildcards:
//! ``` rust
//! # extern crate wildmatch; use wildmatch::WildMatch;
//! assert!(WildMatch::new("cat").matches("cat"));
//! assert!(WildMatch::new("*cat*").matches("dog_cat_dog"));
//! assert!(WildMatch::new("c?t").matches("cat"));
//! assert!(WildMatch::new("c?t").matches("cot"));
//! ```
//! Examples not matching wildcards:
//! ``` rust
//! # extern crate wildmatch; use wildmatch::WildMatch;
//! assert!(!WildMatch::new("dog").matches("cat"));
//! assert!(!WildMatch::new("*d").matches("cat"));
//! assert!(!WildMatch::new("????").matches("cat"));
//! assert!(!WildMatch::new("?").matches("cat"));
//! ```
//!
//! You can specify custom `char` values for the single and multi-character
//! wildcards. For example, to use `%` as the multi-character wildcard and
//! `_` as the single-character wildcard:
//! ```rust
//! # extern crate wildmatch; use wildmatch::WildMatchPattern;
//! assert!(WildMatchPattern::<'%', '_'>::new("%cat%").matches("dog_cat_dog"));
//! ```

use std::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A wildcard matcher using `*` as the multi-character wildcard and `?` as
/// the single-character wildcard.
pub type WildMatch = WildMatchPattern<'*', '?'>;

/// Wildcard matcher used to match strings.
///
/// `MULTI_WILDCARD` is the character used to represent a
/// multiple-character wildcard (e.g., `*`), and `SINGLE_WILDCARD` is the
/// character used to represent a single-character wildcard (e.g., `?`).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct WildMatchPattern<const MULTI_WILDCARD: char, const SINGLE_WILDCARD: char> {
    pattern: Vec<char>,
}

impl<const MULTI_WILDCARD: char, const SINGLE_WILDCARD: char> fmt::Display
    for WildMatchPattern<MULTI_WILDCARD, SINGLE_WILDCARD>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use std::fmt::Write;
        for c in &self.pattern {
            f.write_char(*c)?;
        }
        Ok(())
    }
}

impl<const MULTI_WILDCARD: char, const SINGLE_WILDCARD: char>
    WildMatchPattern<MULTI_WILDCARD, SINGLE_WILDCARD>
{
    /// Constructor with pattern which can be used for matching.
    pub fn new(pattern: &str) -> WildMatchPattern<MULTI_WILDCARD, SINGLE_WILDCARD> {
        let mut simplified: Vec<char> = pattern.chars().collect();
        let mut new_len = simplified.len();
        let mut wildcard_count = 0;

        for idx in (0..simplified.len()).rev() {
            if simplified[idx] == MULTI_WILDCARD {
                wildcard_count += 1;
            } else {
                if wildcard_count > 1 {
                    new_len -= wildcard_count - 1;
                    simplified[idx + 1..].rotate_left(wildcard_count - 1);
                }
                wildcard_count = 0;
            }
        }
        if wildcard_count > 1 {
            new_len -= wildcard_count - 1;
            simplified.rotate_left(wildcard_count - 1);
        }

        simplified.truncate(new_len);

        Self {
            pattern: simplified,
        }
    }

    #[deprecated(since = "2.0.0", note = "use `matches` instead")]
    pub fn is_match(&self, input: &str) -> bool {
        self.matches(input)
    }

    /// Returns true if pattern applies to the given input string
    pub fn matches(&self, input: &str) -> bool {
        if self.pattern.is_empty() {
            return input.is_empty();
        }
        let mut input_chars = input.chars();

        let mut pattern_idx = 0;
        if let Some(mut input_char) = input_chars.next() {
            const NONE: usize = usize::MAX;
            let mut start_idx = NONE;
            let mut matched = "".chars();

            loop {
                if pattern_idx < self.pattern.len()
                    && self.pattern[pattern_idx] == MULTI_WILDCARD
                {
                    start_idx = pattern_idx;
                    matched = input_chars.clone();
                    pattern_idx += 1;
                } else if pattern_idx < self.pattern.len()
                    && (self.pattern[pattern_idx] == SINGLE_WILDCARD
                        || self.pattern[pattern_idx] == input_char)
                {
                    pattern_idx += 1;
                    if let Some(next_char) = input_chars.next() {
                        input_char = next_char;
                    } else {
                        break;
                    }
                } else if start_idx != NONE {
                    pattern_idx = start_idx + 1;
                    if let Some(next_char) = matched.next() {
                        input_char = next_char;
                    } else {
                        break;
                    }
                    input_chars = matched.clone();
                } else {
                    return false;
                }
            }
        }

        while pattern_idx < self.pattern.len() && self.pattern[pattern_idx] == MULTI_WILDCARD {
            pattern_idx += 1;
        }

        // If we have reached the end of both the pattern and the text, the pattern matches the text.
        return pattern_idx == self.pattern.len();
    }
}

impl<'a, const MULTI_WILDCARD: char, const SINGLE_WILDCARD: char> PartialEq<&'a str>
    for WildMatchPattern<MULTI_WILDCARD, SINGLE_WILDCARD>
{
    fn eq(&self, &other: &&'a str) -> bool {
        self.matches(other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ntest::assert_false;
    use ntest::test_case;
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn is_match_random() {
        const PATTERN_LEN: usize = 100;

        for _ in 0..1_000 {
            let mut rng = rand::thread_rng();
            let mut pattern: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(PATTERN_LEN)
                .map(char::from)
                .collect();
            for _ in 0..rng.gen_range(0..15) {
                let idx = rng.gen_range(0..PATTERN_LEN);
                pattern.replace_range(idx..idx + 1, "?")
            }
            for _ in 0..rng.gen_range(0..15) {
                let idx = rng.gen_range(0..PATTERN_LEN);
                pattern.replace_range(idx..idx + 1, "*")
            }
            let m = WildMatch::new(&pattern);
            for pattern_idx in 0..rng.gen_range(0..1_000) {
                let mut input = pattern.clone();
                for (i, c) in pattern.chars().rev().enumerate() {
                    let idx = pattern.len() - i - 1;
                    if c == '?' {
                        let rand_char: String = rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(1)
                            .map(char::from)
                            .collect();
                        input.replace_range(idx..idx + 1, &rand_char)
                    }
                    if c == '*' {
                        let rand_char: String = rand::thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(rng.gen_range(0..15))
                            .map(char::from)
                            .collect();
                        input.replace_range(idx..idx + 1, &rand_char)
                    }
                }
                assert!(
                    m.matches(&input),
                    "Pattern ({}): {} doesn't match input: {}",
                    pattern_idx,
                    pattern,
                    input
                );
            }
        }
    }

    #[test_case("**")]
    #[test_case("*")]
    #[test_case("*?*")]
    #[test_case("c*")]
    #[test_case("c?*")]
    #[test_case("???")]
    #[test_case("c?t")]
    #[test_case("cat")]
    #[test_case("*cat")]
    #[test_case("cat*")]
    fn is_match(pattern: &str) {
        let m = WildMatch::new(pattern);
        assert!(m.matches("cat"));
    }

    #[test_case("*d*")]
    #[test_case("*d")]
    #[test_case("d*")]
    #[test_case("*c")]
    #[test_case("?")]
    #[test_case("??")]
    #[test_case("????")]
    #[test_case("?????")]
    #[test_case("*????")]
    #[test_case("cats")]
    #[test_case("cat?")]
    #[test_case("cacat")]
    #[test_case("cat*dog")]
    fn no_match(pattern: &str) {
        let m = WildMatch::new(pattern);
        assert_false!(m.matches("cat"));
    }

    #[test_case("1", "")]
    #[test_case("?", "")]
    #[test_case("?", "11")]
    #[test_case("*1?", "123")]
    #[test_case("*12", "122")]
    #[test_case("cat?", "wildcats")]
    #[test_case("cat*", "wildcats")]
    #[test_case("*x*", "wildcats")]
    #[test_case("*a", "wildcats")]
    #[test_case("", "wildcats")]
    #[test_case(" ", "wildcats")]
    #[test_case(" ", "\n")]
    #[test_case(" ", "\t", name = "whitespaceMismatch")]
    #[test_case("???", "wildcats")]
    fn no_match_long(pattern: &str, expected: &str) {
        let m = WildMatch::new(pattern);
        assert_false!(m.matches(expected))
    }

    #[test_case("*", "")]
    #[test_case("*", "1")]
    #[test_case("?", "1")]
    #[test_case("*121", "12121")]
    #[test_case("?*3", "111333")]
    #[test_case("*113", "1113")]
    #[test_case("*113", "113")]
    #[test_case("*113", "11113")]
    #[test_case("*113", "111113")]
    #[test_case("*???a", "bbbba")]
    #[test_case("*???a", "bbbbba")]
    #[test_case("*???a", "bbbbbba")]
    #[test_case("*o?a*", "foobar")]
    #[test_case("*ooo?ar", "foooobar")]
    #[test_case("*o?a*r", "foobar")]
    #[test_case("*cat*", "d&(*og_cat_dog")]
    #[test_case("*?*", "d&(*og_cat_dog")]
    #[test_case("*a*", "d&(*og_cat_dog")]
    #[test_case("a*b", "a*xb")]
    #[test_case("*", "*")]
    #[test_case("*", "?")]
    #[test_case("?", "?")]
    #[test_case("wildcats", "wildcats")]
    #[test_case("wild*cats", "wild?cats")]
    #[test_case("wi*ca*s", "wildcats")]
    #[test_case("wi*ca?s", "wildcats")]
    #[test_case("*o?", "hog_cat_dog")]
    #[test_case("*o?", "cat_dog")]
    #[test_case("*at_dog", "cat_dog")]
    #[test_case(" ", " ")]
    #[test_case("* ", "\n ")]
    #[test_case("\n", "\n", name = "special_chars")]
    #[test_case("*32", "432")]
    #[test_case("*32", "332")]
    #[test_case("*332", "332")]
    #[test_case("*32", "32")]
    #[test_case("*32", "3232")]
    #[test_case("*32", "3232332")]
    #[test_case("*?2", "332")]
    #[test_case("*?2", "3332")]
    #[test_case("33*", "333")]
    #[test_case("da*da*da*", "daaadabadmanda")]
    #[test_case("*?", "xx")]
    fn match_long(pattern: &str, expected: &str) {
        let m = WildMatch::new(pattern);
        assert!(
            m.matches(expected),
            "Expected pattern {} to match {}",
            pattern,
            expected
        );
    }

    #[test]
    fn complex_pattern() {
        const TEXT: &str = "Lorem ipsum dolor sit amet, \
        consetetur sadipscing elitr, sed diam nonumy eirmod tempor \
        invidunt ut labore et dolore magna aliquyam erat, sed diam \
        voluptua. At vero eos et accusam et justo duo dolores et ea \
        rebum. Stet clita kasd gubergren, no sea takimata sanctus est \
        Lorem ipsum dolor sit amet.";
        const COMPLEX_PATTERN: &str = "Lorem?ipsum*dolore*ea* ?????ata*.";
        let m = WildMatch::new(COMPLEX_PATTERN);
        assert!(m.matches(TEXT));
    }

    #[test]
    fn complex_pattern_alternative_wildcards() {
        const TEXT: &str = "Lorem ipsum dolor sit amet, \
        consetetur sadipscing elitr, sed diam nonumy eirmod tempor \
        invidunt ut labore et dolore magna aliquyam erat, sed diam \
        voluptua. At vero eos et accusam et justo duo dolores et ea \
        rebum. Stet clita kasd gubergren, no sea takimata sanctus est \
        Lorem ipsum dolor sit amet.";
        const COMPLEX_PATTERN: &str = "Lorem_ipsum%dolore%ea% _____ata%.";
        let m = WildMatchPattern::<'%', '_'>::new(COMPLEX_PATTERN);
        assert!(m.matches(TEXT));
    }

    #[test]
    fn compare_via_equal() {
        let m = WildMatch::new("c?*");
        assert!(m == "cat");
        assert!(m == "car");
        assert!(m != "dog");
    }

    #[test]
    fn compare_empty() {
        let m: WildMatch = WildMatch::new("");
        assert!(m != "bar");
        assert!(m == "");
    }

    #[test]
    fn compare_default() {
        let m: WildMatch = Default::default();
        assert!(m == "");
        assert!(m != "bar");
    }

    #[test]
    fn compare_wild_match() {
        assert_eq!(WildMatch::default(), WildMatch::new(""));
        assert_eq!(WildMatch::new("abc"), WildMatch::new("abc"));
        assert_eq!(WildMatch::new("a*bc"), WildMatch::new("a*bc"));
        assert_ne!(WildMatch::new("abc"), WildMatch::new("a*bc"));
        assert_ne!(WildMatch::new("a*bc"), WildMatch::new("a?bc"));
        assert_eq!(WildMatch::new("a***c"), WildMatch::new("a*c"));
    }

    #[test]
    fn print_string() {
        let m = WildMatch::new("Foo/Bar");
        assert_eq!("Foo/Bar", m.to_string());
    }

    #[test]
    fn to_string_f() {
        let m = WildMatch::new("F");
        assert_eq!("F", m.to_string());
    }

    #[test]
    fn to_string_with_star() {
        assert_eq!("a*bc", WildMatch::new("a*bc").to_string());
        assert_eq!("a*bc", WildMatch::new("a**bc").to_string());
        assert_eq!("a*bc*", WildMatch::new("a*bc*").to_string());
    }

    #[test]
    fn to_string_with_question_sign() {
        assert_eq!("a?bc", WildMatch::new("a?bc").to_string());
        assert_eq!("a??bc", WildMatch::new("a??bc").to_string());
    }

    #[test]
    fn to_string_empty() {
        let m = WildMatch::new("");
        assert_eq!("", m.to_string());
    }
}
