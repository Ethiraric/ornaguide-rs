use crate::{
    codex::{Codex, Skill as CodexSkill},
    error::Error,
    guide::{html_form_parser::ParsedForm, http::Http, AdminGuide, Guide, Skill, Spawn},
    items::{admin::AdminItem, raw::RawItem},
    monsters::{admin::AdminMonster, RawMonster},
    skills::{admin::AdminSkill, RawSkill},
};

/// The main interface for the guide.
pub struct OrnaGuide {
    http: Http,
    items: Option<Vec<RawItem>>,
    monsters: Option<Vec<RawMonster>>,
    skills: Option<Vec<RawSkill>>,
}

impl OrnaGuide {
    /// Construct a bare instance of the guide.
    pub fn new() -> Self {
        Self {
            http: Http::new(),
            items: None,
            monsters: None,
            skills: None,
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

    fn fetch_monsters(&mut self) -> Result<&[crate::monsters::RawMonster], Error> {
        if self.monsters.is_none() {
            self.monsters = Some(self.http.fetch_monsters()?);
        }

        Ok(self.monsters.as_ref().unwrap())
    }

    fn get_monsters(&self) -> Option<&[crate::monsters::RawMonster]> {
        self.monsters.as_ref().map(|monster| monster.as_ref())
    }

    fn fetch_skills(&mut self) -> Result<&[RawSkill], Error> {
        if self.skills.is_none() {
            self.skills = Some(self.http.fetch_skills()?);
        }

        Ok(self.skills.as_ref().unwrap())
    }

    fn get_skills(&self) -> Option<&[RawSkill]> {
        self.skills.as_ref().map(|skill| skill.as_ref())
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

    fn fetch_monsters(&mut self) -> Result<&[crate::monsters::RawMonster], Error> {
        self.guide.fetch_monsters()
    }

    fn get_monsters(&self) -> Option<&[crate::monsters::RawMonster]> {
        self.guide.get_monsters()
    }

    fn fetch_skills(&mut self) -> Result<&[RawSkill], Error> {
        self.guide.fetch_skills()
    }

    fn get_skills(&self) -> Option<&[RawSkill]> {
        self.guide.get_skills()
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

    fn admin_retrieve_monster_by_id(
        &self,
        id: u32,
    ) -> Result<crate::monsters::admin::AdminMonster, Error> {
        Ok(AdminMonster {
            id,
            ..AdminMonster::try_from(self.guide.http().admin_retrieve_monster_by_id(id)?)?
        })
    }

    fn admin_save_monster(
        &self,
        monster: crate::monsters::admin::AdminMonster,
    ) -> Result<(), Error> {
        self.guide
            .http()
            .admin_save_monster(monster.id, ParsedForm::from(monster))
    }

    fn admin_retrieve_skill_by_id(&self, id: u32) -> Result<AdminSkill, Error> {
        Ok(AdminSkill {
            id,
            ..AdminSkill::try_from(self.guide.http().admin_retrieve_skill_by_id(id)?)?
        })
    }

    fn admin_save_skill(&self, skill: AdminSkill) -> Result<(), Error> {
        self.guide
            .http()
            .admin_save_skill(skill.id, ParsedForm::from(skill))
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

    fn admin_retrieve_skills_list(&self) -> Result<Vec<Skill>, Error> {
        Ok(self
            .guide
            .http()
            .admin_retrieve_skills_list()?
            .into_iter()
            .map(|entry| Skill {
                id: entry.id,
                name: entry.value,
            })
            .collect())
    }
}

impl Codex for OrnaAdminGuide {
    fn codex_fetch_skill_list(&self) -> Result<Vec<CodexSkill>, Error> {
        Ok(self
            .guide
            .http()
            .codex_retrieve_skills_list()?
            .into_iter()
            .map(|entry| CodexSkill {
                name: entry.value,
                tier: entry.tier,
                uri: entry.uri,
            })
            .collect())
    }
}
