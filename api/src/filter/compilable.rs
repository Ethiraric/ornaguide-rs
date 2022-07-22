use std::str::FromStr;

use ornaguide_rs::error::Error;

use crate::filter::Filter;

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
            Filter::Expr(_) => Err(Error::Misc(
                "Cannot use expressions with strings".to_string(),
            )),
            _ => Ok(self),
        }
    }
}
