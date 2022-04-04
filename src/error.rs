use std::{
    fmt::{Debug, Display},
    num::{ParseFloatError, ParseIntError},
};

/// Generic error type.
pub enum Error {
    /// There was an error with `serde_json`.
    SerdeJson(serde_json::Error),
    /// There was an error with `std::io`.
    Io(std::io::Error),
    /// A field was missing when converting.
    /// The first `String` is the name of the object that was converted, the second one is the name
    /// of the field.
    MissingField(String, String),
    /// A field that shouldn't appear was found when converting.
    /// The first `String` is the name of the object that was converted, the second one is the name
    /// of the field.
    ExtraField(String, String),
    /// A field had an incorrect value when converting,
    /// The first `String` is the name of the object that was converted, the second one is the name
    /// of the field. The third one is an option of the value of the string.
    InvalidField(String, String, Option<String>),
    /// There was an error with `reqwest`.
    Reqwest(reqwest::Error),
    /// There was an error parsing an integer.
    ParseIntError(ParseIntError),
    /// There was an error parsing a float.
    ParseFloatError(ParseFloatError),
    /// The request was successfully delivered, but the response indicated there was a failure.
    ResponseError(
        /// The status code we received.
        u16,
        /// An error message.
        String,
    ),
    /// There was an error in parsing HTML.
    HTMLParsingError(String),
    /// Miscellaneous error.
    Misc(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SerdeJson(err) => write!(f, "{}", err),
            Error::Io(err) => write!(f, "{}", err),
            Error::MissingField(from, field) => {
                write!(f, "Failed to convert {}: missing field {}", from, field)
            }
            Error::ExtraField(from, field) => {
                write!(f, "Failed to convert {}: extra field {}", from, field)
            }
            Error::InvalidField(from, field, value) => match value {
                Some(s) => {
                    write!(
                        f,
                        "Failed to convert {}: invalid field {}={}",
                        from, field, s
                    )
                }
                None => {
                    write!(f, "Failed to convert {}: invalid field {}", from, field)
                }
            },
            Error::Reqwest(err) => write!(f, "{}", err),
            Error::ParseIntError(err) => write!(f, "{}", err),
            Error::ParseFloatError(err) => write!(f, "{}", err),
            Error::ResponseError(status, err) => write!(f, "HTTP {}: {}", status, err),
            Error::HTMLParsingError(err) => write!(f, "{}", err),
            Error::Misc(err) => write!(f, "{}", err),
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Self::ParseFloatError(err)
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
