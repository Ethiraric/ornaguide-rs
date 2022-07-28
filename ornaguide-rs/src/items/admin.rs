use serde::{Deserialize, Serialize};

use crate::{error::Error, guide::html_form_parser::ParsedForm};

/// An item fetched from the admin panel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminItem {
    /// The CSRF token that was given on the page where the item was fetched.
    #[serde(skip)]
    pub(crate) csrfmiddlewaretoken: String,
    /// Id of the item on the guide.
    pub id: u32,
    /// The URI of the item on the codex.
    /// URI matches `/codex/items/{slug}/` with the trailing slash.
    pub codex_uri: String,
    /// The name of the item on the guide.
    pub name: String,
    /// The tier of the item.
    pub tier: u8,
    /// The id of the type of the item (Curative, Weapon, Head, Material, ...).
    pub type_: u32,
    /// Path to the image of the item.
    pub image_name: String,
    /// In-game description of the item.
    pub description: String,
    /// Handwritten notes from the guide team on the item.
    pub notes: String,
    /// How much HP the item gives, if equippable.
    pub hp: i16,
    /// Whether HP scales with the quality of the item.
    pub hp_affected_by_quality: bool,
    /// How much mana the item gives, if equippable.
    pub mana: i16,
    /// Whether mana scales with the quality of the item.
    pub mana_affected_by_quality: bool,
    /// How much attack the item gives, if equippable.
    pub attack: i16,
    /// Whether attack scales with the quality of the item.
    pub attack_affected_by_quality: bool,
    /// How much magic the item gives, if equippable.
    pub magic: i16,
    /// Whether magic scales with the quality of the item.
    pub magic_affected_by_quality: bool,
    /// How much defense the item gives, if equippable.
    pub defense: i16,
    /// Whether defense scales with the quality of the item.
    pub defense_affected_by_quality: bool,
    /// How much resistance the item gives, if equippable.
    pub resistance: i16,
    /// Whether resistance scales with the quality of the item.
    pub resistance_affected_by_quality: bool,
    /// How much dexterity the item gives, if equippable.
    pub dexterity: i16,
    /// Whether dexterity scales with the quality of the item.
    pub dexterity_affected_by_quality: bool,
    /// How much ward the item gives, if equippable (%).
    pub ward: i8,
    /// Whether ward scales with the quality of the item.
    pub ward_affected_by_quality: bool,
    /// How much crit the item gives, if equippable.
    pub crit: u8,
    /// Whether crit scales with the quality of the item.
    pub crit_affected_by_quality: bool,
    /// How much foresight the item gives, if equippable.
    pub foresight: i8,
    /// How much view distance the item gives (%).
    pub view_distance: u32,
    /// How much the item increases the stats of your follower (%).
    pub follower_stats: u32,
    /// How much the item increases the action rate of your follower (%).
    pub follower_act: i32,
    /// How much the item increases your status infliction rate (%).
    pub status_infliction: u32,
    /// How much the item increases your status protection rate (%).
    pub status_protection: u32,
    /// How much mana you save with this item, if equippable (%).
    pub mana_saver: i8,
    /// How much more effective potions are, if equippable (%).
    pub potion_effectiveness: u8,
    /// Whether the item has adornment slots, if equippable.
    pub has_slots: bool,
    /// The number of adornment slots of the item at common quality level 10.
    /// Bonus slots are added (assuming level 10+, relative to the base adorn slots):
    ///   * 1 at legendary quality
    ///   * 2 at ornate quality
    ///   * 3 when Masterforged
    ///   * 4 when Godforged
    /// Meaning the maximum adorns that an item can have is this + 4.
    pub base_adornment_slots: u8,
    /// Rarity of the item (based on the background of the item at common quality).
    /// Rarity can be either:
    ///   - `NO`: None
    ///   - `CO`: Common
    ///   - `SP`: Superior (green)
    ///   - `FM`: Famed (blue)
    ///   - `LG`: Legendary (purple)
    pub rarity: String,
    /// Id of the element of the item, if equippable.
    pub element: Option<u32>,
    /// Ids of class categories who can equip the item, if equippable.
    pub equipped_by: Vec<u32>,
    /// Whether the item is two handed, if a weapon.
    pub two_handed: bool,
    /// How much more Orns you gain with this item (%).
    pub orn_bonus: f32,
    /// How much more Gold you gain with this item (%).
    pub gold_bonus: f32,
    /// How much more luck you have with this item (%).
    pub drop_bonus: f32,
    /// How much more spawns there are with this item (%).
    pub spawn_bonus: f32,
    /// How much more experience you gain with this item (%).
    pub exp_bonus: f32,
    /// Whether this item is a boss item (affects scaling and assessing).
    pub boss: bool,
    /// Whether this item is in the arena pool.
    pub arena: bool,
    /// Id of the category of the item, if a weapon (Staffs, Daggers, Polearms, ...).
    pub category: Option<u32>,
    /// Ids of statuses the item can inflict, if equippable.
    pub causes: Vec<u32>,
    /// Ids of statuses the item cures.
    pub cures: Vec<u32>,
    /// Ids of statuses the item gives.
    pub gives: Vec<u32>,
    /// Ids of statuses the item grants immunity to, if equippable.
    pub prevents: Vec<u32>,
    /// Ids of materials the item needs to be upgraded, if upgradable.
    pub materials: Vec<u32>,
    /// Price of the item, if it can be bought from shops.
    pub price: u32,
    /// Off-hand ability, if a weapon.
    pub ability: Option<u32>,
}

impl Default for AdminItem {
    fn default() -> Self {
        AdminItem {
            csrfmiddlewaretoken: String::new(),
            id: 0,
            codex_uri: String::new(),
            name: String::new(),
            tier: 0,
            type_: 13, // Corresponds to "TBD" on guide.
            image_name: String::new(),
            description: String::new(),
            notes: String::new(),
            hp: 0,
            hp_affected_by_quality: false,
            mana: 0,
            mana_affected_by_quality: false,
            attack: 0,
            attack_affected_by_quality: true,
            magic: 0,
            magic_affected_by_quality: true,
            defense: 0,
            defense_affected_by_quality: true,
            resistance: 0,
            resistance_affected_by_quality: true,
            dexterity: 0,
            dexterity_affected_by_quality: false,
            ward: 0,
            ward_affected_by_quality: true,
            crit: 0,
            crit_affected_by_quality: false,
            foresight: 0,
            view_distance: 0,
            follower_stats: 0,
            follower_act: 0,
            status_infliction: 0,
            status_protection: 0,
            mana_saver: 0,
            potion_effectiveness: 0,
            has_slots: false,
            base_adornment_slots: 0,
            rarity: "NO".to_string(),
            element: None,
            equipped_by: Vec::new(),
            two_handed: false,
            orn_bonus: 0.0,
            gold_bonus: 0.0,
            drop_bonus: 0.0,
            spawn_bonus: 0.0,
            exp_bonus: 0.0,
            boss: false,
            arena: false,
            category: None,
            causes: Vec::new(),
            cures: Vec::new(),
            gives: Vec::new(),
            prevents: Vec::new(),
            materials: Vec::new(),
            price: 0,
            ability: None,
        }
    }
}

impl TryFrom<ParsedForm> for AdminItem {
    type Error = Error;

    fn try_from(form: ParsedForm) -> Result<Self, Self::Error> {
        let mut item = AdminItem {
            csrfmiddlewaretoken: form.csrfmiddlewaretoken,
            ..Default::default()
        };

        for (key, value) in form.fields.into_iter() {
            match key.as_str() {
                "codex" => item.codex_uri = value,
                "name" => item.name = value,
                "tier" => item.tier = value.parse()?,
                "type" => item.type_ = value.parse()?,
                "image_name" => item.image_name = value,
                "description" => item.description = value,
                "notes" => item.notes = value,
                "hp" => item.hp = value.parse()?,
                "hp_affected_by_quality" => item.hp_affected_by_quality = value == "on",
                "mana" => item.mana = value.parse()?,
                "mana_affected_by_quality" => item.mana_affected_by_quality = value == "on",
                "attack" => item.attack = value.parse()?,
                "attack_affected_by_quality" => item.attack_affected_by_quality = value == "on",
                "magic" => item.magic = value.parse()?,
                "magic_affected_by_quality" => item.magic_affected_by_quality = value == "on",
                "defense" => item.defense = value.parse()?,
                "defense_affected_by_quality" => item.defense_affected_by_quality = value == "on",
                "resistance" => item.resistance = value.parse()?,
                "resistance_affected_by_quality" => {
                    item.resistance_affected_by_quality = value == "on"
                }
                "dexterity" => item.dexterity = value.parse()?,
                "dexterity_affected_by_quality" => {
                    item.dexterity_affected_by_quality = value == "on"
                }
                "ward" => item.ward = value.parse()?,
                "ward_affected_by_quality" => item.ward_affected_by_quality = value == "on",
                "crit" => item.crit = value.parse()?,
                "crit_affected_by_quality" => item.crit_affected_by_quality = value == "on",
                "foresight" => item.foresight = value.parse()?,
                "view_distance" => item.view_distance = value.parse()?,
                "follower_stats" => item.follower_stats = value.parse()?,
                "follower_act" => item.follower_act = value.parse()?,
                "status_infliction" => item.status_infliction = value.parse()?,
                "status_protection" => item.status_protection = value.parse()?,
                "mana_saver" => item.mana_saver = value.parse()?,
                "potion_effectiveness" => item.potion_effectiveness = value.parse()?,
                "has_slots" => item.has_slots = value == "on",
                "base_adornment_slots" => item.base_adornment_slots = value.parse()?,
                "rarity" => item.rarity = value,
                "element" => {
                    item.element = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                "equipped_by" => item.equipped_by.push(value.parse()?),
                "two_handed" => item.two_handed = value == "on",
                "orn_bonus" => item.orn_bonus = value.parse()?,
                "gold_bonus" => item.gold_bonus = value.parse()?,
                "drop_bonus" => item.drop_bonus = value.parse()?,
                "spawn_bonus" => item.spawn_bonus = value.parse()?,
                "exp_bonus" => item.exp_bonus = value.parse()?,
                "boss" => item.boss = value == "on",
                "arena" => item.arena = value == "on",
                "category" => {
                    item.category = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                "causes" => item.causes.push(value.parse()?),
                "cures" => item.cures.push(value.parse()?),
                "gives" => item.gives.push(value.parse()?),
                "prevents" => item.prevents.push(value.parse()?),
                "materials" => item.materials.push(value.parse()?),
                "price" => item.price = value.parse()?,
                "ability" => {
                    item.ability = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                key => {
                    return Err(Error::ExtraField(key.to_string(), value));
                }
            }
        }

        Ok(item)
    }
}

impl From<AdminItem> for ParsedForm {
    fn from(item: AdminItem) -> Self {
        let mut form = ParsedForm {
            csrfmiddlewaretoken: item.csrfmiddlewaretoken,
            ..ParsedForm::default()
        };

        let mut push = |key: &str, value: String| form.fields.push((key.to_string(), value));

        push("codex", item.codex_uri);
        push("name", item.name);
        push("tier", item.tier.to_string());
        push("type", item.type_.to_string());
        push("image_name", item.image_name);
        push("description", item.description);
        push("notes", item.notes);

        push("hp", item.hp.to_string());
        if item.hp_affected_by_quality {
            push("hp_affected_by_quality", "on".to_string());
        }
        push("mana", item.mana.to_string());
        if item.mana_affected_by_quality {
            push("mana_affected_by_quality", "on".to_string());
        }
        push("attack", item.attack.to_string());
        if item.attack_affected_by_quality {
            push("attack_affected_by_quality", "on".to_string());
        }
        push("magic", item.magic.to_string());
        if item.magic_affected_by_quality {
            push("magic_affected_by_quality", "on".to_string());
        }
        push("defense", item.defense.to_string());
        if item.defense_affected_by_quality {
            push("defense_affected_by_quality", "on".to_string());
        }
        push("resistance", item.resistance.to_string());
        if item.resistance_affected_by_quality {
            push("resistance_affected_by_quality", "on".to_string());
        }
        push("dexterity", item.dexterity.to_string());
        if item.dexterity_affected_by_quality {
            push("dexterity_affected_by_quality", "on".to_string());
        }
        push("ward", item.ward.to_string());
        if item.ward_affected_by_quality {
            push("ward_affected_by_quality", "on".to_string());
        }
        push("crit", item.crit.to_string());
        if item.crit_affected_by_quality {
            push("crit_affected_by_quality", "on".to_string());
        }

        push("foresight", item.foresight.to_string());
        push("view_distance", item.view_distance.to_string());
        push("follower_stats", item.follower_stats.to_string());
        push("follower_act", item.follower_act.to_string());
        push("status_infliction", item.status_infliction.to_string());
        push("status_protection", item.status_protection.to_string());
        push("mana_saver", item.mana_saver.to_string());
        push(
            "potion_effectiveness",
            item.potion_effectiveness.to_string(),
        );

        if item.has_slots {
            push("has_slots", "on".to_string());
        }
        push(
            "base_adornment_slots",
            item.base_adornment_slots.to_string(),
        );
        push("rarity", item.rarity.to_string());

        if let Some(element) = item.element {
            push("element", element.to_string());
        } else {
            push("element", String::new());
        }
        for x in item.equipped_by.iter() {
            push("equipped_by", x.to_string());
        }
        if item.two_handed {
            push("two_handed", "on".to_string());
        }

        push("orn_bonus", item.orn_bonus.to_string());
        push("gold_bonus", item.gold_bonus.to_string());
        push("drop_bonus", item.drop_bonus.to_string());
        push("spawn_bonus", item.spawn_bonus.to_string());
        push("exp_bonus", item.exp_bonus.to_string());

        if item.boss {
            push("boss", "on".to_string());
        }
        if item.arena {
            push("arena", "on".to_string());
        }
        if let Some(category) = item.category {
            push("category", category.to_string());
        } else {
            push("category", String::new());
        }

        for x in item.causes.iter() {
            push("causes", x.to_string());
        }
        for x in item.cures.iter() {
            push("cures", x.to_string());
        }
        for x in item.gives.iter() {
            push("gives", x.to_string());
        }
        for x in item.prevents.iter() {
            push("prevents", x.to_string());
        }

        for x in item.materials.iter() {
            push("materials", x.to_string());
        }
        push("price", item.price.to_string());
        if let Some(ability) = item.ability {
            push("ability", ability.to_string());
        } else {
            push("ability", String::new());
        }

        form
    }
}

/// Collection of items from the guide's admin view.
#[derive(Serialize, Deserialize)]
pub struct AdminItems {
    /// Items from the guide's admin view.
    pub items: Vec<AdminItem>,
}

impl<'a> AdminItems {
    /// Find the admin item associated with the given id.
    pub fn find_by_id(&'a self, needle: u32) -> Option<&'a AdminItem> {
        self.items.iter().find(|item| item.id == needle)
    }

    /// Find the admin item associated with the given id.
    /// If there is no match, return an `Err`.
    pub fn get_by_id(&'a self, needle: u32) -> Result<&'a AdminItem, Error> {
        self.find_by_id(needle)
            .ok_or_else(|| Error::Misc(format!("No match for admin item with id {}", needle)))
    }

    /// Find the admin item associated with the given uri.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a AdminItem> {
        self.items.iter().find(|item| item.codex_uri == needle)
    }

    /// Find the admin item associated with the given uri.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a AdminItem, Error> {
        self.find_by_uri(needle).ok_or_else(|| {
            Error::Misc(format!("No match for admin item with codex_uri {}", needle))
        })
    }

    /// Find the admin item associated with the given slug.
    pub fn find_by_slug(&'a self, needle: &str) -> Option<&'a AdminItem> {
        self.items.iter().find(|item| {
            !item.codex_uri.is_empty()
                && item.codex_uri["/codex/items/".len()..].trim_end_matches('/') == needle
        })
    }

    /// Find the admin item associated with the given slug.
    /// If there is no match, return an `Err`.
    pub fn get_by_slug(&'a self, needle: &str) -> Result<&'a AdminItem, Error> {
        self.find_by_slug(needle).ok_or_else(|| {
            Error::Misc(format!(
                "No match for admin item with codex slug {}",
                needle
            ))
        })
    }
}
