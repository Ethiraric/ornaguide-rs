use std::str::FromStr;

use itertools::Itertools;
use ornaguide_rs::{
    error::{Error as OError, Kind},
    pets::admin::CostType,
};

use crate::{
    error::{Error, ToErrorable},
    filter::Filter,
};

use regex::Regex;

pub trait Compilable<'a, T> {
    /// If the filter is an expression one, "compile" it to a more efficient representation.
    /// Parse the expression and create a closure from it.
    fn compiled(self) -> Result<Filter<'a, T>, Error>;
}

/// If the filter is an expression one, "compile" it to a more efficient representation.
/// Parse the expression and create a closure from it.
pub fn compile_from_str<'a, T>(str: &str) -> Result<Filter<'a, T>, Error>
where
    T: 'a + FromStr + std::cmp::PartialOrd,
    <T as FromStr>::Err: ToString,
{
    let result = (|| -> Result<Filter<'a, T>, OError> {
        // The expression must have at least 2 chars if `<` or `>`, 3 if `<=`, `>=`, `==` or `!=`, 4 if
        // `|[x]` or `^[x]`.
        if str.len() < 2
            || (str.chars().nth(1).unwrap() == '=' && str.len() < 3)
            || ("|^".contains(str.chars().next().unwrap()) && str.len() < 4)
        {
            return Err(Kind::Misc(format!("Expression is too short: '{str}'")).into());
        }

        let first_char = str.chars().next().unwrap();
        let second_char = str.chars().nth(1).unwrap();

        // Parse a `<=`, `>=`, `==` or `!=` expression.
        if second_char == '=' {
            // If we have a 2 chars operator, parse value starting from 3rd char.
            let expected_value = T::from_str(&str[2..]).map_err(|s| Kind::Misc(s.to_string()))?;

            // Match the first char and create a closure accordingly.
            match first_char {
                '=' => Ok(Filter::Compiled(Box::new(move |a| *a == expected_value))),
                '!' => Ok(Filter::Compiled(Box::new(move |a| *a != expected_value))),
                '>' => Ok(Filter::Compiled(Box::new(move |a| *a >= expected_value))),
                '<' => Ok(Filter::Compiled(Box::new(move |a| *a <= expected_value))),
                // Error on weird operators (`,=` would be one).
                _ => Err(Kind::Misc(format!("Invalid operator in expression: {str}")).into()),
            }
        // Parse a `<` or `>` expression.
        } else if "><".contains(first_char) {
            // If we have a 1 char operator, parse value starting from 2nd char.
            let expected_value = T::from_str(&str[1..]).map_err(|s| Kind::Misc(s.to_string()))?;

            // Match the first char and create a closure accordingly.
            match str.chars().next() {
                Some('>') => Ok(Filter::Compiled(Box::new(move |a| *a > expected_value))),
                Some('<') => Ok(Filter::Compiled(Box::new(move |a| *a < expected_value))),
                // Error on weird operators (`,` would be one).
                Some(_) => Err(Kind::Misc(format!("Invalid operator in expression: {str}")).into()),
                // Error if somehow we fail to get the first char.
                None => Err(Kind::Misc(format!(
                    "Failed to get the first character of the expression '{str}'"
                ))
                .into()),
            }
        // Parse a `|[x]` or `^[x]` expression.
        } else if "|^".contains(first_char) {
            let (match_type, values) = parse_array_filter_of::<T>(str)?;

            match match_type {
                VecMatch::OneOf => Ok(Filter::Compiled(Box::new(move |a| values.contains(a)))),
                VecMatch::None => Ok(Filter::Compiled(Box::new(move |a| !values.contains(a)))),
                _ => {
                    Err(Kind::Misc(format!("Logic error: Unknown array expression: {str}")).into())
                }
            }
        } else {
            Err(Kind::Misc(format!("Unknown expression: {str}")).into())
        }
    })();

    result.to_bad_request()
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
            Filter::Expr(_) => {
                Err(Kind::Misc("Cannot use expressions with booleans".to_string()).into_err())
                    .to_bad_request()
            }
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
                    let regex = Regex::new(&format!("(?i){str}"))
                        .map_err(|e| {
                            Kind::Misc(format!(
                                "Failed to create regular expression: '{}': {}",
                                &str[1..],
                                e
                            ))
                            .into_err()
                        })
                        .to_bad_request()?;
                    Ok(Filter::Compiled(Box::new(move |a| regex.is_match(a))))
                // Otherwise, do some `contains`.
                } else {
                    let words = str.split(' ').map(str::to_lowercase).collect_vec();
                    Ok(Filter::Compiled(Box::new(move |a| {
                        words.iter().all(|word| case_insensitive_contains(a, word))
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
                        } else if &str == "!=<none>" {
                            Ok(Filter::Compiled(Box::new(move |a| a.is_some())))
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

enum VecMatch {
    Exact,
    All,
    OneOf,
    None,
}

macro_rules! compilable_vec {
    ($ty:ident) => {
        impl<'a> Compilable<'a, Vec<$ty>> for Filter<'a, Vec<$ty>> {
            fn compiled(self) -> Result<Filter<'a, Vec<$ty>>, Error> {
                match self {
                    // If we have an expression, rewrite it.
                    Filter::Expr(str) => {
                        let (match_style, values) =
                            parse_array_filter_of::<$ty>(&str).to_bad_request()?;

                        match match_style {
                            VecMatch::Exact => Ok(Filter::Compiled(Box::new(move |a| {
                                a.len() == values.len()
                                    && values.iter().map(|x| a.contains(x)).all(|x| x)
                            }))),
                            VecMatch::All => Ok(Filter::Compiled(Box::new(move |a| {
                                values.iter().map(|x| a.contains(x)).all(|x| x)
                            }))),
                            VecMatch::OneOf => Ok(Filter::Compiled(Box::new(move |a| {
                                values.iter().map(|x| a.contains(x)).any(|x| x)
                            }))),
                            VecMatch::None => Ok(Filter::Compiled(Box::new(move |a| {
                                !values.iter().map(|x| a.contains(x)).any(|x| x)
                            }))),
                        }
                    }
                    // If we don't have an expression, we don't need to transform `self`.
                    _ => Ok(self),
                }
            }
        }
    };
}

compilable_vec!(u32);
compilable_vec!(f32);
compilable_vec!(String);

impl<'a> Compilable<'a, CostType> for Filter<'a, CostType> {
    fn compiled(self) -> Result<Filter<'a, CostType>, Error> {
        match self {
            Filter::Expr(str) => match str.as_str() {
                "Orn" => Ok(Filter::Compiled(Box::new(|a| *a == CostType::Orn))),
                "Gold" => Ok(Filter::Compiled(Box::new(|a| *a == CostType::Gold))),
                _ => Err(
                    Kind::Misc("Expected 'Orn' or 'Gold' for 'cost_type' field".to_string())
                        .into_err(),
                )
                .to_bad_request(),
            },
            _ => Ok(self),
        }
    }
}

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
    let Some(first_needle) = lowercase_needle.chars().next() else {
        return true;
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

/// Parse an expression of either of the form:
///   - `[x, y]`: Exact match, all elements must be contained, no more.
///   - `&[x, y]`: All of match, all elements must be contained.
///   - `|[x, y]`: One of match, one element at least must be contained.
///   - `^[x, y]`: None match, none  of the elements must be contained.
fn parse_array_filter_of<T>(expression: &str) -> Result<(VecMatch, Vec<T>), OError>
where
    T: FromStr + PartialOrd,
    <T as FromStr>::Err: ToString,
{
    // Retrieve match style. Default is Exact.
    let mut match_style = VecMatch::Exact;
    let str = if let Some(str) = expression.strip_prefix('&') {
        match_style = VecMatch::All;
        str
    } else if let Some(str) = expression.strip_prefix('|') {
        match_style = VecMatch::OneOf;
        str
    } else if let Some(str) = expression.strip_prefix('!') {
        match_style = VecMatch::None;
        str
    } else {
        expression
    };

    if !str.starts_with('[') || !str.ends_with(']') {
        return Err(Kind::Misc("Vec filter missing square brackets".to_string()).into());
    }
    let str = &str[1..str.len() - 1];

    // Convert a string of comma-separated values to a `Vec<$ty>`.
    let values = str
        .split(',')
        .map(str::trim)
        .map(T::from_str)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| Kind::Misc(e.to_string()))?
        .into_iter()
        .sorted_by(|a, b| a.partial_cmp(b).unwrap())
        .dedup()
        .collect_vec();
    Ok((match_style, values))
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
