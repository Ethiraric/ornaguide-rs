use crate::error::Error;

pub use crate::items::raw::{
    ItemDroppedBy, ItemEquippedBy, ItemMaterial, ItemQuest, ItemStats, RawItem,
};

/// An item in Orna. This enum splits into types the different items.
#[derive(Clone)]
pub enum Item {
    /// An armor item.
    Armor(ArmorItem),
    /// A weapon item.
    Weapon(WeaponItem),
    /// A legs item.
    Legs(LegsItem),
    /// A head item.
    Head(HeadItem),
    /// A material item.
    Material(MaterialItem),
    /// An accessory item.
    Accessory(AccessoryItem),
    /// An off-hand item.
    OffHand(OffHandItem),
    /// An "item" item.
    Item(ItemItem),
    /// An adornment item.
    Adornment(AdornmentItem),
    /// A curative item.
    Curative(CurativeItem),
    /// A fish item.
    Fish(FishItem),
    /// An other item.
    Other(OtherItem),
}

impl Item {
    pub fn get_id(&self) -> u32 {
        match self {
            Item::Armor(x) => x.id,
            Item::Weapon(x) => x.id,
            Item::Legs(x) => x.id,
            Item::Head(x) => x.id,
            Item::Material(x) => x.id,
            Item::Accessory(x) => x.id,
            Item::OffHand(x) => x.id,
            Item::Item(x) => x.id,
            Item::Adornment(x) => x.id,
            Item::Curative(x) => x.id,
            Item::Fish(x) => x.id,
            Item::Other(x) => x.id,
        }
    }
}

impl TryFrom<RawItem> for Item {
    type Error = Error;

    fn try_from(raw_item: RawItem) -> Result<Self, Self::Error> {
        match raw_item.type_.as_str() {
            "Armor" => Ok(Self::Armor(ArmorItem::try_from(raw_item)?)),
            "Weapon" => Ok(Self::Weapon(WeaponItem::try_from(raw_item)?)),
            "Legs" => Ok(Self::Legs(LegsItem::try_from(raw_item)?)),
            "Head" => Ok(Self::Head(HeadItem::try_from(raw_item)?)),
            "Material" => Ok(Self::Material(MaterialItem::try_from(raw_item)?)),
            "Accessory" => Ok(Self::Accessory(AccessoryItem::try_from(raw_item)?)),
            "Off-hand" => Ok(Self::OffHand(OffHandItem::try_from(raw_item)?)),
            "Item" => Ok(Self::Item(ItemItem::try_from(raw_item)?)),
            "Adornment" => Ok(Self::Adornment(AdornmentItem::try_from(raw_item)?)),
            "Curative" => Ok(Self::Curative(CurativeItem::try_from(raw_item)?)),
            "Fish" => Ok(Self::Fish(FishItem::try_from(raw_item)?)),
            "Other" => Ok(Self::Other(OtherItem::try_from(raw_item)?)),
            _ => Err(Error::InvalidField(
                "Item".to_string(),
                "type".to_string(),
                Some(raw_item.type_),
            )),
        }
    }
}

/// An armor item in Orna.
#[derive(Clone)]
pub struct ArmorItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
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

impl TryFrom<RawItem> for ArmorItem {
    type Error = Error;

    /// Create an `Armor` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Armor`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{ExtraField, InvalidField, MissingField};

        if item.type_ != "Armor" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        if item.category.is_some() {
            return Err(ExtraField(item.name, "category".to_string()));
        }

        let missing_field =
            |field: &'static str| || MissingField(item.name.clone(), field.to_string());

        Ok(Self {
            name: item.name.clone(),
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            stats: item.stats.ok_or_else(missing_field("stats"))?,
            element: item.element,
            materials: item.materials.ok_or_else(missing_field("materials"))?,
            dropped_by: item.dropped_by.unwrap_or_default(),
            quests: item.quests.unwrap_or_default(),
            equipped_by: item.equipped_by.unwrap_or_default(),
            prevents: item.prevents.unwrap_or_default(),
            causes: item.causes.unwrap_or_default(),
            cures: item.cures.unwrap_or_default(),
            gives: item.gives.unwrap_or_default(),
        })
    }
}

/// An weapon item in Orna.
#[derive(Clone)]
pub struct WeaponItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub view_distance: Option<u32>,
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

impl TryFrom<RawItem> for WeaponItem {
    type Error = Error;

    /// Create an `Weapon` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Weapon`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Weapon" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(item.name.clone(), field.to_string());

        Ok(Self {
            name: item.name.clone(),
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            view_distance: item.view_distance,
            stats: item.stats.ok_or_else(missing_field("stats"))?,
            element: item.element,
            materials: item.materials.ok_or_else(missing_field("materials"))?,
            dropped_by: item.dropped_by.unwrap_or_default(),
            quests: item.quests.unwrap_or_default(),
            equipped_by: item.equipped_by.unwrap_or_default(),
            prevents: item.prevents.unwrap_or_default(),
            causes: item.causes.unwrap_or_default(),
            cures: item.cures.unwrap_or_default(),
            gives: item.gives.unwrap_or_default(),
            category: item.category.ok_or_else(missing_field("category"))?,
        })
    }
}

/// An legs item in Orna.
#[derive(Clone)]
pub struct LegsItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
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

impl TryFrom<RawItem> for LegsItem {
    type Error = Error;

    /// Create an `Legs` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Legs`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Legs" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(item.name.clone(), field.to_string());

        Ok(Self {
            name: item.name.clone(),
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            stats: item.stats.ok_or_else(missing_field("stats"))?,
            element: item.element,
            materials: item.materials.ok_or_else(missing_field("materials"))?,
            dropped_by: item.dropped_by.unwrap_or_default(),
            quests: item.quests.unwrap_or_default(),
            equipped_by: item.equipped_by.unwrap_or_default(),
            prevents: item.prevents.unwrap_or_default(),
            causes: item.causes.unwrap_or_default(),
            cures: item.cures.unwrap_or_default(),
            gives: item.gives.unwrap_or_default(),
        })
    }
}

/// A head item in Orna.
#[derive(Clone)]
pub struct HeadItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
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

impl TryFrom<RawItem> for HeadItem {
    type Error = Error;

    /// Create an `Head` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Head`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Head" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(item.name.clone(), field.to_string());

        Ok(Self {
            name: item.name.clone(),
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            stats: item.stats.ok_or_else(missing_field("stats"))?,
            element: item.element,
            materials: item.materials.ok_or_else(missing_field("materials"))?,
            dropped_by: item.dropped_by.unwrap_or_default(),
            quests: item.quests.unwrap_or_default(),
            equipped_by: item.equipped_by.unwrap_or_default(),
            prevents: item.prevents.unwrap_or_default(),
            causes: item.causes.unwrap_or_default(),
            cures: item.cures.unwrap_or_default(),
            gives: item.gives.unwrap_or_default(),
        })
    }
}

/// A material item in Orna.
#[derive(Clone)]
pub struct MaterialItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub materials: Vec<ItemMaterial>,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
}

impl TryFrom<RawItem> for MaterialItem {
    type Error = Error;

    /// Create an `Material` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Material`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Material" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        // Sanity check that a material is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    item.name,
                    "equipped_by[]".to_string(),
                    Some(
                        v.iter()
                            .map(|equip| equip.name.clone())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                ));
            }
        }

        let missing_field =
            |field: &'static str| || MissingField(item.name.clone(), field.to_string());

        Ok(Self {
            name: item.name.clone(),
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            // Herbalist materials do not have any item to which they are materials for.
            materials: item.materials.unwrap_or_default(),
            dropped_by: item.dropped_by.unwrap_or_default(),
            equipped_by: item.equipped_by.ok_or_else(missing_field("equipped_by"))?,
            quests: item.quests.unwrap_or_default(),
        })
    }
}

/// An accessory item in Orna.
#[derive(Clone)]
pub struct AccessoryItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub view_distance: Option<u32>,
    pub stats: Option<ItemStats>,
    pub element: Option<String>,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
    pub prevents: Vec<String>,
    pub causes: Vec<String>,
    pub cures: Vec<String>,
    pub gives: Vec<String>,
}

impl TryFrom<RawItem> for AccessoryItem {
    type Error = Error;

    /// Create an `Accessory` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Accessory`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if item.type_ != "Accessory" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        // Sanity check that an accessory is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.is_empty() {
                return Err(InvalidField(
                    item.name,
                    "equipped_by[]".to_string(),
                    Some("[]".to_string()),
                ));
            }
        }

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            view_distance: item.view_distance,
            stats: item.stats,
            element: item.element,
            dropped_by: item.dropped_by.unwrap_or_default(),
            quests: item.quests.unwrap_or_default(),
            equipped_by: item.equipped_by.unwrap_or_default(),
            prevents: item.prevents.unwrap_or_default(),
            causes: item.causes.unwrap_or_default(),
            cures: item.cures.unwrap_or_default(),
            gives: item.gives.unwrap_or_default(),
        })
    }
}

/// An off-hand item in Orna.
#[derive(Clone)]
pub struct OffHandItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub view_distance: Option<u32>,
    // Only Bag of Tricks has no stats... It's the only item responsible for that `Option`.
    pub stats: Option<ItemStats>,
    pub element: Option<String>,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
    pub prevents: Vec<String>,
    pub causes: Vec<String>,
    pub cures: Vec<String>,
    pub gives: Vec<String>,
}

impl TryFrom<RawItem> for OffHandItem {
    type Error = Error;

    /// Create an `OffHand` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `OffHand`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if item.type_ != "Off-hand" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            view_distance: item.view_distance,
            stats: item.stats,
            element: item.element,
            dropped_by: item.dropped_by.unwrap_or_default(),
            quests: item.quests.unwrap_or_default(),
            equipped_by: item.equipped_by.unwrap_or_default(),
            prevents: item.prevents.unwrap_or_default(),
            causes: item.causes.unwrap_or_default(),
            cures: item.cures.unwrap_or_default(),
            gives: item.gives.unwrap_or_default(),
        })
    }
}

/// An "item" item in Orna.
#[derive(Clone)]
pub struct ItemItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
}

impl TryFrom<RawItem> for ItemItem {
    type Error = Error;

    /// Create an `Item` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Item`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Item" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        // Sanity check that an item is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    item.name,
                    "equipped_by[]".to_string(),
                    Some(
                        v.iter()
                            .map(|equip| equip.name.clone())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                ));
            }
        }

        let missing_field =
            |field: &'static str| || MissingField(item.name.clone(), field.to_string());

        Ok(Self {
            name: item.name.clone(),
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            dropped_by: item.dropped_by.unwrap_or_default(),
            equipped_by: item.equipped_by.ok_or_else(missing_field("equipped_by"))?,
            quests: item.quests.unwrap_or_default(),
        })
    }
}

/// An adornment item in Orna.
#[derive(Clone)]
pub struct AdornmentItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub view_distance: Option<u32>,
    pub stats: Option<ItemStats>,
    pub element: Option<String>,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
    pub prevents: Vec<String>,
    pub causes: Vec<String>,
    pub cures: Vec<String>,
    pub gives: Vec<String>,
}

impl TryFrom<RawItem> for AdornmentItem {
    type Error = Error;

    /// Create an `Adornment` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Adornment`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if item.type_ != "Adornment" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            view_distance: item.view_distance,
            stats: item.stats,
            element: item.element,
            dropped_by: item.dropped_by.unwrap_or_default(),
            quests: item.quests.unwrap_or_default(),
            equipped_by: item.equipped_by.unwrap_or_default(),
            prevents: item.prevents.unwrap_or_default(),
            causes: item.causes.unwrap_or_default(),
            cures: item.cures.unwrap_or_default(),
            gives: item.gives.unwrap_or_default(),
        })
    }
}

/// A curative item in Orna.
#[derive(Clone)]
pub struct CurativeItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
}

impl TryFrom<RawItem> for CurativeItem {
    type Error = Error;

    /// Create a `Curative` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Curative`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Curative" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        // Sanity check that a curable is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    item.name,
                    "equipped_by[]".to_string(),
                    Some(
                        v.iter()
                            .map(|equip| equip.name.clone())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                ));
            }
        }

        let missing_field =
            |field: &'static str| || MissingField(item.name.clone(), field.to_string());

        Ok(Self {
            name: item.name.clone(),
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            dropped_by: item.dropped_by.unwrap_or_default(),
            equipped_by: item.equipped_by.ok_or_else(missing_field("equipped_by"))?,
            quests: item.quests.unwrap_or_default(),
        })
    }
}

/// A fish item in Orna.
#[derive(Clone)]
pub struct FishItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
}

impl TryFrom<RawItem> for FishItem {
    type Error = Error;

    /// Create a `Fish` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Fish`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if item.type_ != "Fish" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            dropped_by: item.dropped_by.unwrap_or_default(),
            quests: item.quests.unwrap_or_default(),
        })
    }
}

/// An other item in Orna.
#[derive(Clone)]
pub struct OtherItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u8,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub dropped_by: Vec<ItemDroppedBy>,
    pub quests: Vec<ItemQuest>,
    pub equipped_by: Vec<ItemEquippedBy>,
}

impl TryFrom<RawItem> for OtherItem {
    type Error = Error;

    /// Create an `Other` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Other`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Other" {
            return Err(InvalidField(
                item.name,
                "type".to_string(),
                Some(item.type_),
            ));
        }

        // Sanity check that a curable is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    item.name,
                    "equipped_by[]".to_string(),
                    Some(
                        v.iter()
                            .map(|equip| equip.name.clone())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                ));
            }
        }

        let missing_field =
            |field: &'static str| || MissingField(item.name.clone(), field.to_string());

        Ok(Self {
            name: item.name.clone(),
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            dropped_by: item.dropped_by.unwrap_or_default(),
            equipped_by: item.equipped_by.ok_or_else(missing_field("equipped_by"))?,
            quests: item.quests.unwrap_or_default(),
        })
    }
}
