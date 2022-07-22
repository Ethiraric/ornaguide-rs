use std::str::FromStr;

use itertools::Itertools;
use ornaguide_rs::error::Error;

use crate::filter::Filter;

use regex::Regex;

pub trait Compilable<'a, T> {
    /// If the filter is an expression one, "compile" it to a more efficient representation.
    /// Parse the expression and create a closure from it.
    fn compiled(self) -> Result<Filter<'a, T>, Error>;
}

/// If the filter is an expression one, "compile" it to a more efficient representation.
/// Parse the expression and create a closure from it.
pub fn compile_from_str<'a, T: 'a>(str: &str) -> Result<Filter<'a, T>, Error>
where
    T: FromStr + std::cmp::PartialOrd,
    <T as FromStr>::Err: ToString,
{
    // The expression must have at least 2 chars if `<` or `>`, 3 otherwise.
    if str.len() < 2 || (str.chars().nth(1).unwrap() == '=' && str.len() < 3) {
        Err(Error::Misc(format!("Expression is too short: '{}'", str)))
    } else if str.chars().nth(1).unwrap() == '=' {
        // If we have a 2 chars operator, parse value starting from 3rd char.
        let expected_value = T::from_str(&str[2..]).map_err(|s| Error::Misc(s.to_string()))?;

        // Match the first char and create a closure accordingly.
        match str.chars().next() {
            Some('=') => Ok(Filter::Compiled(Box::new(move |a| *a == expected_value))),
            Some('!') => Ok(Filter::Compiled(Box::new(move |a| *a != expected_value))),
            Some('>') => Ok(Filter::Compiled(Box::new(move |a| *a >= expected_value))),
            Some('<') => Ok(Filter::Compiled(Box::new(move |a| *a <= expected_value))),
            // Error on weird operators (`,=` would be one).
            Some(_) => Err(Error::Misc(format!(
                "Invalid operator in expression: {}",
                str
            ))),
            // Error if somehow we fail to get the first char.
            None => Err(Error::Misc(format!(
                "Failed to get the first character of the expression '{}'",
                str
            ))),
        }
    } else {
        // If we have a 1 char operator, parse value starting from 2nd char.
        let expected_value = T::from_str(&str[1..]).map_err(|s| Error::Misc(s.to_string()))?;

        // Match the first char and create a closure accordingly.
        match str.chars().next() {
            Some('>') => Ok(Filter::Compiled(Box::new(move |a| *a > expected_value))),
            Some('<') => Ok(Filter::Compiled(Box::new(move |a| *a < expected_value))),
            // Error on weird operators (`,` would be one).
            Some(_) => Err(Error::Misc(format!(
                "Invalid operator in expression: {}",
                str
            ))),
            // Error if somehow we fail to get the first char.
            None => Err(Error::Misc(format!(
                "Failed to get the first character of the expression '{}'",
                str
            ))),
        }
    }
}

macro_rules! compilable_scalar {
    ($ty:ident) => {
        impl<'a> Compilable<'a, $ty> for Filter<'a, $ty> {
            fn compiled(self) -> Result<Filter<'a, $ty>, Error> {
                match self {
                    // If we have an expression, rewrite it.
                    Filter::Expr(str) => compile_from_str::<$ty>(&str),
                    // If we don't have an expression, we don't need to transform `self`.
                    _ => Ok(self),
                }
            }
        }
    };
}

compilable_scalar!(i8);
compilable_scalar!(i16);
compilable_scalar!(i32);
compilable_scalar!(i64);
compilable_scalar!(u8);
compilable_scalar!(u16);
compilable_scalar!(u32);
compilable_scalar!(u64);
compilable_scalar!(f32);
compilable_scalar!(f64);

impl<'a> Compilable<'a, bool> for Filter<'a, bool> {
    fn compiled(self) -> Result<Filter<'a, bool>, Error> {
        match self {
            Filter::Expr(_) => Err(Error::Misc(
                "Cannot use expressions with booleans".to_string(),
            )),
            _ => Ok(self),
        }
    }
}

impl<'a> Compilable<'a, String> for Filter<'a, String> {
    fn compiled(self) -> Result<Filter<'a, String>, Error> {
        match self {
            Filter::Expr(str) => {
                // If the string starts with `==`, we want equality.
                if let Some(str) = str.strip_prefix("==") {
                    Ok(Filter::Value(str.to_string()))
                // If the string starts with a `/`, that's a regex.
                } else if let Some(str) = str.strip_prefix('/') {
                    let regex = Regex::new(&format!("(?i){}", str)).map_err(|e| {
                        Error::Misc(format!(
                            "Failed to create regular expression: '{}': {}",
                            &str[1..],
                            e
                        ))
                    })?;
                    Ok(Filter::Compiled(Box::new(move |a| regex.is_match(a))))
                // Otherwise, do some `contains`.
                } else {
                    let words = str.split(' ').map(str::to_lowercase).collect_vec();
                    Ok(Filter::Compiled(Box::new(move |a| {
                        words
                            .iter()
                            .map(|word| case_insensitive_contains(a, word))
                            .all(|ok| ok)
                    })))
                }
            }
            _ => Ok(self),
        }
    }
}

macro_rules! compilable_option {
    ($ty:ident) => {
        impl<'a> Compilable<'a, Option<$ty>> for Filter<'a, Option<$ty>> {
            fn compiled(self) -> Result<Filter<'a, Option<$ty>>, Error> {
                match self {
                    // If we have an expression, rewrite it.
                    Filter::Expr(str) => {
                        if &str == "<none>" {
                            Ok(Filter::Value(None))
                        } else {
                            match Filter::<'a, $ty>::Expr(str).compiled()? {
                                Filter::<'a, $ty>::Compiled(f) => {
                                    Ok(Filter::Compiled(Box::new(move |a| {
                                        a.as_ref().map(|x| (f)(x)).unwrap_or(false)
                                    })))
                                }
                                _ => panic!("Option wrappee didn't compile"),
                            }
                        }
                    }
                    // If we don't have an expression, we don't need to transform `self`.
                    _ => Ok(self),
                }
            }
        }
    };
}

compilable_option!(i8);
compilable_option!(i16);
compilable_option!(i32);
compilable_option!(i64);
compilable_option!(u8);
compilable_option!(u16);
compilable_option!(u32);
compilable_option!(u64);
compilable_option!(f32);
compilable_option!(f64);
compilable_option!(String);

/// Compare 2 strings, one of which is lowercase, case insensitively.
/// The haystack need not be lowercase. The needle must be lowercase.
///
/// Examples:
/// ```
/// assert_eq!(case_insensitive_contains("haystack", "stack"));
/// assert_eq!(case_insensitive_contains("Haystack", "stack"));
/// assert_eq!(case_insensitive_contains("haySTACK", "stack"));
/// ```
fn case_insensitive_contains(not_lowercase_haystack: &str, lowercase_needle: &str) -> bool {
    // Make a lowercase iterator from the haystack.
    let mut it = not_lowercase_haystack
        .chars()
        .map(|c| c.to_ascii_lowercase());

    // Get the first character from the needle.
    let first_needle = match lowercase_needle.chars().next() {
        Some(c) => c,
        None => return true,
    };

    // Iterate through that iterator.
    while let Some(c) = it.next() {
        // If the first letter matches the first letter of our needle, ...
        if c == first_needle
            && it
                .clone()
                .take(lowercase_needle.len() - 1)
                // ... compare with the rest of the needle.
                .eq(lowercase_needle.chars().skip(1))
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use crate::filter::compilable::case_insensitive_contains;

    #[test]
    fn test_with_all_lowercase() {
        assert!(case_insensitive_contains("haystack", "stack"));
        assert!(case_insensitive_contains("haystack", "s"));
        assert!(!case_insensitive_contains("haystack", "r"));
        assert!(!case_insensitive_contains("haystack", "stacke"));
        assert!(!case_insensitive_contains("haystack", "stacr"));
    }

    #[test]
    fn test_with_all_both_cases() {
        assert!(case_insensitive_contains("Haystack", "stack"));
        assert!(case_insensitive_contains("HayStack", "stack"));
        assert!(case_insensitive_contains("haystack", "s"));
        assert!(case_insensitive_contains("hayStack", "s"));
        assert!(!case_insensitive_contains("haystack", "r"));
        assert!(!case_insensitive_contains("hayStack", "stacke"));
        assert!(!case_insensitive_contains("haystaSk", "stacr"));
    }
}
