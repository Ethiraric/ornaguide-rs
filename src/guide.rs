use crate::{
    error::Error,
    items::{admin::AdminItem, raw::RawItem},
    monsters::{admin::AdminMonster, raw::RawMonster},
    skills::{admin::AdminSkill, raw::RawSkill},
};

mod cache;
pub(crate) mod html_form_parser;
pub(crate) mod html_list_parser;
mod http;
mod ornaguide;
mod r#static;

pub use r#static::{
    Element, EquippedBy, ItemCategory, ItemType, MonsterFamily, Spawn, Static, StatusEffect,
};

/// A skill "row" when listing the skills from the admin guide. It does not contain much details.
#[derive(Debug)]
pub struct SkillRow {
    /// Id of the skill.
    pub id: u32,
    /// Name of the skill.
    pub name: String,
}

/// A source of information from the game. On the site, this represents the public API.
/// Note that the info can be fetched locally from a cache.
pub trait Guide {
    /// If not already done, query the API of the guide for the list of items and store it in a
    /// cache. If the cache is already fetched, return it. The latter case cannot return an `Err`.
    fn fetch_items(&mut self) -> Result<&[RawItem], Error>;

    /// Return the cache, if already fetched. This method will always return `None` before a call
    /// to `fetch_items`.
    fn get_items(&self) -> Option<&[RawItem]>;

    /// If not already done, query the API of the guide for the list of monsters and store it in a
    /// cache. If the cache is already fetched, return it. The latter case cannot return an `Err`.
    fn fetch_monsters(&mut self) -> Result<&[RawMonster], Error>;

    /// Return the cache, if already fetched. This method will always return `None` before a call
    /// to `fetch_monsters`.
    fn get_monsters(&self) -> Option<&[RawMonster]>;

    /// If not already done, query the API of the guide for the list of skills and store it in a
    /// cache. If the cache is already fetched, return it. The latter case cannot return an `Err`.
    fn fetch_skills(&mut self) -> Result<&[RawSkill], Error>;

    /// Return the cache, if already fetched. This method will always return `None` before a call
    /// to `fetch_skills`.
    fn get_skills(&self) -> Option<&[RawSkill]>;
}

/// A read-write access to the administrator panel of the guide.
pub trait AdminGuide {
    /// Retrieve the item with the given id from the guide.
    fn admin_retrieve_item_by_id(&self, id: u32) -> Result<AdminItem, Error>;
    /// Save the given item to the guide.
    fn admin_save_item(&self, item: AdminItem) -> Result<(), Error>;

    /// Retrieve the monster with the given id from the guide.
    fn admin_retrieve_monster_by_id(&self, id: u32) -> Result<AdminMonster, Error>;
    /// Save the given monster to the guide.
    fn admin_save_monster(&self, monster: AdminMonster) -> Result<(), Error>;

    /// Retrieve the skill with the given id from the guide.
    fn admin_retrieve_skill_by_id(&self, id: u32) -> Result<AdminSkill, Error>;
    /// Save the given skill to the guide.
    fn admin_save_skill(&self, skill: AdminSkill) -> Result<(), Error>;
    /// Retrieve the list of skills from the admin view.
    fn admin_retrieve_skills_list(&self) -> Result<Vec<SkillRow>, Error>;

    /// Retrieve the list of spawns from the admin view.
    fn admin_retrieve_spawns_list(&self) -> Result<Vec<Spawn>, Error>;
    /// Retrieve the list of item categories from the admin view.
    fn admin_retrieve_item_categories_list(&self) -> Result<Vec<ItemCategory>, Error>;
    /// Retrieve the list of item types from the admin view.
    fn admin_retrieve_item_types_list(&self) -> Result<Vec<ItemType>, Error>;
    /// Retrieve the list of monster families from the admin view.
    fn admin_retrieve_monster_families_list(&self) -> Result<Vec<MonsterFamily>, Error>;
    /// Retrieve the list of status effects from the admin view.
    fn admin_retrieve_status_effects_list(&self) -> Result<Vec<StatusEffect>, Error>;
    /// Retrieve the list of elements.
    fn admin_retrieve_elements_list(&self) -> Vec<Element>;
    /// Retrieve the list of `equipped_by`s.
    fn admin_retrieve_equipped_bys_list(&self) -> Vec<EquippedBy>;
    /// Retrieve all static resources from the admin view.
    fn admin_retrieve_static_resources(&self) -> Result<Static, Error> {
        Ok(Static {
            spawns: self.admin_retrieve_spawns_list()?,
            item_categories: self.admin_retrieve_item_categories_list()?,
            item_types: self.admin_retrieve_item_types_list()?,
            monster_families: self.admin_retrieve_monster_families_list()?,
            status_effects: self.admin_retrieve_status_effects_list()?,
            elements: self.admin_retrieve_elements_list(),
            equipped_bys: self.admin_retrieve_equipped_bys_list(),
        })
    }
}

pub use cache::CachedGuide;
pub use ornaguide::{OrnaAdminGuide, OrnaGuide};
