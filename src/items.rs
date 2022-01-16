use crate::{
    error::Error,
    raw_items::{self, RawItem},
};

pub type ItemDroppedBy = raw_items::ItemDroppedBy;
pub type ItemEquippedBy = raw_items::ItemEquippedBy;
pub type ItemMaterial = raw_items::ItemMaterial;
pub type ItemQuest = raw_items::ItemQuest;
pub type ItemStats = raw_items::ItemStats;

/// An item in Orna. This enum splits into types the different items.
pub enum Item {
    /// An armor item.
    Armor(Armor),
    /// A weapon item.
    Weapon(Weapon),
}

impl TryFrom<RawItem> for Item {
    type Error = Error;

    fn try_from(raw_item: RawItem) -> Result<Self, Self::Error> {
        match raw_item.type_.as_str() {
            "Armor" => Ok(Self::Armor(Armor::try_from(raw_item)?)),
            "Weapon" => Ok(Self::Weapon(Weapon::try_from(raw_item)?)),
            _ => Err(Error::InvalidField(
                String::from("Item"),
                String::from("type"),
                Some(raw_item.type_),
            )),
        }
    }
}

/// An armor item in Orna.
pub struct Armor {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub stats: ItemStats,
    pub element: Option<String>,
    pub materials: Vec<ItemMaterial>,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
    pub prevents: Vec<String>,
    pub causes: Vec<String>,
    pub cures: Vec<String>,
    pub gives: Vec<String>,
}

impl TryFrom<RawItem> for Armor {
    type Error = Error;

    /// Create an `Armor` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Armor`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{ExtraField, InvalidField, MissingField};

        if item.type_ != "Armor" {
            return Err(InvalidField(
                String::from("Armor"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        if item.category.is_some() {
            return Err(ExtraField(String::from("Armor"), String::from("category")));
        }

        let missing_field =
            |field: &'static str| move || MissingField(String::from("Armor"), String::from(field));

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            stats: item.stats.ok_or_else(missing_field("stats"))?,
            element: item.element,
            materials: item.materials.ok_or_else(missing_field("materials"))?,
            dropped_by: item.dropped_by.unwrap_or_else(Vec::new),
            quests: item.quests.unwrap_or_else(Vec::new),
            equipped_by: item.equipped_by.unwrap_or_else(Vec::new),
            prevents: item.prevents.unwrap_or_else(Vec::new),
            causes: item.causes.unwrap_or_else(Vec::new),
            cures: item.cures.unwrap_or_else(Vec::new),
            gives: item.gives.unwrap_or_else(Vec::new),
        })
    }
}

/// An weapon item in Orna.
pub struct Weapon {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub stats: ItemStats,
    pub element: Option<String>,
    pub materials: Vec<ItemMaterial>,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
    pub prevents: Vec<String>,
    pub causes: Vec<String>,
    pub cures: Vec<String>,
    pub gives: Vec<String>,
    pub category: String,
}

impl TryFrom<RawItem> for Weapon {
    type Error = Error;

    /// Create an `Weapon` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Weapon`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Weapon" {
            return Err(InvalidField(
                String::from("Weapon"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        let missing_field =
            |field: &'static str| move || MissingField(String::from("Weapon"), String::from(field));

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            stats: item.stats.ok_or_else(missing_field("stats"))?,
            element: item.element,
            materials: item.materials.ok_or_else(missing_field("materials"))?,
            dropped_by: item.dropped_by.unwrap_or_else(Vec::new),
            quests: item.quests.unwrap_or_else(Vec::new),
            equipped_by: item.equipped_by.unwrap_or_else(Vec::new),
            prevents: item.prevents.unwrap_or_else(Vec::new),
            causes: item.causes.unwrap_or_else(Vec::new),
            cures: item.cures.unwrap_or_else(Vec::new),
            gives: item.gives.unwrap_or_else(Vec::new),
            category: item.category.ok_or_else(missing_field("category"))?,
        })
    }
}
