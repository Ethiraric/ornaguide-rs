use std::str::FromStr;

use ornaguide_rs::error::Error;
use serde::{Deserialize, Serialize};

/// A field in a request which allows filtering the results.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Filter<'a, T> {
    /// No filter. Will always allow any item through.
    None,
    /// A value to which to compare for equality.
    Value(T),
    /// An expression. Must start with an operator (`==`, `!=`, `>`, `<`, `>=`, `<=`) and be
    /// immediately followed by a string parseable into `T`.
    Expr(String),
    /// Parsed version of an expression string.
    #[serde(skip)]
    Compiled(Box<dyn Fn(&T) -> bool + 'a>),
}

impl<'a, T> Default for Filter<'a, T> {
    fn default() -> Self {
        Self::None
    }
}

impl<'a, T> Filter<'a, T>
where
    T: std::str::FromStr + std::cmp::PartialOrd + 'a,
{
    /// Run the filter with the given value.
    /// Returns true if the filter validates the value, false otherwise.
    pub fn filter(&self, value: &T) -> bool {
        match self {
            Filter::Value(x) => value == x,
            Filter::Expr(str) => {
                warn!("Uncompiled filter '{}'", str);
                true
            }
            Filter::Compiled(f) => f(value),
            Filter::None => true,
        }
    }

    /// If the filter is an expression one, "compile" it to a more efficient representation.
    /// Parse the expression and create a closure from it.
    pub fn compiled(self) -> Result<Self, Error>
    where
        <T as FromStr>::Err: ToString,
    {
        match self {
            // If we have an expression, rewrite it.
            Filter::Expr(str) => {
                // The expression must have at least 2 chars if `<` or `>`, 3 otherwise.
                if str.len() < 2 || (str.chars().nth(1).unwrap() == '=' && str.len() < 3) {
                    Err(Error::Misc(format!("Expression is too short: '{}'", str)))
                } else if str.chars().nth(1).unwrap() == '=' {
                    // If we have a 2 chars operator, parse value starting from 3rd char.
                    let expected_value =
                        T::from_str(&str[2..]).map_err(|s| Error::Misc(s.to_string()))?;

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
                    let expected_value =
                        T::from_str(&str[1..]).map_err(|s| Error::Misc(s.to_string()))?;

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
            // If we don't have an expression, we don't need to transform `self`.
            _ => Ok(self),
        }
    }
}
