use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};

use crate::{
    error::Error,
    guide::html_parser::{parse_item_html, ParsedForm},
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

/// Names of the fields in the admin item change page.
const ITEM_FORM_FIELD_NAMES: &[&str] = &[
    "name",
    "tier",
    "type",
    "image_name",
    "description",
    "hp",
    "hp_affected_by_quality",
    "mana",
    "mana_affected_by_quality",
    "attack",
    "attack_affected_by_quality",
    "magic",
    "magic_affected_by_quality",
    "defense",
    "defense_affected_by_quality",
    "resistance",
    "resistance_affected_by_quality",
    "dexterity",
    "dexterity_affected_by_quality",
    "ward",
    "ward_affected_by_quality",
    "crit",
    "crit_affected_by_quality",
    "view_distance",
    "element",
    "equipped_by",
    "two_handed",
    "orn_bonus",
    "gold_bonus",
    "drop_bonus",
    "spawn_bonus",
    "exp_bonus",
    "boss",
    "arena",
    "category",
    "causes",
    "cures",
    "gives",
    "prevents",
    "materials",
    "price",
    "ability",
];

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

    pub(crate) fn admin_retrieve_item_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        let url = format!(concat!(BASE_PATH!(), "/admin/items/item/{}/change/"), id);
        parse_item_html(&self.http.get(url).send()?.text()?, ITEM_FORM_FIELD_NAMES)
    }

    pub(crate) fn admin_save_item(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        let mut url = reqwest::Url::parse("http://x").unwrap();
        url.query_pairs_mut()
            .extend_pairs(form.fields.iter())
            .append_pair("csrfmiddlewaretoken", &form.csrfmiddlewaretoken)
            .append_pair("_save", "Save");
        let body = url.query().unwrap().to_string();
        let response = self
            .http
            .post(format!(
                concat!(BASE_PATH!(), "/admin/items/item/{}/change/"),
                id
            ))
            .header(
                "Referer",
                format!(concat!(BASE_PATH!(), "/admin/items/item/{}/change/"), id),
            )
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Origin", "orna.guide")
            .body(body)
            .send()?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::ResponseError(
                response.status().as_u16(),
                response.text()?,
            ))
        }
    }
}
