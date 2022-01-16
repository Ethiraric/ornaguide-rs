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
    /// An other item.
    Other(OtherItem),
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
            "Other" => Ok(Self::Other(OtherItem::try_from(raw_item)?)),
            _ => Err(Error::InvalidField(
                String::from("Item"),
                String::from("type"),
                Some(raw_item.type_),
            )),
        }
    }
}

/// An armor item in Orna.
pub struct ArmorItem {
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

impl TryFrom<RawItem> for ArmorItem {
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
pub struct WeaponItem {
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

impl TryFrom<RawItem> for WeaponItem {
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

/// An legs item in Orna.
pub struct LegsItem {
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

impl TryFrom<RawItem> for LegsItem {
    type Error = Error;

    /// Create an `Legs` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Legs`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Legs" {
            return Err(InvalidField(
                String::from("Legs"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        let missing_field =
            |field: &'static str| move || MissingField(String::from("Legs"), String::from(field));

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

/// A head item in Orna.
pub struct HeadItem {
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

impl TryFrom<RawItem> for HeadItem {
    type Error = Error;

    /// Create an `Head` from a `RawItem`.
    /// The `RawItem`'s `type` field must be `Head`.
    fn try_from(item: RawItem) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if item.type_ != "Head" {
            return Err(InvalidField(
                String::from("Head"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        let missing_field =
            |field: &'static str| move || MissingField(String::from("Head"), String::from(field));

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

/// A material item in Orna.
pub struct MaterialItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
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
                String::from("Material"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        // Sanity check that a material is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    String::from("Material"),
                    String::from("equipped_by[]"),
                    Some(
                        v.iter()
                            .map(|equip| equip.name.clone())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                ));
            }
        }

        let missing_field = |field: &'static str| {
            move || MissingField(String::from("Material"), String::from(field))
        };

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            materials: item.materials.ok_or_else(missing_field("materials"))?,
            dropped_by: item.dropped_by.unwrap_or_else(Vec::new),
            equipped_by: item.equipped_by.ok_or_else(missing_field("equipped_by"))?,
            quests: item.quests.unwrap_or_else(Vec::new),
        })
    }
}

/// An accessory item in Orna.
pub struct AccessoryItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub stats: ItemStats,
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
        use Error::{InvalidField, MissingField};

        if item.type_ != "Accessory" {
            return Err(InvalidField(
                String::from("Accessory"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        // Sanity check that an accessory is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    String::from("Accessory"),
                    String::from("equipped_by[]"),
                    Some(
                        v.iter()
                            .map(|equip| equip.name.clone())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                ));
            }
        }

        let missing_field = |field: &'static str| {
            move || MissingField(String::from("Accessory"), String::from(field))
        };

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

/// An off-hand item in Orna.
pub struct OffHandItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
    pub stats: ItemStats,
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
        use Error::{InvalidField, MissingField};

        if item.type_ != "Off-hand" {
            return Err(InvalidField(
                String::from("OffHand"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        let missing_field = |field: &'static str| {
            move || MissingField(String::from("OffHand"), String::from(field))
        };

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

/// An "item" item in Orna.
pub struct ItemItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
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
                String::from("Item"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        // Sanity check that an item is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    String::from("Item"),
                    String::from("equipped_by[]"),
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
            |field: &'static str| move || MissingField(String::from("Item"), String::from(field));

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            dropped_by: item.dropped_by.unwrap_or_else(Vec::new),
            equipped_by: item.equipped_by.ok_or_else(missing_field("equipped_by"))?,
            quests: item.quests.unwrap_or_else(Vec::new),
        })
    }
}

/// An adornment item in Orna.
pub struct AdornmentItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
    pub boss: bool,
    pub arena: bool,
    pub image: String,
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
                String::from("Adornment"),
                String::from("type"),
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
            stats: item.stats,
            element: item.element,
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

/// A curative item in Orna.
pub struct CurativeItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
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
                String::from("Curative"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        // Sanity check that a curable is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    String::from("Curative"),
                    String::from("equipped_by[]"),
                    Some(
                        v.iter()
                            .map(|equip| equip.name.clone())
                            .collect::<Vec<_>>()
                            .join(","),
                    ),
                ));
            }
        }

        let missing_field = |field: &'static str| {
            move || MissingField(String::from("Curative"), String::from(field))
        };

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            dropped_by: item.dropped_by.unwrap_or_else(Vec::new),
            equipped_by: item.equipped_by.ok_or_else(missing_field("equipped_by"))?,
            quests: item.quests.unwrap_or_else(Vec::new),
        })
    }
}

/// An other item in Orna.
pub struct OtherItem {
    pub name: String,
    pub id: u32,
    pub description: String,
    pub tier: u32,
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
                String::from("Other"),
                String::from("type"),
                Some(item.type_),
            ));
        }

        // Sanity check that a curable is usable by all 3 classes.
        if let Some(v) = &item.equipped_by {
            if v.len() != 3 {
                return Err(InvalidField(
                    String::from("Other"),
                    String::from("equipped_by[]"),
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
            |field: &'static str| move || MissingField(String::from("Other"), String::from(field));

        Ok(Self {
            name: item.name,
            id: item.id,
            description: item.description,
            tier: item.tier,
            boss: item.boss,
            arena: item.arena,
            image: item.image,
            dropped_by: item.dropped_by.unwrap_or_else(Vec::new),
            equipped_by: item.equipped_by.ok_or_else(missing_field("equipped_by"))?,
            quests: item.quests.unwrap_or_else(Vec::new),
        })
    }
}
