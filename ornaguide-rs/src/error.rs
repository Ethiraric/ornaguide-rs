use std::{
    fmt::{Debug, Display},
    io::IntoInnerError,
    num::{ParseFloatError, ParseIntError},
    str::ParseBoolError,
    string::FromUtf8Error,
};

use backtrace::Backtrace;
use color_backtrace::termcolor::Ansi;

/// Generic error type.
pub enum Kind {
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

impl Kind {
    /// Convert the error kind into an error with backtrace and context.
    #[must_use]
    pub fn into_err(self) -> Error {
        Error::from(self)
    }
}

/// Main error type, containing both the error and its context.
#[derive(Debug)]
pub struct Error {
    /// The error that happened.
    pub kind: Kind,
    /// The backtrace when the error happened, if activated.
    pub backtrace: Box<Backtrace>,
    /// Context that was added to the error.
    pub context: Vec<String>,
}

impl Error {
    /// Pushes an element into the context stack.
    /// The function consumes `self` and returns it so it is easier to use in a `map`.
    #[must_use]
    pub fn ctx_push(mut self, contents: String) -> Self {
        self.context.push(contents);
        self
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error {{\n  kind: {:?},\n  context: {:?}\n}}\n",
            self.kind, self.context
        )?;
        color_backtrace::BacktracePrinter::new()
            .print_trace(
                &self.backtrace,
                &mut Ansi::new(WritableFormatter(f)), // &mut color_backtrace::default_output_stream(),
            )
            .unwrap();
        Ok(())
    }
}

impl<T: Into<Kind>> From<T> for Error {
    fn from(err: T) -> Self {
        Self {
            kind: err.into(),
            backtrace: Box::new(Backtrace::new()),
            context: vec![],
        }
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::SerdeJson(err, name) => {
                if name.is_empty() {
                    write!(f, "{err}")
                } else {
                    write!(f, "{name}: {err}")
                }
            }
            Kind::Io(err) => write!(f, "{err}"),
            Kind::MissingField(from, field) => {
                write!(f, "Failed to convert {from}: missing field {field}")
            }
            Kind::ExtraField(from, field) => {
                write!(f, "Failed to convert {from}: extra field {field}")
            }
            Kind::InvalidField(from, field, value) => match value {
                Some(s) => {
                    write!(f, "Failed to convert {from}: invalid field {field}={s}")
                }
                None => {
                    write!(f, "Failed to convert {from}: invalid field {field}")
                }
            },
            Kind::Reqwest(err) => write!(f, "{err}"),
            Kind::ParseEnumError(name, err) => {
                write!(f, "Could not parse enum {name}: {err}")
            }
            Kind::ParseBoolError(err) => write!(f, "{err}"),
            Kind::ParseIntError(err) => write!(f, "{err}"),
            Kind::ParseFloatError(err) => write!(f, "{err}"),
            Kind::ResponseError(method, url, status, err) => {
                write!(f, "HTTP {method} {url} {status}: {err}")
            }
            Kind::GuidePostFormError(url, generic, errors) => {
                write!(f, "HTTP POST {url}: {generic}: {errors:?}")
            }
            Kind::HTMLParsingError(err) => write!(f, "{err}"),
            Kind::PartialCodexStatusEffectsConversion(found, not_found) => write!(
                f,
                "Partial codex status effects conversion: OK {found:?}, KO {not_found:?}"
            ),
            Kind::PartialCodexSkillsConversion(found, not_found) => write!(
                f,
                "Partial codex skills conversion: OK {found:?}, KO {not_found:?}"
            ),
            Kind::PartialCodexItemDroppedBysConversion(found, not_found) => write!(
                f,
                "Partial codex item dropped_bys conversion: OK {found:?}, KO {not_found:?}"
            ),
            Kind::PartialCodexItemUpgradeMaterialsConversion(found, not_found) => write!(
                f,
                "Partial codex item upgrade materials conversion: OK {found:?}, KO {not_found:?}"
            ),
            Kind::PartialCodexFollowerAbilitiesConversion(found, not_found) => write!(
                f,
                "Partial codex follower abilities conversion: OK {found:?}, KO {not_found:?}"
            ),
            Kind::PartialCodexMonsterAbilitiesConversion(found, not_found) => write!(
                f,
                "Partial codex monster abilities conversion: OK {found:?}, KO {not_found:?}"
            ),
            Kind::PartialCodexEventsConversion(found, not_found) => write!(
                f,
                "Partial codex events conversion: OK {found:?}, KO {not_found:?}"
            ),
            Kind::InvalidUTF8Conversion(err)
            | Kind::BufferConversionError(err)
            | Kind::Misc(err) => {
                write!(f, "{err}")
            }
        }
    }
}

impl Debug for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self as &dyn Display).fmt(f)
    }
}

impl From<ParseBoolError> for Kind {
    fn from(err: ParseBoolError) -> Self {
        Self::ParseBoolError(err)
    }
}

impl From<ParseFloatError> for Kind {
    fn from(err: ParseFloatError) -> Self {
        Self::ParseFloatError(err)
    }
}

impl From<ParseIntError> for Kind {
    fn from(err: ParseIntError) -> Self {
        Self::ParseIntError(err)
    }
}

impl From<reqwest::Error> for Kind {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<serde_json::Error> for Kind {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeJson(err, String::new())
    }
}

impl From<std::io::Error> for Kind {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl<T> From<IntoInnerError<T>> for Kind {
    fn from(err: IntoInnerError<T>) -> Self {
        Self::BufferConversionError(err.to_string())
    }
}

impl From<FromUtf8Error> for Kind {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidUTF8Conversion(err.to_string())
    }
}

/// A newtype to have `Formatter` impl `io::Write`
struct WritableFormatter<'a, 'b>(&'a mut std::fmt::Formatter<'b>);

impl<'a, 'b> std::io::Write for WritableFormatter<'a, 'b> {
    fn write(&mut self, bytes: &[u8]) -> std::result::Result<usize, std::io::Error> {
        self.0
            .write_str(&String::from_utf8_lossy(bytes))
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        todo!()
    }
}
