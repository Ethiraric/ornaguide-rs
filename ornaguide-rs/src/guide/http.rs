use std::{
    fs::File,
    io::{BufWriter, Write},
};

use reqwest::{
    blocking::{Client, Response},
    header::{HeaderMap, HeaderValue},
    StatusCode, Url,
};

use crate::{
    codex::{
        html_follower_parser::parse_html_codex_follower,
        html_item_parser::parse_html_codex_item,
        html_list_parser::{parse_html_codex_list, Entry as CodexListEntry, ParsedList},
        html_monster_parser::{
            parse_html_codex_boss, parse_html_codex_monster, parse_html_codex_raid,
        },
        html_skill_parser::parse_html_codex_skill,
        CodexBoss, CodexFollower, CodexItem, CodexMonster, CodexRaid, CodexSkill,
    },
    error::Error,
    guide::{
        html_form_parser::{
            parse_item_html, parse_monster_html, parse_pet_html, parse_skill_html,
            parse_spawn_html, parse_status_effect_html, ParsedForm,
        },
        html_list_parser::{parse_list_html, Entry, ParsedTable},
    },
    items::RawItem,
    monsters::RawMonster,
    skills::RawSkill,
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

/// Base path of the API of `playorna.com` (`protocol://host[:port]`).
/// Can be used in `concat!`.
macro_rules! PLAYORNA_BASE_PATH {
    () => {
        "http://localhost:12345"
        // "https://playorna.com"
    };
}

/// Names of the fields in the admin item change page.
const ITEM_FORM_FIELD_NAMES: &[&str] = &[
    "codex",
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
    "potion_effectiveness",
];

/// Names of the fields in the admin monster change page.
const MONSTER_FORM_FIELD_NAMES: &[&str] = &[
    "codex",
    "name",
    "tier",
    "family",
    "image_name",
    "boss",
    "level",
    "hp",
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

/// Names of the fields in the admin skill change page.
const SKILL_FORM_FIELD_NAMES: &[&str] = &[
    "codex",
    "name",
    "tier",
    "type",
    "is_magic",
    "mana_cost",
    "description",
    "element",
    "offhand",
    "cost",
    "bought",
    "skill_power",
    "strikes",
    "modifier_min",
    "modifier_max",
    "extra",
    "buffed_by",
    "causes",
    "cures",
    "gives",
];

/// Names of the fields in the admin pet change page.
const PET_FORM_FIELD_NAMES: &[&str] = &[
    "codex",
    "name",
    "tier",
    "image_name",
    "description",
    "attack",
    "heal",
    "buff",
    "debuff",
    "spell",
    "protect",
    "cost",
    "cost_type",
    "limited",
    "limited_details",
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
            "POST".to_string(),
            url.to_string(),
            response.status().as_u16(),
            response.text()?,
        ))
    }
}

/// Send an HTTP GET request and expect that the response will be a 200 OK.
/// If the response isn't, return an error.
fn get_expect_200(http: &Client, url: &str) -> Result<Response, Error> {
    let response = http.get(url).send()?;
    if response.status() == StatusCode::OK {
        Ok(response)
    } else {
        Err(Error::ResponseError(
            "GET".to_string(),
            url.to_string(),
            response.status().as_u16(),
            response.text()?,
        ))
    }
}

/// Execute a GET HTTP request and save the output.
fn get_and_save(http: &Client, url: &str) -> Result<String, Error> {
    let response = get_expect_200(http, url)?;
    let body = response.text()?;
    let url = Url::parse(url).unwrap();
    if url.host_str().unwrap() != "localhost" {
        let path = url.path().replace('/', "_");
        let param = if let Some(x) = url.query() {
            format!("?{}", x)
        } else {
            String::new()
        };
        let filename = format!("htmls/{}{}{}.html", url.host_str().unwrap(), path, param);
        let mut writer = BufWriter::new(File::create(filename)?);
        write!(writer, "{}", body)?;
    }
    Ok(body)
}

/// Cycles through the different pages of the route and reads each table.
fn query_all_pages(base_url: &str, http: &Client) -> Result<Vec<Entry>, Error> {
    let ParsedTable {
        entries,
        number_entries,
    } = parse_list_html(&get_and_save(http, base_url)?)?;

    if entries.len() >= number_entries {
        Ok(entries)
    } else {
        let mut ret = entries;
        let mut page_no = 1;
        while ret.len() < number_entries {
            let ParsedTable {
                mut entries,
                number_entries: _,
            } = parse_list_html(&get_and_save(
                http,
                &format!("{}/?p={}", base_url, page_no),
            )?)?;
            page_no += 1;
            ret.append(&mut entries);
        }
        Ok(ret)
    }
}

/// Cycles through the different pages of the route and reads each table.
fn query_all_codex_pages(base_url: &str, http: &Client) -> Result<Vec<CodexListEntry>, Error> {
    let ParsedList {
        entries,
        mut has_next_page,
    } = parse_html_codex_list(&get_and_save(http, base_url)?)?;

    if !has_next_page {
        Ok(entries)
    } else {
        let mut ret = entries;
        let mut page_no = 2;
        while has_next_page {
            let ParsedList {
                mut entries,
                has_next_page: not_done,
            } = parse_html_codex_list(&get_and_save(
                http,
                &format!("{}/?p={}", base_url, page_no),
            )?)?;
            page_no += 1;
            ret.append(&mut entries);
            has_next_page = not_done;
        }
        Ok(ret)
    }
}

impl Http {
    // --- Misc ---
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

    // --- Guide API ---

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

    pub(crate) fn fetch_skills(&self) -> Result<Vec<RawSkill>, Error> {
        Ok(self
            .http
            .post(concat!(BASE_PATH!(), "/api/v1/skill"))
            .json("{}")
            .send()?
            .json()?)
    }

    // --- Guide Admin ---

    // Guide Admin Items
    pub(crate) fn admin_retrieve_item_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        let url = format!(concat!(BASE_PATH!(), "/admin/items/item/{}/change/"), id);
        parse_item_html(&get_and_save(&self.http, &url)?, ITEM_FORM_FIELD_NAMES)
    }

    pub(crate) fn admin_save_item(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!(concat!(BASE_PATH!(), "/admin/items/item/{}/change/"), id),
            form,
        )
    }

    pub(crate) fn admin_retrieve_items_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/items/item/");
        query_all_pages(url, &self.http)
    }

    // Guide Admin Monsters
    pub(crate) fn admin_retrieve_monster_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        let url = format!(
            concat!(BASE_PATH!(), "/admin/monsters/monster/{}/change/"),
            id
        );
        parse_monster_html(&get_and_save(&self.http, &url)?, MONSTER_FORM_FIELD_NAMES)
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

    pub(crate) fn admin_retrieve_monsters_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/monsters/monster/");
        query_all_pages(url, &self.http)
    }

    // Guide Admin Skills
    pub(crate) fn admin_retrieve_skill_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        let url = format!(concat!(BASE_PATH!(), "/admin/skills/skill/{}/change/"), id);
        parse_skill_html(&get_and_save(&self.http, &url)?, SKILL_FORM_FIELD_NAMES)
    }

    pub(crate) fn admin_save_skill(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!(concat!(BASE_PATH!(), "/admin/skills/skill/{}/change/"), id),
            form,
        )
    }

    pub(crate) fn admin_retrieve_skills_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/skills/skill/");
        query_all_pages(url, &self.http)
    }

    pub(crate) fn admin_add_skill(&self, form: ParsedForm) -> Result<(), Error> {
        let url = concat!(BASE_PATH!(), "/admin/skills/skill/add/");
        let mut post_form = parse_skill_html(&get_and_save(&self.http, url)?, &[])?;
        post_form.fields = form.fields;
        post_forms_to(&self.http, url, post_form)
    }

    // Guide Admin Pets
    pub(crate) fn admin_save_pet(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!(concat!(BASE_PATH!(), "/admin/pets/pet/{}/change/"), id),
            form,
        )
    }

    pub(crate) fn admin_retrieve_pet_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        let url = format!(concat!(BASE_PATH!(), "/admin/pets/pet/{}/change/"), id);
        parse_pet_html(&get_and_save(&self.http, &url)?, PET_FORM_FIELD_NAMES)
    }

    pub(crate) fn admin_retrieve_pets_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/pets/pet/");
        query_all_pages(url, &self.http)
    }

    // Guide Static data
    pub(crate) fn admin_retrieve_spawns_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/orna/spawn/");
        query_all_pages(url, &self.http)
    }

    pub(crate) fn admin_retrieve_item_categories_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/items/category/");
        query_all_pages(url, &self.http)
    }

    pub(crate) fn admin_retrieve_item_types_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/items/type/");
        query_all_pages(url, &self.http)
    }

    pub(crate) fn admin_retrieve_monster_families_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/monsters/family/");
        query_all_pages(url, &self.http)
    }

    pub(crate) fn admin_retrieve_status_effects_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/orna/statuseffect/");
        query_all_pages(url, &self.http)
    }

    pub(crate) fn admin_retrieve_skill_types_list(&self) -> Result<Vec<Entry>, Error> {
        let url = concat!(BASE_PATH!(), "/admin/skills/skilltype/");
        query_all_pages(url, &self.http)
    }

    pub(crate) fn admin_add_spawn(&self, spawn_name: &str) -> Result<(), Error> {
        let url = concat!(BASE_PATH!(), "/admin/orna/spawn/add/");
        let mut form = parse_spawn_html(&get_and_save(&self.http, url)?)?;
        form.fields
            .push(("description".to_string(), spawn_name.to_string()));
        post_forms_to(&self.http, url, form)
    }

    pub(crate) fn admin_add_status_effect(&self, status_effect_name: &str) -> Result<(), Error> {
        let url = concat!(BASE_PATH!(), "/admin/orna/statuseffect/add/");
        let mut form = parse_status_effect_html(&get_and_save(&self.http, url)?)?;
        form.fields
            .push(("name".to_string(), status_effect_name.to_string()));
        post_forms_to(&self.http, url, form)
    }

    // --- Codex ---

    // Codex Skills
    pub(crate) fn codex_retrieve_skills_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = concat!(PLAYORNA_BASE_PATH!(), "/codex/spells");
        query_all_codex_pages(url, &self.http)
    }

    pub(crate) fn codex_retrieve_skill(&self, skill_name: &str) -> Result<CodexSkill, Error> {
        let url = format!(
            concat!(PLAYORNA_BASE_PATH!(), "/codex/spells/{}"),
            skill_name
        );
        parse_html_codex_skill(&get_and_save(&self.http, &url)?, skill_name.to_string())
    }

    // Codex Monsters
    pub(crate) fn codex_retrieve_monsters_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = concat!(PLAYORNA_BASE_PATH!(), "/codex/monsters");
        query_all_codex_pages(url, &self.http)
    }

    pub(crate) fn codex_retrieve_monster(&self, monster_name: &str) -> Result<CodexMonster, Error> {
        let url = format!(
            concat!(PLAYORNA_BASE_PATH!(), "/codex/monsters/{}"),
            monster_name
        );
        parse_html_codex_monster(&get_and_save(&self.http, &url)?, monster_name.to_string())
    }

    // Codex Bosses
    pub(crate) fn codex_retrieve_bosses_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = concat!(PLAYORNA_BASE_PATH!(), "/codex/bosses");
        query_all_codex_pages(url, &self.http)
    }

    pub(crate) fn codex_retrieve_boss(&self, boss_name: &str) -> Result<CodexBoss, Error> {
        let url = format!(
            concat!(PLAYORNA_BASE_PATH!(), "/codex/bosses/{}"),
            boss_name
        );
        parse_html_codex_boss(&get_and_save(&self.http, &url)?, boss_name.to_string())
    }

    // Codex Raids
    pub(crate) fn codex_retrieve_raids_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = concat!(PLAYORNA_BASE_PATH!(), "/codex/raids");
        query_all_codex_pages(url, &self.http)
    }

    pub(crate) fn codex_retrieve_raid(&self, raid_name: &str) -> Result<CodexRaid, Error> {
        let url = format!(concat!(PLAYORNA_BASE_PATH!(), "/codex/raids/{}"), raid_name);
        parse_html_codex_raid(&get_and_save(&self.http, &url)?, raid_name.to_string())
    }

    // Codex Items
    pub(crate) fn codex_retrieve_items_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = concat!(PLAYORNA_BASE_PATH!(), "/codex/items");
        query_all_codex_pages(url, &self.http)
    }

    pub(crate) fn codex_retrieve_item(&self, item_name: &str) -> Result<CodexItem, Error> {
        let url = format!(concat!(PLAYORNA_BASE_PATH!(), "/codex/items/{}"), item_name);
        parse_html_codex_item(&get_and_save(&self.http, &url)?, item_name.to_string())
    }

    // Codex Followers
    pub(crate) fn codex_retrieve_followers_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = concat!(PLAYORNA_BASE_PATH!(), "/codex/followers");
        query_all_codex_pages(url, &self.http)
    }

    pub(crate) fn codex_retrieve_follower(
        &self,
        follower_name: &str,
    ) -> Result<CodexFollower, Error> {
        let url = format!(
            concat!(PLAYORNA_BASE_PATH!(), "/codex/followers/{}"),
            follower_name
        );
        parse_html_codex_follower(&get_and_save(&self.http, &url)?, follower_name.to_string())
    }
}
