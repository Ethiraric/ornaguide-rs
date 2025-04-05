use std::{
    fs::File,
    io::{BufWriter, Write},
};

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Response, StatusCode, Url,
};

use crate::{
    codex::{
        html_follower_parser::{parse_html_codex_follower, parse_html_codex_follower_translation},
        html_item_parser::{parse_html_codex_item, parse_html_codex_item_translation},
        html_list_parser::{parse_html_codex_list, Entry as CodexListEntry, ParsedList},
        html_monster_parser::{
            parse_html_codex_boss, parse_html_codex_boss_translation, parse_html_codex_monster,
            parse_html_codex_monster_translation, parse_html_codex_raid,
            parse_html_codex_raid_translation,
        },
        html_skill_parser::{parse_html_codex_skill, parse_html_codex_skill_translation},
        CodexBoss, CodexFollower, CodexItem, CodexMonster, CodexRaid, CodexSkill,
    },
    config::debug_urls,
    error::{Error, Kind},
    guide::{
        html_form_parser::{
            parse_item_html, parse_monster_html, parse_pet_html, parse_skill_html,
            parse_spawn_html, parse_status_effect_html, ParsedForm, ITEM_FORM_FIELD_NAMES,
            MONSTER_FORM_FIELD_NAMES, PET_FORM_FIELD_NAMES, SKILL_FORM_FIELD_NAMES,
        },
        html_list_parser::{parse_list_html, Entry, ParsedTable},
        post_error_parser::parse_post_error_html,
    },
    utils::block_on_this_thread,
};

#[allow(clippy::struct_field_names)]
pub(crate) struct Http {
    http: Client,
    orna_guide_host: String,
    playorna_host: String,
}

/// Perform a POST request on the URL, serializing the form as an urlencoded body and setting the
/// referer to the URL.
async fn async_post_forms_to(
    http: &Client,
    url: &str,
    form: ParsedForm,
    form_root_name: &str,
) -> Result<(), Error> {
    if debug_urls()? {
        eprintln!("--- POST {url}");
    }

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
        .send()
        .await?;

    let status = response.status();
    let text = response.text().await?;
    parse_post_error_html(url, &text, form_root_name)?;

    if status.is_success() {
        Ok(())
    } else {
        Err(Kind::ResponseError("POST".to_string(), url.to_string(), status.as_u16(), text).into())
    }
}

/// Perform a POST request on the URL, serializing the form as an urlencoded body and setting the
/// referer to the URL.
fn post_forms_to(
    http: &Client,
    url: &str,
    form: ParsedForm,
    form_root_name: &str,
) -> Result<(), Error> {
    block_on_this_thread(async_post_forms_to(http, url, form, form_root_name))
}

/// Send an HTTP GET request and expect that the response will be a 200 OK.
/// If the response isn't, return an error.
async fn get_expect_200(http: &Client, url: &str) -> Result<Response, Error> {
    let response = http.get(url).send().await?;
    if response.status() == StatusCode::OK {
        Ok(response)
    } else {
        Err(Kind::ResponseError(
            "GET".to_string(),
            url.to_string(),
            response.status().as_u16(),
            response.text().await?,
        )
        .into())
    }
}

/// Execute a GET HTTP request and save the output.
async fn async_get_and_save(http: &Client, url: &str) -> Result<String, Error> {
    if debug_urls()? {
        eprintln!("--- GET {url}");
    }
    let response = get_expect_200(http, url).await?;
    let body = response.text().await?;
    let url = Url::parse(url).unwrap();
    if url.host_str().unwrap() != "localhost" {
        let path = url.path().replace('/', "_");
        let param = if let Some(x) = url.query() {
            format!("?{x}")
        } else {
            String::new()
        };
        let filename = format!(
            "data/htmls/{}{}{}.html",
            url.host_str().unwrap(),
            path,
            param
        );
        let mut writer = BufWriter::new(File::create(filename)?);
        write!(writer, "{body}")?;
    }
    Ok(body)
}

/// Execute a GET HTTP request and save the output.
/// We need to have both the `send` and the `text` calls run on the same runtime. We cannot use two
/// calls to `block_on` in `async_get_and_save`.
fn get_and_save(http: &Client, url: &str) -> Result<String, Error> {
    block_on_this_thread(async_get_and_save(http, url))
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
            } = parse_list_html(&get_and_save(http, &format!("{base_url}/?p={page_no}"))?)?;
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

    if has_next_page {
        let mut ret = entries;
        let mut page_no = 2;
        while has_next_page {
            let ParsedList {
                mut entries,
                has_next_page: not_done,
            } = parse_html_codex_list(&get_and_save(http, &format!("{base_url}/?p={page_no}"))?)?;
            page_no += 1;
            ret.append(&mut entries);
            has_next_page = not_done;
        }
        Ok(ret)
    } else {
        Ok(entries)
    }
}

impl Http {
    // --- Misc ---
    pub(crate) fn new() -> Self {
        Self {
            http: Client::new(),
            orna_guide_host: "https://orna.guide".to_string(),
            playorna_host: "https://playorna.com".to_string(),
        }
    }

    pub(crate) fn new_with_cookie(cookie: &str) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", HeaderValue::from_str(cookie).unwrap());
        Ok(Self {
            http: Client::builder().default_headers(headers).build()?,
            ..Self::new()
        })
    }

    pub(crate) fn new_with_cookie_and_hosts(
        cookie: &str,
        orna_guide: String,
        playorna: String,
    ) -> Result<Self, Error> {
        let mut headers = HeaderMap::new();
        headers.insert("Cookie", HeaderValue::from_str(cookie).unwrap());
        Ok(Self {
            orna_guide_host: orna_guide,
            playorna_host: playorna,
            ..Self::new_with_cookie(cookie)?
        })
    }

    // --- Guide Admin ---

    // Guide Admin Items
    pub(crate) async fn async_admin_retrieve_item_by_id(
        &self,
        id: u32,
    ) -> Result<ParsedForm, Error> {
        let url = format!("{}/admin/items/item/{}/change/", self.orna_guide_host, id);
        parse_item_html(
            &async_get_and_save(&self.http, &url).await?,
            ITEM_FORM_FIELD_NAMES,
        )
    }

    #[allow(dead_code)]
    pub(crate) fn admin_retrieve_item_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        block_on_this_thread(self.async_admin_retrieve_item_by_id(id))
    }

    pub(crate) fn admin_save_item(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!("{}/admin/items/item/{}/change/", self.orna_guide_host, id),
            form,
            "#item_form",
        )
    }

    pub(crate) fn admin_retrieve_items_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/items/item/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_add_item(&self, form: ParsedForm) -> Result<(), Error> {
        let url = format!("{}/admin/items/item/add/", self.orna_guide_host);
        let mut post_form = parse_item_html(&get_and_save(&self.http, &url)?, &[])?;
        post_form.fields = form.fields;
        post_forms_to(&self.http, &url, post_form, "#item_form")
    }

    // Guide Admin Monsters
    pub(crate) async fn async_admin_retrieve_monster_by_id(
        &self,
        id: u32,
    ) -> Result<ParsedForm, Error> {
        let url = format!(
            "{}/admin/monsters/monster/{}/change/",
            self.orna_guide_host, id
        );
        parse_monster_html(
            &async_get_and_save(&self.http, &url).await?,
            MONSTER_FORM_FIELD_NAMES,
        )
    }

    #[allow(dead_code)]
    pub(crate) fn admin_retrieve_monster_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        block_on_this_thread(self.async_admin_retrieve_monster_by_id(id))
    }

    pub(crate) fn admin_save_monster(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!(
                "{}/admin/monsters/monster/{}/change/",
                self.orna_guide_host, id
            ),
            form,
            "#monster_form",
        )
    }

    pub(crate) fn admin_retrieve_monsters_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/monsters/monster/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_add_monster(&self, form: ParsedForm) -> Result<(), Error> {
        let url = format!("{}/admin/monsters/monster/add/", self.orna_guide_host);
        let mut post_form = parse_monster_html(&get_and_save(&self.http, &url)?, &[])?;
        post_form.fields = form.fields;
        post_forms_to(&self.http, &url, post_form, "#monster_form")
    }

    // Guide Admin Skills
    pub(crate) async fn async_admin_retrieve_skill_by_id(
        &self,
        id: u32,
    ) -> Result<ParsedForm, Error> {
        let url = format!("{}/admin/skills/skill/{}/change/", self.orna_guide_host, id);
        parse_skill_html(
            &async_get_and_save(&self.http, &url).await?,
            SKILL_FORM_FIELD_NAMES,
        )
    }

    #[allow(dead_code)]
    pub(crate) fn admin_retrieve_skill_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        block_on_this_thread(self.async_admin_retrieve_skill_by_id(id))
    }

    pub(crate) fn admin_save_skill(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!("{}/admin/skills/skill/{}/change/", self.orna_guide_host, id),
            form,
            "#skill_form",
        )
    }

    pub(crate) fn admin_retrieve_skills_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/skills/skill/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_add_skill(&self, form: ParsedForm) -> Result<(), Error> {
        let url = format!("{}/admin/skills/skill/add/", self.orna_guide_host);
        let mut post_form = parse_skill_html(&get_and_save(&self.http, &url)?, &[])?;
        post_form.fields = form.fields;
        post_forms_to(&self.http, &url, post_form, "#skill_form")
    }

    // Guide Admin Pets
    pub(crate) async fn async_admin_retrieve_pet_by_id(
        &self,
        id: u32,
    ) -> Result<ParsedForm, Error> {
        let url = format!("{}/admin/pets/pet/{}/change/", self.orna_guide_host, id);
        parse_pet_html(
            &async_get_and_save(&self.http, &url).await?,
            PET_FORM_FIELD_NAMES,
        )
    }

    #[allow(dead_code)]
    pub(crate) fn admin_retrieve_pet_by_id(&self, id: u32) -> Result<ParsedForm, Error> {
        block_on_this_thread(self.async_admin_retrieve_pet_by_id(id))
    }

    pub(crate) fn admin_save_pet(&self, id: u32, form: ParsedForm) -> Result<(), Error> {
        post_forms_to(
            &self.http,
            &format!("{}/admin/pets/pet/{}/change/", self.orna_guide_host, id),
            form,
            "#pet_form",
        )
    }

    pub(crate) fn admin_retrieve_pets_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/pets/pet/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_add_pet(&self, form: ParsedForm) -> Result<(), Error> {
        let url = format!("{}/admin/pets/pet/add/", self.orna_guide_host);
        let mut post_form = parse_pet_html(&get_and_save(&self.http, &url)?, &[])?;
        post_form.fields = form.fields;
        post_forms_to(&self.http, &url, post_form, "#pet_form")
    }

    // Guide Static data
    pub(crate) fn admin_retrieve_spawns_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/orna/spawn/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_retrieve_item_categories_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/items/category/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_retrieve_item_types_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/items/type/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_retrieve_monster_families_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/monsters/family/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_retrieve_status_effects_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/orna/statuseffect/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_retrieve_skill_types_list(&self) -> Result<Vec<Entry>, Error> {
        let url = format!("{}/admin/skills/skilltype/", self.orna_guide_host);
        query_all_pages(&url, &self.http)
    }

    pub(crate) fn admin_add_spawn(&self, spawn_name: &str) -> Result<(), Error> {
        let url = format!("{}/admin/orna/spawn/add/", self.orna_guide_host);
        let mut form = parse_spawn_html(&get_and_save(&self.http, &url)?)?;
        form.fields
            .push(("description".to_string(), spawn_name.to_string()));
        post_forms_to(&self.http, &url, form, "#spawn_form")
    }

    pub(crate) fn admin_add_status_effect(&self, status_effect_name: &str) -> Result<(), Error> {
        let url = format!("{}/admin/orna/statuseffect/add/", self.orna_guide_host);
        let mut form = parse_status_effect_html(&get_and_save(&self.http, &url)?)?;
        form.fields
            .push(("name".to_string(), status_effect_name.to_string()));
        post_forms_to(&self.http, &url, form, "#statuseffect_form")
    }

    // --- Codex ---

    // Codex Skills
    pub(crate) fn codex_retrieve_skills_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = format!("{}/codex/spells", self.playorna_host);
        query_all_codex_pages(&url, &self.http)
    }

    pub(crate) fn codex_retrieve_skill_page(&self, skill_name: &str) -> Result<String, Error> {
        let url = format!("{}/codex/spells/{}", self.playorna_host, skill_name);
        get_and_save(&self.http, &url)
    }

    pub(crate) fn codex_retrieve_skill(&self, skill_name: &str) -> Result<CodexSkill, Error> {
        parse_html_codex_skill(
            &self.codex_retrieve_skill_page(skill_name)?,
            skill_name.to_string(),
        )
    }

    // Codex Monsters
    pub(crate) fn codex_retrieve_monsters_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = format!("{}/codex/monsters", self.playorna_host);
        query_all_codex_pages(&url, &self.http)
    }

    pub(crate) fn codex_retrieve_monster_page(&self, monster_name: &str) -> Result<String, Error> {
        let url = format!("{}/codex/monsters/{}", self.playorna_host, monster_name);
        get_and_save(&self.http, &url)
    }

    pub(crate) fn codex_retrieve_monster(&self, monster_name: &str) -> Result<CodexMonster, Error> {
        parse_html_codex_monster(
            &self.codex_retrieve_monster_page(monster_name)?,
            monster_name,
        )
    }

    // Codex Bosses
    pub(crate) fn codex_retrieve_bosses_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = format!("{}/codex/bosses", self.playorna_host);
        query_all_codex_pages(&url, &self.http)
    }

    pub(crate) fn codex_retrieve_boss_page(&self, boss_name: &str) -> Result<String, Error> {
        let url = format!("{}/codex/bosses/{}", self.playorna_host, boss_name);
        get_and_save(&self.http, &url)
    }

    pub(crate) fn codex_retrieve_boss(&self, boss_name: &str) -> Result<CodexBoss, Error> {
        parse_html_codex_boss(&self.codex_retrieve_boss_page(boss_name)?, boss_name)
    }

    // Codex Raids
    pub(crate) fn codex_retrieve_raids_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = format!("{}/codex/raids", self.playorna_host);
        query_all_codex_pages(&url, &self.http)
    }

    pub(crate) fn codex_retrieve_raid_page(&self, raid_name: &str) -> Result<String, Error> {
        let url = format!("{}/codex/raids/{}", self.playorna_host, raid_name);
        get_and_save(&self.http, &url)
    }

    pub(crate) fn codex_retrieve_raid(&self, raid_name: &str) -> Result<CodexRaid, Error> {
        parse_html_codex_raid(&self.codex_retrieve_raid_page(raid_name)?, raid_name)
    }

    // Codex Items
    pub(crate) fn codex_retrieve_items_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = format!("{}/codex/items", self.playorna_host);
        query_all_codex_pages(&url, &self.http)
    }

    pub(crate) fn codex_retrieve_item_page(&self, item_name: &str) -> Result<String, Error> {
        let url = format!("{}/codex/items/{}", self.playorna_host, item_name);
        get_and_save(&self.http, &url)
    }

    pub(crate) fn codex_retrieve_item(&self, item_name: &str) -> Result<CodexItem, Error> {
        parse_html_codex_item(&self.codex_retrieve_item_page(item_name)?, item_name)
    }

    // Codex Followers
    pub(crate) fn codex_retrieve_followers_list(&self) -> Result<Vec<CodexListEntry>, Error> {
        let url = format!("{}/codex/followers", self.playorna_host);
        query_all_codex_pages(&url, &self.http)
    }

    pub(crate) fn codex_retrieve_follower_page(
        &self,
        follower_name: &str,
    ) -> Result<String, Error> {
        let url = format!("{}/codex/followers/{}", self.playorna_host, follower_name);
        get_and_save(&self.http, &url)
    }

    pub(crate) fn codex_retrieve_follower(
        &self,
        follower_name: &str,
    ) -> Result<CodexFollower, Error> {
        parse_html_codex_follower(
            &self.codex_retrieve_follower_page(follower_name)?,
            follower_name.to_string(),
        )
    }

    // --- Codex i18n ---

    pub(crate) fn codex_retrieve_skill_translation(
        &self,
        skill_name: &str,
        locale: &str,
    ) -> Result<CodexSkill, Error> {
        let url = format!(
            "{}/codex/spells/{}/?lang={}",
            self.playorna_host, skill_name, locale
        );
        parse_html_codex_skill_translation(&get_and_save(&self.http, &url)?, skill_name.to_string())
    }

    pub(crate) fn codex_retrieve_monster_translation(
        &self,
        monster_name: &str,
        locale: &str,
    ) -> Result<CodexMonster, Error> {
        let url = format!(
            "{}/codex/monsters/{}/?lang={}",
            self.playorna_host, monster_name, locale
        );
        parse_html_codex_monster_translation(&get_and_save(&self.http, &url)?, monster_name)
    }

    pub(crate) fn codex_retrieve_boss_translation(
        &self,
        boss_name: &str,
        locale: &str,
    ) -> Result<CodexBoss, Error> {
        let url = format!(
            "{}/codex/bosses/{}/?lang={}",
            self.playorna_host, boss_name, locale
        );
        parse_html_codex_boss_translation(&get_and_save(&self.http, &url)?, boss_name)
    }

    pub(crate) fn codex_retrieve_raid_translation(
        &self,
        raid_name: &str,
        locale: &str,
    ) -> Result<CodexRaid, Error> {
        let url = format!(
            "{}/codex/raids/{}/?lang={}",
            self.playorna_host, raid_name, locale
        );
        parse_html_codex_raid_translation(&get_and_save(&self.http, &url)?, raid_name)
    }

    pub(crate) fn codex_retrieve_item_translation(
        &self,
        item_name: &str,
        locale: &str,
    ) -> Result<CodexItem, Error> {
        let url = format!(
            "{}/codex/items/{}/?lang={}",
            self.playorna_host, item_name, locale
        );
        parse_html_codex_item_translation(&get_and_save(&self.http, &url)?, item_name.to_string())
    }

    pub(crate) fn codex_retrieve_follower_translation(
        &self,
        follower_name: &str,
        locale: &str,
    ) -> Result<CodexFollower, Error> {
        let url = format!(
            "{}/codex/followers/{}/?lang={}",
            self.playorna_host, follower_name, locale
        );
        parse_html_codex_follower_translation(
            &get_and_save(&self.http, &url)?,
            follower_name.to_string(),
        )
    }
}
