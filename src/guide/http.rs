use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};

use crate::{
    error::Error,
    items::RawItem,
};

pub(crate) struct Http {
    http: Client,
}

/// Base path of the API (`protocol://host[:port]`).
/// Can be used in `concat!`.
macro_rules! BASE_PATH {
    () => {
        "http://localhost:12345"
        // "https://orna.guide/"
    };
}

impl Http {
    pub(crate) fn new() -> Self {
        Self {
            http: Client::new(),
        }
    }

    pub(crate) fn new_with_cookie(cookie: &str) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", HeaderValue::from_str(cookie).unwrap());
        Ok(Self {
            http: Client::builder().default_headers(headers).build()?,
        })
    }
    pub(crate) fn fetch_items(&self) -> Result<Vec<RawItem>, Error> {
        Ok(self
            .http
            .post(concat!(BASE_PATH!(), "/api/v1/items"))
            .json("{}")
            .send()?
            .json()?)
    }
}
