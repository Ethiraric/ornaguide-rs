use crate::{
    codex::{
        BossEntry as CodexBossEntry, Codex, CodexMonster, CodexSkill,
        FollowerEntry as CodexFollowerEntry, ItemEntry as CodexItemEntry,
        MonsterEntry as CodexMonsterEntry, RaidEntry as CodexRaidEntry,
        SkillEntry as CodexSkillEntry,
    },
    error::{Error, Kind},
    guide::{
        html_form_parser::ParsedForm, http::Http, AdminGuide, Element, EquippedBy, ItemCategory,
        ItemRow, ItemType, MonsterFamily, MonsterRow, PetRow, SkillRow, SkillType, Spawn,
        StatusEffect,
    },
    items::admin::AdminItem,
    monsters::admin::AdminMonster,
    pets::admin::AdminPet,
    skills::admin::AdminSkill,
    utils::block_on_this_thread,
};

/// The main interface for the guide.
pub struct OrnaGuide {
    http: Http,
}

impl OrnaGuide {
    /// Construct a bare instance of the guide.
    #[must_use]
    pub fn new() -> Self {
        Self { http: Http::new() }
    }

    /// Construct an instance of the guide from an existing http session.
    /// This can be use to "subclass" the guide.
    fn from_http(http: Http) -> Self {
        Self { http }
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

pub struct OrnaAdminGuide {
    guide: OrnaGuide,
}

impl OrnaAdminGuide {
    /// Construct a bare instance of the guide.
    ///
    /// # Errors
    /// Errors if the HTTP client could not be built.
    pub fn new(cookie: &str) -> Result<Self, Error> {
        Ok(Self {
            guide: OrnaGuide::from_http(Http::new_with_cookie(cookie)?),
        })
    }

    /// Construct an instance of the guide with the given hosts.
    ///
    /// # Errors
    /// Errors if the HTTP client could not be built.
    pub fn new_with_hosts(
        cookie: &str,
        orna_guide: String,
        playorna: String,
    ) -> Result<Self, Error> {
        Ok(Self {
            guide: OrnaGuide::from_http(Http::new_with_cookie_and_hosts(
                cookie, orna_guide, playorna,
            )?),
        })
    }

    /// Retrieve the item with the given id from the guide (asynchronous).
    ///
    /// # Errors
    /// Errors on I/O, HTTP or parsing error.
    pub async fn async_admin_retrieve_item_by_id(&self, id: u32) -> Result<AdminItem, Error> {
        Ok(AdminItem {
            id,
            ..AdminItem::try_from(
                self.guide
                    .http()
                    .async_admin_retrieve_item_by_id(id)
                    .await?,
            )
            .map_err(|e| e.ctx_push(format!("While parsing item {id}")))?
        })
    }

    /// Retrieve the monster with the given id from the guide (asynchronous).
    ///
    /// # Errors
    /// Errors on I/O, HTTP or parsing error.
    pub async fn async_admin_retrieve_monster_by_id(&self, id: u32) -> Result<AdminMonster, Error> {
        Ok(AdminMonster {
            id,
            ..AdminMonster::try_from(
                self.guide
                    .http()
                    .async_admin_retrieve_monster_by_id(id)
                    .await?,
            )
            .map_err(|e| e.ctx_push(format!("While parsing monster {id}")))?
        })
    }

    /// Retrieve the skill with the given id from the guide (asynchronous).
    ///
    /// # Errors
    /// Errors on I/O, HTTP or parsing error.
    pub async fn async_admin_retrieve_skill_by_id(&self, id: u32) -> Result<AdminSkill, Error> {
        Ok(AdminSkill {
            id,
            ..AdminSkill::try_from(
                self.guide
                    .http()
                    .async_admin_retrieve_skill_by_id(id)
                    .await?,
            )
            .map_err(|e| e.ctx_push(format!("While parsing skill {id}")))?
        })
    }

    /// Retrieve the pet with the given id from the guide (asynchronous).
    ///
    /// # Errors
    /// Errors on I/O, HTTP or parsing error.
    pub async fn async_admin_retrieve_pet_by_id(&self, id: u32) -> Result<AdminPet, Error> {
        Ok(AdminPet {
            id,
            ..AdminPet::try_from(self.guide.http().async_admin_retrieve_pet_by_id(id).await?)
                .map_err(|e| e.ctx_push(format!("While parsing pet {id}")))?
        })
    }
}

impl AdminGuide for OrnaAdminGuide {
    fn admin_retrieve_item_by_id(&self, id: u32) -> Result<AdminItem, Error> {
        block_on_this_thread(self.async_admin_retrieve_item_by_id(id))
    }

    fn admin_save_item(&self, item: AdminItem) -> Result<(), Error> {
        self.guide
            .http()
            .admin_save_item(item.id, ParsedForm::from(item))
    }

    fn admin_retrieve_items_list(&self) -> Result<Vec<ItemRow>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_items_list()?
            .into_iter()
            .map(|entry| ItemRow {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_add_item(&self, item: AdminItem) -> Result<(), Error> {
        self.guide.http().admin_add_item(ParsedForm::from(item))
    }

    fn admin_retrieve_monster_by_id(&self, id: u32) -> Result<AdminMonster, Error> {
        block_on_this_thread(self.async_admin_retrieve_monster_by_id(id))
    }

    fn admin_save_monster(
        &self,
        monster: crate::monsters::admin::AdminMonster,
    ) -> Result<(), Error> {
        self.guide
            .http()
            .admin_save_monster(monster.id, ParsedForm::from(monster))
    }

    fn admin_retrieve_monsters_list(&self) -> Result<Vec<MonsterRow>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_monsters_list()?
            .into_iter()
            .map(|entry| MonsterRow {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_add_monster(&self, monster: AdminMonster) -> Result<(), Error> {
        self.guide
            .http()
            .admin_add_monster(ParsedForm::from(monster))
    }

    fn admin_retrieve_skill_by_id(&self, id: u32) -> Result<AdminSkill, Error> {
        block_on_this_thread(self.async_admin_retrieve_skill_by_id(id))
    }

    fn admin_save_skill(&self, skill: AdminSkill) -> Result<(), Error> {
        self.guide
            .http()
            .admin_save_skill(skill.id, ParsedForm::from(skill))
    }

    fn admin_retrieve_skills_list(&self) -> Result<Vec<SkillRow>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_skills_list()?
            .into_iter()
            .map(|entry| SkillRow {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_add_skill(&self, skill: AdminSkill) -> Result<(), Error> {
        self.guide.http().admin_add_skill(ParsedForm::from(skill))
    }

    fn admin_retrieve_pet_by_id(&self, id: u32) -> Result<AdminPet, Error> {
        block_on_this_thread(self.async_admin_retrieve_pet_by_id(id))
    }

    fn admin_save_pet(&self, pet: AdminPet) -> Result<(), Error> {
        self.guide
            .http()
            .admin_save_pet(pet.id, ParsedForm::from(pet))
    }

    fn admin_retrieve_pets_list(&self) -> Result<Vec<PetRow>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_pets_list()?
            .into_iter()
            .map(|entry| PetRow {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_add_pet(&self, pet: AdminPet) -> Result<(), Error> {
        self.guide.http().admin_add_pet(ParsedForm::from(pet))
    }

    fn admin_retrieve_spawns_list(&self) -> Result<Vec<Spawn>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_spawns_list()?
            .into_iter()
            .map(|entry| Spawn {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_retrieve_item_categories_list(&self) -> Result<Vec<ItemCategory>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_item_categories_list()?
            .into_iter()
            .map(|entry| ItemCategory {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_retrieve_item_types_list(&self) -> Result<Vec<ItemType>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_item_types_list()?
            .into_iter()
            .map(|entry| ItemType {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_retrieve_monster_families_list(&self) -> Result<Vec<MonsterFamily>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_monster_families_list()?
            .into_iter()
            .map(|entry| MonsterFamily {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_retrieve_status_effects_list(&self) -> Result<Vec<StatusEffect>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_status_effects_list()?
            .into_iter()
            .map(|entry| StatusEffect {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_retrieve_elements_list(&self) -> Vec<Element> {
        vec![
            Element {
                id: 1,
                name: "Fire".to_string(),
            },
            Element {
                id: 2,
                name: "Water".to_string(),
            },
            Element {
                id: 3,
                name: "Lightning".to_string(),
            },
            Element {
                id: 4,
                name: "Earthen".to_string(),
            },
            Element {
                id: 5,
                name: "Holy".to_string(),
            },
            Element {
                id: 6,
                name: "Dark".to_string(),
            },
            Element {
                id: 7,
                name: "Dragon".to_string(),
            },
            Element {
                id: 9,
                name: "Physical".to_string(),
            },
            Element {
                id: 11,
                name: "Arcane".to_string(),
            },
        ]
    }

    fn admin_retrieve_equipped_bys_list(&self) -> Vec<EquippedBy> {
        vec![
            EquippedBy {
                id: 1,
                name: "Warrior".to_string(),
            },
            EquippedBy {
                id: 2,
                name: "Mage".to_string(),
            },
            EquippedBy {
                id: 3,
                name: "Thief".to_string(),
            },
        ]
    }

    fn admin_retrieve_skill_types_list(&self) -> Result<Vec<SkillType>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_skill_types_list()?
            .into_iter()
            .map(|entry| SkillType {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }

    fn admin_add_spawn(&self, spawn_name: &str) -> Result<(), Error> {
        self.guide.http().admin_add_spawn(spawn_name)
    }

    fn admin_add_status_effect(&self, status_effect_name: &str) -> Result<(), Error> {
        self.guide
            .http()
            .admin_add_status_effect(status_effect_name)
    }
}

impl Codex for OrnaAdminGuide {
    fn codex_fetch_skill_list(&self) -> Result<Vec<CodexSkillEntry>, Error> {
        Ok(self
            .guide
            .http()
            .codex_retrieve_skills_list()?
            .into_iter()
            .map(|entry| CodexSkillEntry {
                name: entry.value,
                tier: entry.tier,
                uri: entry.uri,
            })
            .collect())
    }

    fn codex_fetch_skill_page(&self, skill_name: &str) -> Result<String, Error> {
        self.guide.http().codex_retrieve_skill_page(skill_name)
    }

    fn codex_fetch_skill(&self, skill_name: &str) -> Result<CodexSkill, Error> {
        self.guide.http().codex_retrieve_skill(skill_name)
    }

    fn codex_fetch_monster_list(&self) -> Result<Vec<CodexMonsterEntry>, Error> {
        self.guide
            .http()
            .codex_retrieve_monsters_list()?
            .into_iter()
            .map(|entry| {
                Ok(CodexMonsterEntry {
                    name: entry.value,
                    family: entry.meta.ok_or_else(|| {
                        Kind::HTMLParsingError(
                            "Failed to retrieve meta field of monster".to_string(),
                        )
                    })?,
                    tier: entry.tier,
                    uri: entry.uri,
                })
            })
            .collect()
    }

    fn codex_fetch_monster_page(&self, monster_name: &str) -> Result<String, Error> {
        self.guide.http().codex_retrieve_monster_page(monster_name)
    }

    fn codex_fetch_monster(&self, monster_name: &str) -> Result<CodexMonster, Error> {
        self.guide.http().codex_retrieve_monster(monster_name)
    }

    fn codex_fetch_boss_list(&self) -> Result<Vec<CodexBossEntry>, Error> {
        self.guide
            .http()
            .codex_retrieve_bosses_list()?
            .into_iter()
            .map(|entry| {
                Ok(CodexBossEntry {
                    name: entry.value,
                    family: entry.meta.ok_or_else(|| {
                        Kind::HTMLParsingError("Failed to retrieve meta field of boss".to_string())
                    })?,
                    tier: entry.tier,
                    uri: entry.uri,
                })
            })
            .collect()
    }

    fn codex_fetch_boss_page(&self, boss_name: &str) -> Result<String, Error> {
        self.guide.http().codex_retrieve_boss_page(boss_name)
    }

    fn codex_fetch_boss(&self, boss_name: &str) -> Result<crate::codex::CodexBoss, Error> {
        self.guide.http().codex_retrieve_boss(boss_name)
    }

    fn codex_fetch_raid_list(&self) -> Result<Vec<CodexRaidEntry>, Error> {
        self.guide
            .http()
            .codex_retrieve_raids_list()?
            .into_iter()
            .map(|entry| {
                Ok(CodexRaidEntry {
                    name: entry.value,
                    tier: entry.tier,
                    uri: entry.uri,
                })
            })
            .collect()
    }

    fn codex_fetch_raid_page(&self, raid_name: &str) -> Result<String, Error> {
        self.guide.http().codex_retrieve_raid_page(raid_name)
    }

    fn codex_fetch_raid(&self, raid_name: &str) -> Result<crate::codex::CodexRaid, Error> {
        self.guide.http().codex_retrieve_raid(raid_name)
    }

    fn codex_fetch_item_list(&self) -> Result<Vec<CodexItemEntry>, Error> {
        self.guide
            .http()
            .codex_retrieve_items_list()?
            .into_iter()
            .map(|entry| {
                Ok(CodexItemEntry {
                    name: entry.value,
                    tier: entry.tier,
                    uri: entry.uri,
                })
            })
            .collect()
    }

    fn codex_fetch_item_page(&self, item_name: &str) -> Result<String, Error> {
        self.guide.http().codex_retrieve_item_page(item_name)
    }

    fn codex_fetch_item(&self, item_name: &str) -> Result<crate::codex::CodexItem, Error> {
        self.guide.http().codex_retrieve_item(item_name)
    }

    fn codex_fetch_follower_list(&self) -> Result<Vec<crate::codex::FollowerEntry>, Error> {
        self.guide
            .http()
            .codex_retrieve_followers_list()?
            .into_iter()
            .map(|entry| {
                Ok(CodexFollowerEntry {
                    name: entry.value,
                    tier: entry.tier,
                    uri: entry.uri,
                })
            })
            .collect()
    }

    fn codex_fetch_follower_page(&self, follower_name: &str) -> Result<String, Error> {
        self.guide
            .http()
            .codex_retrieve_follower_page(follower_name)
    }

    fn codex_fetch_follower(
        &self,
        follower_name: &str,
    ) -> Result<crate::codex::CodexFollower, Error> {
        self.guide.http().codex_retrieve_follower(follower_name)
    }

    fn codex_fetch_skill_with_locale(
        &self,
        skill_name: &str,
        locale: &str,
    ) -> Result<CodexSkill, Error> {
        self.guide
            .http()
            .codex_retrieve_skill_translation(skill_name, locale)
    }

    fn codex_fetch_monster_with_locale(
        &self,
        monster_name: &str,
        locale: &str,
    ) -> Result<CodexMonster, Error> {
        self.guide
            .http()
            .codex_retrieve_monster_translation(monster_name, locale)
    }

    fn codex_fetch_boss_with_locale(
        &self,
        boss_name: &str,
        locale: &str,
    ) -> Result<crate::codex::CodexBoss, Error> {
        self.guide
            .http()
            .codex_retrieve_boss_translation(boss_name, locale)
    }

    fn codex_fetch_raid_with_locale(
        &self,
        raid_name: &str,
        locale: &str,
    ) -> Result<crate::codex::CodexRaid, Error> {
        self.guide
            .http()
            .codex_retrieve_raid_translation(raid_name, locale)
    }

    fn codex_fetch_item_with_locale(
        &self,
        item_name: &str,
        locale: &str,
    ) -> Result<crate::codex::CodexItem, Error> {
        self.guide
            .http()
            .codex_retrieve_item_translation(item_name, locale)
    }

    fn codex_fetch_follower_with_locale(
        &self,
        follower_name: &str,
        locale: &str,
    ) -> Result<crate::codex::CodexFollower, Error> {
        self.guide
            .http()
            .codex_retrieve_follower_translation(follower_name, locale)
    }
}
