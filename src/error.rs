use std::fmt::Display;

/// Generic error type.
pub enum Error {
    /// There was an error with `serde_json`.
    SerdeJson(serde_json::Error),
    /// There was an error with `std::io`.
    Io(std::io::Error),
    /// A field was missing when converting.
    /// The first `String` is the type of the object that was converted, the second one is the name
    /// of the field.
    MissingField(String, String),
    /// A field that shouldn't appear was found when converting.
    /// The first `String` is the type of the object that was converted, the second one is the name
    /// of the field.
    ExtraField(String, String),
    /// A field had an incorrect value when converting,
    /// The first `String` is the type of the object that was converted, the second one is the name
    /// of the field. The third one is an option of the value of the string.
    InvalidField(String, String, Option<String>),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::SerdeJson(err) => write!(f, "{}", err),
            Error::Io(err) => write!(f, "{}", err),
            Error::MissingField(from, field) => {
                write!(f, "Failed to convert to {}: missing field {}", from, field)
            }
            Error::ExtraField(from, field) => {
                write!(f, "Failed to convert to {}: extra field {}", from, field)
            }
            Error::InvalidField(from, field, value) => match value {
                Some(s) => {
                    write!(
                        f,
                        "Failed to convert to {}: invalid field {}={}",
                        from, field, s
                    )
                }
                None => {
                    write!(f, "Failed to convert to {}: invalid field {}", from, field)
                }
            },
        }
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
