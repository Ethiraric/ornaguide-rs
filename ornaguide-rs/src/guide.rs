use crate::{
    error::Error, items::admin::AdminItem, monsters::admin::AdminMonster, pets::admin::AdminPet,
    skills::admin::AdminSkill,
};

pub(crate) mod html_form_parser;
pub(crate) mod html_list_parser;
pub(crate) mod html_utils;
mod http;
mod ornaguide;
mod r#static;

pub mod fetch;

pub use r#static::{
    Element, EquippedBy, ItemCategory, ItemType, MonsterFamily, SkillType, Spawn, Static,
    StatusEffect, VecElements,
};

/// A skill "row" when listing the skills from the admin guide. It does not contain much details.
#[derive(Debug)]
pub struct SkillRow {
    /// Id of the skill.
    pub id: u32,
    /// Name of the skill.
    pub name: String,
}

/// An item "row" when listing the items from the admin guide. It does not contain much details.
#[derive(Debug)]
pub struct ItemRow {
    /// Id of the item.
    pub id: u32,
    /// Name of the item.
    pub name: String,
}

/// A monster "row" when listing the monsters from the admin guide. It does not contain much details.
#[derive(Debug)]
pub struct MonsterRow {
    /// Id of the monster.
    pub id: u32,
    /// Name of the monster.
    pub name: String,
}

/// A pet "row" when listing the pets from the admin guide. It does not contain much details.
#[derive(Debug)]
pub struct PetRow {
    /// Id of the pet.
    pub id: u32,
    /// Name of the pet.
    pub name: String,
}

/// A read-write access to the administrator panel of the guide.
pub trait AdminGuide {
    /// Retrieve the item with the given id from the guide.
    fn admin_retrieve_item_by_id(&self, id: u32) -> Result<AdminItem, Error>;
    /// Save the given item to the guide.
    fn admin_save_item(&self, item: AdminItem) -> Result<(), Error>;
    /// Retrieve the list of items from the admin view.
    fn admin_retrieve_items_list(&self) -> Result<Vec<ItemRow>, Error>;
    /// Add a new item to the guide.
    /// The csrfmiddlewaretoken and id fields of the provided item will be ignored.
    /// In order to retrieve the id of the new item, all items have to be queried again.
    fn admin_add_item(&self, item: AdminItem) -> Result<(), Error>;

    /// Retrieve the monster with the given id from the guide.
    fn admin_retrieve_monster_by_id(&self, id: u32) -> Result<AdminMonster, Error>;
    /// Save the given monster to the guide.
    fn admin_save_monster(&self, monster: AdminMonster) -> Result<(), Error>;
    /// Retrieve the list of monsters from the admin view.
    fn admin_retrieve_monsters_list(&self) -> Result<Vec<MonsterRow>, Error>;
    /// Add a new monster to the guide.
    /// The csrfmiddlewaretoken and id fields of the provided monster will be ignored.
    /// In order to retrieve the id of the new monster, all monsters have to be queried again.
    fn admin_add_monster(&self, monster: AdminMonster) -> Result<(), Error>;

    /// Retrieve the skill with the given id from the guide.
    fn admin_retrieve_skill_by_id(&self, id: u32) -> Result<AdminSkill, Error>;
    /// Save the given skill to the guide.
    fn admin_save_skill(&self, skill: AdminSkill) -> Result<(), Error>;
    /// Retrieve the list of skills from the admin view.
    fn admin_retrieve_skills_list(&self) -> Result<Vec<SkillRow>, Error>;
    /// Add a new skill to the guide.
    /// The csrfmiddlewaretoken and id fields of the provided skill will be ignored.
    /// In order to retrieve the id of the new skill, all skills have to be queried again.
    fn admin_add_skill(&self, skill: AdminSkill) -> Result<(), Error>;

    /// Retrieve the pet with the given id from the guide.
    fn admin_retrieve_pet_by_id(&self, id: u32) -> Result<AdminPet, Error>;
    /// Save the given pet to the guide.
    fn admin_save_pet(&self, pet: AdminPet) -> Result<(), Error>;
    /// Retrieve the list of pets from the admin view.
    fn admin_retrieve_pets_list(&self) -> Result<Vec<PetRow>, Error>;
    /// Add a new pet to the guide.
    /// The csrfmiddlewaretoken and id fields of the provided pet will be ignored.
    /// In order to retrieve the id of the new pet, all pets have to be queried again.
    fn admin_add_pet(&self, pet: AdminPet) -> Result<(), Error>;

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
    /// Retrieve the list of skill types.
    fn admin_retrieve_skill_types_list(&self) -> Result<Vec<SkillType>, Error>;
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
            skill_types: self.admin_retrieve_skill_types_list()?,
        })
    }

    /// Add a new spawn to the guide.
    /// In order to retrieve the id of the new spawn, all spawns have to be queried again.
    fn admin_add_spawn(&self, spawn_name: &str) -> Result<(), Error>;
    /// Add a new status effect to the guide.
    /// In order to retrieve the id of the new status effect, all status effects have to be queried again.
    fn admin_add_status_effect(&self, status_effect_name: &str) -> Result<(), Error>;
}

pub use ornaguide::{OrnaAdminGuide, OrnaGuide};
