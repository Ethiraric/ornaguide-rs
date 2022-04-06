use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderValue},
};

use crate::{
    error::Error,
    guide::{
        html_form_parser::{parse_item_html, parse_monster_html, ParsedForm},
        html_list_parser::{parse_list_html, Entry, ParsedTable},
    },
    items::RawItem,
    monsters::RawMonster,
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
    "notes",
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
    "foresight",
    "view_distance",
    "follower_stats",
    "follower_act",
    "status_infliction",
    "status_protection",
    "mana_saver",
    "has_slots",
    "base_adornment_slots",
    "rarity",
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

/// Names of the fields in the admin monster change page.
const MONSTER_FORM_FIELD_NAMES: &[&str] = &[
    "name",
    "tier",
    "family",
    "image_name",
    "boss",
    "level",
    "notes",
    "spawns",
    "weak_to",
    "resistant_to",
    "immune_to",
    "immune_to_status",
    "vulnerable_to_status",
    "drops",
    "skills",
];

/// Perform a POST request on the URL, serializing the form as an urlencoded body and setting the
/// referer to the URL.
fn post_forms_to(http: &Client, url: &str, form: ParsedForm) -> Result<(), Error> {
    let mut tmpurl = reqwest::Url::parse("http://x").unwrap();
    tmpurl
        .query_pairs_mut()
        .extend_pairs(form.fields.iter())
        .append_pair("csrfmiddlewaretoken", &form.csrfmiddlewaretoken)
        .append_pair("_save", "Save");
    let body = tmpurl.query().unwrap().to_string();
    let response = http
        .post(url)
        .header("Referer", url)
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

/// Cycles through the different pages of the route and reads each table.
fn query_all_pages(base_url: &str, http: &Client) -> Result<Vec<Entry>, Error> {
    let ParsedTable {
        entries,
        number_entries,
    } = parse_list_html(&http.get(base_url).send()?.text()?)?;

    if entries.len() >= number_entries {
        Ok(entries)
    } else {
        let mut ret = entries;
        let mut page_no = 1;
        while ret.len() < number_entries {
            let ParsedTable {
                mut entries,
                number_entries: _,
            } = parse_list_html(
                &http
                    .get(format!("{}/?p={}", base_url, page_no))
                    .send()?
                    .text()?,
            )?;
            page_no += 1;
            ret.append(&mut entries);
        }
        Ok(ret)
    }
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

    pub(crate) fn fetch_monsters(&self) -> Result<Vec<RawMonster>, Error> {
        Ok(self
            .http
            .post(concat!(BASE_PATH!(), "/api/v1/monster"))
            .json("{}")
            .send()?
            .json()?)
    }

    pub(crate) fn admin_retrieve_item_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        let url = format!(concat!(BASE_PATH!(), "/admin/items/item/{}/change/"), id);
        parse_item_html(&self.http.get(url).send()?.text()?, ITEM_FORM_FIELD_NAMES)
    }

    pub(crate) fn admin_save_item(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!(concat!(BASE_PATH!(), "/admin/items/item/{}/change/"), id),
            form,
        )
    }

    pub(crate) fn admin_retrieve_monster_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        let url = format!(
            concat!(BASE_PATH!(), "/admin/monsters/monster/{}/change/"),
            id
        );
        parse_monster_html(
            &self.http.get(url).send()?.text()?,
            MONSTER_FORM_FIELD_NAMES,
        )
    }

    pub(crate) fn admin_save_monster(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!(
                concat!(BASE_PATH!(), "/admin/monsters/monster/{}/change/"),
                id
            ),
            form,
        )
    }

    pub(crate) fn admin_retrieve_spawns_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/orna/spawn/");
        query_all_pages(url, &self.http)
    }

    pub(crate) fn admin_retrieve_skills_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/skills/skill/");
        query_all_pages(url, &self.http)
    }
}
