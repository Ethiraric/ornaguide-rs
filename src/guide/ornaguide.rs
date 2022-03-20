use crate::{
    error::Error,
    guide::{html_parser::ParsedForm, http::Http, AdminGuide, Guide},
    items::{admin::AdminItem, raw::RawItem},
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

pub struct OrnaAdminGuide {
    guide: OrnaGuide,
}

impl OrnaAdminGuide {
    /// Construct a bare instance of the guide.
    pub fn new(cookie: &str) -> Result<Self, Error> {
        Ok(Self {
            guide: OrnaGuide::from_http(Http::new_with_cookie(cookie)?),
        })
    }
}

impl Guide for OrnaAdminGuide {
    fn fetch_items(&mut self) -> Result<&[RawItem], Error> {
        self.guide.fetch_items()
    }

    fn get_items(&self) -> Option<&[RawItem]> {
        self.guide.get_items()
    }
}

impl AdminGuide for OrnaAdminGuide {
    fn admin_retrieve_item_by_id(&self, id: u32) -> Result<AdminItem, Error> {
        Ok(AdminItem {
            id,
            ..AdminItem::try_from(self.guide.http().admin_retrieve_item_by_id(id)?)?
        })
    }
    fn admin_save_item(&self, item: AdminItem) -> Result<(), Error> {
        self.guide
            .http()
            .admin_save_item(item.id, ParsedForm::from(item))
    }
}
