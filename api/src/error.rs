use std::io::Cursor;

use rocket::{
    http::{ContentType, Status},
    response::{self, Responder, Response},
    Request,
};
use serde_json::Map;

/// A wrapper over the regular error, with the HTTP status included.
pub struct Error {
    /// HTTP status that goes with the response.
    pub status: Status,
    /// The error that happened.
    pub error: ornaguide_rs::error::Error,
}

impl Clone for Error {
    fn clone(&self) -> Self {
        Error {
            status: self.status,
            error: ornaguide_rs::error::Kind::Misc(format!("{}", self.error)).into(),
        }
    }
}

/// A type that can be converted to the above error, if `Err`.
/// Does not touch the `Ok` value.
pub trait ToErrorable<T> {
    /// Base methods. Consumes `self` with a status and combine them both.
    fn to_api_error(self, status: Status) -> Result<T, Error>;
    /// Shorthand for `to_api_error(Status::BadRequest)`.
    fn to_bad_request(self) -> Result<T, Error>;
    /// Shorthand for `to_api_error(Status::InternalServerError)`.
    fn to_internal_server_error(self) -> Result<T, Error>;
}

impl<T> ToErrorable<T> for Result<T, ornaguide_rs::error::Error> {
    fn to_api_error(self, status: Status) -> Result<T, Error> {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(Error { status, error: e }),
        }
    }

    fn to_bad_request(self) -> Result<T, Error> {
        self.to_api_error(Status::BadRequest)
    }

    fn to_internal_server_error(self) -> Result<T, Error> {
        self.to_api_error(Status::InternalServerError)
    }
}

impl<T> ToErrorable<T> for Result<T, &ornaguide_rs::error::Error> {
    fn to_api_error(self, status: Status) -> Result<T, Error> {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(Error {
                status,
                error: ornaguide_rs::error::Kind::Misc(format!("{e}")).into(),
            }),
        }
    }

    fn to_bad_request(self) -> Result<T, Error> {
        self.to_api_error(Status::BadRequest)
    }

    fn to_internal_server_error(self) -> Result<T, Error> {
        self.to_api_error(Status::InternalServerError)
    }
}

/// Responder type that either returns the `Ok` value with 200 OK or the `Err` value with the given
/// status code.
pub struct MaybeResponse {
    /// The result that will be transformed into a response.
    pub contents: Result<serde_json::Value, Error>,
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for MaybeResponse {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        use serde_json::Value;

        // Convert the `Ok` into a JSON value.
        self.contents
            .and_then(|x| {
                let s = serde_json::to_string(&x)
                    .map_err(ornaguide_rs::error::Error::from)
                    .to_api_error(Status::InternalServerError)?;
                Ok(Response::build()
                    .status(Status::Ok)
                    .header(ContentType::JSON)
                    .sized_body(s.len(), Cursor::new(s))
                    .ok())
            })
            // If something fails return the error.
            .unwrap_or_else(|err| {
                let mut map = Map::new();
                map.insert("error".to_string(), Value::String(format!("{}", err.error)));
                let s = serde_json::to_string(&Value::Object(map)).unwrap_or_else(|_| {
                    // Fall back to a default error string if we fail to serialize the error.
                    "{\"error\":\"Failed to make error json string\"}".to_string()
                });
                Response::build()
                    .status(err.status)
                    .header(ContentType::JSON)
                    .sized_body(s.len(), Cursor::new(s))
                    .ok()
            })
    }
}

/// Return an API error wrapping an [`ornaguide_rs::error::Error`] with the given string and status.
pub fn from_og(status: Status, contents: String) -> Error {
    Error {
        status,
        error: ornaguide_rs::error::Kind::Misc(contents).into(),
    }
}
