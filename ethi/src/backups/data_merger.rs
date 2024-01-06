use std::collections::HashMap;

use ornaguide_rs::{
    codex::{
        CodexBoss, CodexBosses, CodexFollower, CodexFollowers, CodexItem, CodexItems, CodexMonster,
        CodexMonsters, CodexRaid, CodexRaids, CodexSkill, CodexSkills,
    },
    data::{CodexData, GuideData, OrnaData},
    guide::{
        Element, EquippedBy, ItemCategory, ItemType, MonsterFamily, SkillType, Spawn, Static,
        StatusEffect,
    },
    items::admin::{AdminItem, AdminItems},
    monsters::admin::{AdminMonster, AdminMonsters},
    pets::admin::{AdminPet, AdminPets},
    skills::admin::{AdminSkill, AdminSkills},
};

/// A structure capable of aggregating multiple `OrnaData`s and summing them all into one
/// `OrnaData` that contains every entry met.
/// Newer entries replace older ones.
#[derive(Default)]
pub struct DataMerger {
    /// Guide data.
    pub guide: GuideDataMerger,
    /// Codex data.
    pub codex: CodexDataMerger,
}

/// A structure capable of aggregating multiple `GuideData`s and summing them all into one
/// `GuideData` that contains every entry met.
/// Newer entries replace older ones.
#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct GuideDataMerger {
    /// All items encountered until now, hashed by their admin ID.
    pub items: HashMap<u32, AdminItem>,
    /// All monsters encountered until now, hashed by their admin ID.
    pub monsters: HashMap<u32, AdminMonster>,
    /// All skills encountered until now, hashed by their admin ID.
    pub skills: HashMap<u32, AdminSkill>,
    /// All pets encountered until now, hashed by their admin ID.
    pub pets: HashMap<u32, AdminPet>,
    /// All spawns encountered until now, hashed by their admin ID.
    pub spawns: HashMap<u32, Spawn>,
    /// All item categories encountered until now, hashed by their admin ID.
    pub item_categories: HashMap<u32, ItemCategory>,
    /// All item types encountered until now, hashed by their admin ID.
    pub item_types: HashMap<u32, ItemType>,
    /// All monster families encountered until now, hashed by their admin ID.
    pub monster_families: HashMap<u32, MonsterFamily>,
    /// All status effects encountered until now, hashed by their admin ID.
    pub status_effects: HashMap<u32, StatusEffect>,
    /// All elements encountered until now, hashed by their admin ID.
    pub elements: HashMap<u32, Element>,
    /// All equipped bys until now, hashed by their admin ID.
    pub equipped_bys: HashMap<u32, EquippedBy>,
    /// All skill types until now, hashed by their admin ID.
    pub skill_types: HashMap<u32, SkillType>,
}

/// A structure capable of aggregating multiple `CodexData`s and summing them all into one
/// `CodexData` that contains every entry met.
/// Newer entries replace older ones.
#[allow(clippy::module_name_repetitions)]
#[derive(Default)]
pub struct CodexDataMerger {
    /// All items encountered until now, hashed by their URI.
    pub items: HashMap<String, CodexItem>,
    /// All raids encountered until now, hashed by their URI.
    pub raids: HashMap<String, CodexRaid>,
    /// All monsters encountered until now, hashed by their URI.
    pub monsters: HashMap<String, CodexMonster>,
    /// All bosses encountered until now, hashed by their URI.
    pub bosses: HashMap<String, CodexBoss>,
    /// All skills encountered until now, hashed by their URI.
    pub skills: HashMap<String, CodexSkill>,
    /// All followers encountered until now, hashed by their URI.
    pub followers: HashMap<String, CodexFollower>,
}

impl DataMerger {
    /// Merge an `OrnaData` into `self`.
    /// If entries are present into both `self` and `data`, the value of `self` is updated with
    /// that of `data`.
    pub fn merge_with(&mut self, data: OrnaData) {
        self.guide.merge_with(data.guide);
        self.codex.merge_with(data.codex);
    }
    /// Consume `self` and aggregate data to a `OrnaData`.
    pub fn into_orna_data(self) -> OrnaData {
        OrnaData {
            codex: self.codex.into_codex_data(),
            guide: self.guide.into_guide_data(),
        }
    }
}

impl GuideDataMerger {
    /// Merge a `GuideData` into `self`.
    /// If entries are present into both `self` and `data`, the value of `self` is updated with
    /// that of `data`.
    pub fn merge_with(&mut self, data: GuideData) {
        for item in data.items.items {
            self.items.insert(item.id, item);
        }
        for monster in data.monsters.monsters {
            self.monsters.insert(monster.id, monster);
        }
        for skill in data.skills.skills {
            self.skills.insert(skill.id, skill);
        }
        for pet in data.pets.pets {
            self.pets.insert(pet.id, pet);
        }
        for spawn in data.static_.spawns {
            self.spawns.insert(spawn.id, spawn);
        }
        for item_category in data.static_.item_categories {
            self.item_categories.insert(item_category.id, item_category);
        }
        for item_type in data.static_.item_types {
            self.item_types.insert(item_type.id, item_type);
        }
        for monster_family in data.static_.monster_families {
            self.monster_families
                .insert(monster_family.id, monster_family);
        }
        for status_effect in data.static_.status_effects {
            self.status_effects.insert(status_effect.id, status_effect);
        }
        for element in data.static_.elements {
            self.elements.insert(element.id, element);
        }
        for equipped_by in data.static_.equipped_bys {
            self.equipped_bys.insert(equipped_by.id, equipped_by);
        }
        for skill_type in data.static_.skill_types {
            self.skill_types.insert(skill_type.id, skill_type);
        }
    }

    /// Consume `self` and aggregate data to a `GuideData`.
    pub fn into_guide_data(self) -> GuideData {
        GuideData {
            items: AdminItems {
                items: self.items.into_values().collect(),
            },
            monsters: AdminMonsters {
                monsters: self.monsters.into_values().collect(),
            },
            skills: AdminSkills {
                skills: self.skills.into_values().collect(),
            },
            pets: AdminPets {
                pets: self.pets.into_values().collect(),
            },
            static_: Static {
                spawns: self.spawns.into_values().collect(),
                item_categories: self.item_categories.into_values().collect(),
                item_types: self.item_types.into_values().collect(),
                monster_families: self.monster_families.into_values().collect(),
                status_effects: self.status_effects.into_values().collect(),
                elements: self.elements.into_values().collect(),
                equipped_bys: self.equipped_bys.into_values().collect(),
                skill_types: self.skill_types.into_values().collect(),
            },
        }
    }
}

impl CodexDataMerger {
    /// Merge a `CodexData` into `self`.
    /// If entries are present into both `self` and `data`, the value of `self` is updated with
    /// that of `data`.
    pub fn merge_with(&mut self, data: CodexData) {
        for item in data.items.items {
            self.items.insert(item.slug.clone(), item);
        }
        for raid in data.raids.raids {
            self.raids.insert(raid.slug.clone(), raid);
        }
        for monster in data.monsters.monsters {
            self.monsters.insert(monster.slug.clone(), monster);
        }
        for boss in data.bosses.bosses {
            // This entry got changed. The slug went to `immortal-lord-418e5cff`.
            if boss.slug != "immortal-lord" {
                self.bosses.insert(boss.slug.clone(), boss);
            }
        }
        for skill in data.skills.skills {
            self.skills.insert(skill.slug.clone(), skill);
        }
        for follower in data.followers.followers {
            self.followers.insert(follower.slug.clone(), follower);
        }
    }

    /// Consume `self` and aggregate data to a `CodexData`.
    pub fn into_codex_data(self) -> CodexData {
        CodexData {
            items: CodexItems {
                items: self.items.into_values().collect(),
            },
            raids: CodexRaids {
                raids: self.raids.into_values().collect(),
            },
            monsters: CodexMonsters {
                monsters: self.monsters.into_values().collect(),
            },
            bosses: CodexBosses {
                bosses: self.bosses.into_values().collect(),
            },
            skills: CodexSkills {
                skills: self.skills.into_values().collect(),
            },
            followers: CodexFollowers {
                followers: self.followers.into_values().collect(),
            },
        }
    }
}
