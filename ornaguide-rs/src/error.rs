use std::{
    backtrace::Backtrace,
    fmt::{Debug, Display},
    io::IntoInnerError,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
    string::FromUtf8Error,
};

/// Generic error type.
pub enum ErrorKind {
    /// There was an error with `serde_json`.
    SerdeJson(serde_json::Error, String),
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
    /// There was an error parsing an enum.
    ParseEnumError(
        /// Name of the enum.
        String,
        /// Error message.
        String,
    ),
    /// There was an error parsing a boolean.
    ParseBoolError(ParseBoolError),
    /// There was an error parsing an integer.
    ParseIntError(ParseIntError),
    /// There was an error parsing a float.
    ParseFloatError(ParseFloatError),
    /// The request was successfully delivered, but the response indicated there was a failure.
    ResponseError(
        /// The method that was used on the request..
        String,
        /// The URL that was requested.
        String,
        /// The status code we received.
        u16,
        /// An error message.
        String,
    ),
    /// The request was successfully delivered, the guide responded 200 OK, but there are surprise
    /// errors which caused the guide to just ignore your POST request.
    GuidePostFormError(
        /// The URL that was requested.
        String,
        /// The generic error message.
        String,
        /// A list of errors found throughout the page.
        Vec<String>,
    ),
    /// There was an error in parsing HTML.
    HTMLParsingError(String),
    /// A conversion from multiple codex status effects to guide ids did not fully succeed.
    PartialCodexStatusEffectsConversion(
        /// The status effects that were successfully converted.
        Vec<u32>,
        /// The status effects that were not found on the guide.
        Vec<String>,
    ),
    /// A conversion from multiple codex skills to guide ids did not fully succeed.
    PartialCodexSkillsConversion(
        /// The skills that were successfully converted.
        Vec<u32>,
        /// The skills codex URIs that were not found on the guide.
        Vec<String>,
    ),
    /// A conversion from multiple codex item dropped_bys to guide ids did not fully succeed.
    PartialCodexItemDroppedBysConversion(
        /// The dropped_bys that were successfully converted.
        Vec<u32>,
        /// The monster codex URIs that were not found on the guide.
        Vec<String>,
    ),
    /// A conversion from multiple codex item upgrade materials to guide ids did not fully succeed.
    PartialCodexItemUpgradeMaterialsConversion(
        /// The upgrade materials that were successfully converted.
        Vec<u32>,
        /// The item codex URIs that were not found on the guide.
        Vec<String>,
    ),
    /// A conversion from multiple codex follower abilities to guide ids did not fully succeed.
    PartialCodexFollowerAbilitiesConversion(
        /// The abilities that were successfully converted.
        Vec<u32>,
        /// The skill codex URIs that were not found on the guide.
        Vec<String>,
    ),
    /// A conversion from multiple codex monster abilities to guide ids did not fully succeed.
    PartialCodexMonsterAbilitiesConversion(
        /// The abilities that were successfully converted.
        Vec<u32>,
        /// The skill codex URIs that were not found on the guide.
        Vec<String>,
    ),
    /// A conversion from multiple codex events to guide ids did not fully succeed.
    PartialCodexEventsConversion(
        /// The events that were successfully converted.
        Vec<u32>,
        /// The event names that were not found on the guide.
        Vec<String>,
    ),
    /// A conversion from different buffer types failed.
    BufferConversionError(String),
    /// An UTF-8 error occured.
    InvalidUTF8Conversion(String),
    /// Miscellaneous error.
    Misc(String),
}

impl ErrorKind {
    /// Convert the error kind into an error with backtrace and context.
    pub fn into_err(self) -> Error {
        Error::from(self)
    }
}

/// Main error type, containing both the error and its context.
#[derive(Debug)]
pub struct Error {
    /// The error that happened.
    pub kind: ErrorKind,
    /// The backtrace when the error happened, if activated.
    pub backtrace: Box<Backtrace>,
    /// Context that was added to the error.
    pub context: Vec<String>,
}

impl Error {
    /// Pushes an element into the context stack.
    /// The function consumes `self` and returns it so it is easier to use in a `map`.
    pub fn ctx_push(mut self, contents: String) -> Self {
        self.context.push(contents);
        self
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Debug).fmt(f)
    }
}

impl<T: Into<ErrorKind>> From<T> for Error {
    fn from(err: T) -> Self {
        Self {
            kind: err.into(),
            backtrace: Box::new(std::backtrace::Backtrace::capture()),
            context: vec![],
        }
    }
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::SerdeJson(err, name) => {
                if name.is_empty() {
                    write!(f, "{}", err)
                } else {
                    write!(f, "{}: {}", name, err)
                }
            }
            ErrorKind::Io(err) => write!(f, "{}", err),
            ErrorKind::MissingField(from, field) => {
                write!(f, "Failed to convert {}: missing field {}", from, field)
            }
            ErrorKind::ExtraField(from, field) => {
                write!(f, "Failed to convert {}: extra field {}", from, field)
            }
            ErrorKind::InvalidField(from, field, value) => match value {
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
            ErrorKind::Reqwest(err) => write!(f, "{}", err),
            ErrorKind::ParseEnumError(name, err) => {
                write!(f, "Could not parse enum {}: {}", name, err)
            }
            ErrorKind::ParseBoolError(err) => write!(f, "{}", err),
            ErrorKind::ParseIntError(err) => write!(f, "{}", err),
            ErrorKind::ParseFloatError(err) => write!(f, "{}", err),
            ErrorKind::ResponseError(method, url, status, err) => {
                write!(f, "HTTP {} {} {}: {}", method, url, status, err)
            }
            ErrorKind::GuidePostFormError(url, generic, errors) => {
                write!(f, "HTTP POST {}: {}: {:?}", url, generic, errors)
            }
            ErrorKind::HTMLParsingError(err) => write!(f, "{}", err),
            ErrorKind::PartialCodexStatusEffectsConversion(found, not_found) => write!(
                f,
                "Partial codex status effects conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            ErrorKind::PartialCodexSkillsConversion(found, not_found) => write!(
                f,
                "Partial codex skills conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            ErrorKind::PartialCodexItemDroppedBysConversion(found, not_found) => write!(
                f,
                "Partial codex item dropped_bys conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            ErrorKind::PartialCodexItemUpgradeMaterialsConversion(found, not_found) => write!(
                f,
                "Partial codex item upgrade materials conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            ErrorKind::PartialCodexFollowerAbilitiesConversion(found, not_found) => write!(
                f,
                "Partial codex follower abilities conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            ErrorKind::PartialCodexMonsterAbilitiesConversion(found, not_found) => write!(
                f,
                "Partial codex monster abilities conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            ErrorKind::PartialCodexEventsConversion(found, not_found) => write!(
                f,
                "Partial codex events conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            ErrorKind::InvalidUTF8Conversion(err) => write!(f, "{}", err),
            ErrorKind::BufferConversionError(err) => write!(f, "{}", err),
            ErrorKind::Misc(err) => write!(f, "{}", err),
        }
    }
}

impl Debug for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl From<ParseBoolError> for ErrorKind {
    fn from(err: ParseBoolError) -> Self {
        Self::ParseBoolError(err)
    }
}

impl From<ParseFloatError> for ErrorKind {
    fn from(err: ParseFloatError) -> Self {
        Self::ParseFloatError(err)
    }
}

impl From<ParseIntError> for ErrorKind {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<reqwest::Error> for ErrorKind {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err, String::new())
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl<T> From<IntoInnerError<T>> for ErrorKind {
    fn from(err: IntoInnerError<T>) -> Self {
        Self::BufferConversionError(err.to_string())
    }
}

impl From<FromUtf8Error> for ErrorKind {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidUTF8Conversion(err.to_string())
    }
}
