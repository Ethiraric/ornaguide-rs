use serde::{Deserialize, Serialize};

pub mod compilable;

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
    /// Check whether the filter is `Filter::None`.
    pub fn is_none(&self) -> bool {
        matches!(self, Filter::None)
    }

    /// Run the filter with the given value.
    /// Returns true if the filter validates the value, false otherwise.
    pub fn filter(&self, value: &T) -> bool {
        match self {
            Filter::Value(x) => value == x,
            Filter::Expr(str) => {
                panic!("Uncompiled filter '{}'", str);
            }
            Filter::Compiled(f) => f(value),
            Filter::None => true,
        }
    }
}
