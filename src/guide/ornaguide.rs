use crate::{
    error::Error,
    guide::{http::Http, Guide},
    items::raw::RawItem,
};

/// The main interface for the guide.
pub struct OrnaGuide {
    http: Http,
    items: Option<Vec<RawItem>>,
}

impl OrnaGuide {
    /// Construct a bare instance of the guide.
    pub fn new() -> Self {
        Self {
            http: Http::new(),
            items: None,
        }
    }

    /// Construct an instance of the guide from an existing http session.
    /// This can be use to "subclass" the guide.
    fn from_http(http: Http) -> Self {
        Self {
            http,
            ..Default::default()
        }
    }

    /// Get the http session from the guide.
    fn http(&self) -> &Http {
        &self.http
    }
}

impl Default for OrnaGuide {
    fn default() -> Self {
        Self::new()
    }
}

impl Guide for OrnaGuide {
    fn fetch_items(&mut self) -> Result<&[RawItem], Error> {
        if self.items.is_none() {
            self.items = Some(self.http.fetch_items()?);
        }

        Ok(self.items.as_ref().unwrap())
    }

    fn get_items(&self) -> Option<&[RawItem]> {
        self.items.as_ref().map(|items| items.as_ref())
    }
}
