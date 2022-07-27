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
        /// The method that was used on the request..
        String,
        /// The URL that was requested.
        String,
        /// The status code we received.
        u16,
        /// An error message.
        String,
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
            Error::ResponseError(method, url, status, err) => {
                write!(f, "HTTP {} {} {}: {}", method, url, status, err)
            }
            Error::HTMLParsingError(err) => write!(f, "{}", err),
            Error::PartialCodexStatusEffectsConversion(found, not_found) => write!(
                f,
                "Partial codex status effects conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            Error::PartialCodexSkillsConversion(found, not_found) => write!(
                f,
                "Partial codex skills conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            Error::PartialCodexItemDroppedBysConversion(found, not_found) => write!(
                f,
                "Partial codex item dropped_bys conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            Error::PartialCodexItemUpgradeMaterialsConversion(found, not_found) => write!(
                f,
                "Partial codex item upgrade materials conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            Error::PartialCodexFollowerAbilitiesConversion(found, not_found) => write!(
                f,
                "Partial codex follower abilities conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            Error::PartialCodexMonsterAbilitiesConversion(found, not_found) => write!(
                f,
                "Partial codex monster abilities conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
            Error::PartialCodexEventsConversion(found, not_found) => write!(
                f,
                "Partial codex events conversion: OK {:?}, KO {:?}",
                found, not_found
            ),
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
