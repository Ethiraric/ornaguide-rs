use std::str::FromStr;

use crate::{
    codex::affix::Affix,
    data::GuideData,
    error::{Error, Kind},
    guide::{html_utils::Tag, Static, VecElements},
    items::admin::AdminItem,
    misc::{
        codex_effect_name_iter_to_guide_id_results, codex_effect_name_to_guide_name,
        VecIdConversionResult,
    },
};
use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// An element (fire, water, arcane, ...).
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Element {
    Fire,
    Water,
    Earthen,
    Lightning,
    Holy,
    Dark,
    Arcane,
    Dragon,
    Physical,
}

/// An equipment slot in which the item can be equipped.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Place {
    Head,
    Weapon,
    Torso,
    OffHand,
    Legs,
    Accessory,
    Armor,
    Material,
    Augment,
}

/// Stats of an item.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(default)]
pub struct Stats {
    /// The base attack stat of the item.
    pub attack: Option<i16>,
    /// The base magic stat of the item.
    pub magic: Option<i16>,
    /// The base HP stat of the item.
    pub hp: Option<i16>,
    /// The base MP stat of the item.
    pub mana: Option<i16>,
    /// The base defense stat of the item.
    pub defense: Option<i16>,
    /// The base resistance stat of the item.
    pub resistance: Option<i16>,
    /// The base ward stat of the item (%).
    pub ward: Option<i8>,
    /// The base dexterity stat of the item.
    pub dexterity: Option<i16>,
    /// The crit stat of the item.
    pub crit: Option<u8>,
    /// The foresight of the item.
    pub foresight: Option<i16>,
    /// The number of adorn slots at level 10, common quality.
    pub adornment_slots: Option<u8>,
    /// The elment of the item.
    pub element: Option<Element>,
    /// Whether the item is `two_handed`.
    /// This will be set to false for all items to which this does not apply. The online codex has
    /// no mention along the lines of "not two-handed".
    pub two_handed: bool,
    /// Equipment slot on which the item is equipped.
    pub place: Option<Place>,
    /// The skills the item grants, either to oneself or the followers/summons.
    pub skills_granted: Vec<String>,
    /// The [affixes] the item grants.
    ///
    /// [affixes]: Affix
    pub affixes: Vec<Affix>,
}

/// The ability the item has in off-hand.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Ability {
    /// The name of the ability.
    pub name: String,
    /// The description of the ability.
    pub description: String,
}

/// A monster dropping an item.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct DroppedBy {
    /// The name of the monster.
    pub name: String,
    /// The uri to the monster.
    pub uri: String,
    /// The icon of the monster.
    pub icon: String,
}

/// A monster dropping an item.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct UpgradeMaterial {
    /// The name of the material.
    pub name: String,
    /// The uri to the material.
    pub uri: String,
    /// The icon of the material.
    pub icon: String,
}

/// A debuff the item causes.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Cause {
    /// The name of the debuff.
    pub name: String,
    /// The icon of the debuff.
    pub icon: String,
}

/// A buff the item gives.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Give {
    /// The name of the buff.
    pub name: String,
    /// The chance (0-100) of the effect happening.
    pub chance: i8,
    /// The icon of the buff.
    pub icon: String,
}

/// A debuff the item cures.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Cure {
    /// The name of the buff.
    pub name: String,
    /// The icon of the buff.
    pub icon: String,
}

/// An debuff the item prevents.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Immunity {
    /// The name of the debuff.
    pub name: String,
    /// The icon of the debuff.
    pub icon: String,
}

/// An item on the codex.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Default)]
pub struct Item {
    /// The slug of the item (`https://playorna.com/codex/items/{slug}`).
    pub slug: String,
    /// The name of the item.
    pub name: String,
    /// The icon of the item.
    pub icon: String,
    /// The description of the item.
    pub description: String,
    /// The tier of the item.
    pub tier: u8,
    /// Tags attached to the item.
    pub tags: Vec<Tag>,
    /// The stats of the item.
    pub stats: Option<Stats>,
    /// The ability of the item.
    pub ability: Option<Ability>,
    /// Debuffs the item can cause.
    pub causes: Vec<Cause>,
    /// Debuffs the item cures.
    pub cures: Vec<Cure>,
    /// Buffs the item can give.
    pub gives: Vec<Give>,
    /// Immunities the item grants.
    pub immunities: Vec<Immunity>,
    /// The monsters that drop the item.
    pub dropped_by: Vec<DroppedBy>,
    /// The materials needed to upgrade the item.
    pub upgrade_materials: Vec<UpgradeMaterial>,
}

impl Item {
    /// Return whether the item can be found in shops.
    #[must_use]
    pub fn found_in_shops(&self) -> bool {
        self.tags.iter().any(|tag| *tag == Tag::FoundInShops)
    }

    /// Try to convert `self` to an `AdminItem`.
    ///
    ///  - Unknown status effects are ignored, rather than returning an error.
    ///  - Unknown upgrade materials are ignored, rather than returning an error.
    ///  - An unknown ability will be ignored, rather than returning an error.
    ///  - An unknown element will be ignored, rather than returning an error.
    ///  - `self.dropped_by` is ignored and will not be saved in the returned `AdminItem`.
    ///
    /// # Panics
    /// Panics if an ID conversion failed.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn to_admin_item(&self, guide_data: &GuideData) -> AdminItem {
        AdminItem {
            codex_uri: format!("/codex/items/{}/", self.slug),
            name: self.name.clone(),
            tier: self.tier,
            image_name: self.icon.clone(),
            description: if self.description.is_empty() {
                ".".to_string()
            } else {
                self.description.clone()
            },
            hp: self.stats.as_ref().and_then(|stats| stats.hp).unwrap_or(0),
            mana: self
                .stats
                .as_ref()
                .and_then(|stats| stats.mana)
                .unwrap_or(0),
            attack: self
                .stats
                .as_ref()
                .and_then(|stats| stats.attack)
                .unwrap_or(0),
            magic: self
                .stats
                .as_ref()
                .and_then(|stats| stats.magic)
                .unwrap_or(0),
            defense: self
                .stats
                .as_ref()
                .and_then(|stats| stats.defense)
                .unwrap_or(0),
            resistance: self
                .stats
                .as_ref()
                .and_then(|stats| stats.resistance)
                .unwrap_or(0),
            dexterity: self
                .stats
                .as_ref()
                .and_then(|stats| stats.dexterity)
                .unwrap_or(0),
            ward: self
                .stats
                .as_ref()
                .and_then(|stats| stats.ward)
                .unwrap_or(0),
            crit: self
                .stats
                .as_ref()
                .and_then(|stats| stats.crit)
                .unwrap_or(0),
            foresight: self
                .stats
                .as_ref()
                .and_then(|stats| stats.foresight)
                .unwrap_or(0),
            base_adornment_slots: self
                .stats
                .as_ref()
                .and_then(|stats| stats.adornment_slots)
                .unwrap_or(0),
            has_slots: self
                .stats
                .as_ref()
                .and_then(|stats| stats.adornment_slots)
                .unwrap_or(0)
                > 0,
            element: self
                .stats
                .as_ref()
                .and_then(|stats| stats.element.as_ref())
                .and_then(|elem| {
                    guide_data
                        .static_
                        .elements
                        .get_element_by_name(&elem.to_string())
                        .ok()
                })
                .map(|elem| elem.id),
            ability: self.ability.as_ref().and_then(|ability| {
                guide_data
                    .skills
                    .find_offhand_from_name(&ability.name)
                    .map(|skill| skill.id)
            }),
            causes: self
                .causes
                .try_to_guide_ids(&guide_data.static_)
                .ignore_failed_id_conversions()
                .expect("only possible error should be partial conversions"),
            cures: self
                .cures
                .try_to_guide_ids(&guide_data.static_)
                .ignore_failed_id_conversions()
                .expect("only possible error should be partial conversions"),
            gives: self
                .gives
                .try_to_guide_ids(&guide_data.static_)
                .ignore_failed_id_conversions()
                .expect("only possible error should be partial conversions"),
            prevents: self
                .immunities
                .try_to_guide_ids(&guide_data.static_)
                .ignore_failed_id_conversions()
                .expect("only possible error should be partial conversions"),
            materials: self
                .upgrade_materials
                .iter()
                .filter_map(|item| guide_data.items.find_by_uri(&item.uri).map(|item| item.id))
                .collect(),
            ..AdminItem::default()
        }
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Element::Fire => "Fire",
                Element::Water => "Water",
                Element::Earthen => "Earthen",
                Element::Lightning => "Lightning",
                Element::Holy => "Holy",
                Element::Dark => "Dark",
                Element::Arcane => "Arcane",
                Element::Dragon => "Dragon",
                Element::Physical => "Physical",
            }
        )
    }
}

impl std::fmt::Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Place::Head => "Head",
                Place::Weapon => "Weapon",
                Place::Torso => "Torso",
                Place::OffHand => "Off-hand",
                Place::Legs => "Legs",
                Place::Accessory => "Accessory",
                Place::Armor => "Armor",
                Place::Augment => "Augment",
                // TODO(ethiraric, 26/01/2023): Check if this is a typo.
                Place::Material => "material",
            }
        )
    }
}

impl FromStr for Place {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Head" => Ok(Place::Head),
            "Weapon" => Ok(Place::Weapon),
            "Torso" => Ok(Place::Torso),
            "Off-hand" => Ok(Place::OffHand),
            "Legs" => Ok(Place::Legs),
            "Accessory" => Ok(Place::Accessory),
            "Armor" | "Armor (for adornments)" => Ok(Place::Armor),
            "Augment (for celestial weapons)" => Ok(Place::Augment),
            "material" => Ok(Place::Material),
            _ => {
                Err(Kind::ParseEnumError("Place".to_string(), format!("Invalid value: {s}")).into())
            }
        }
    }
}

/// A trait to extend `Vec`s of `Cure`s, `Give`s, ....
#[allow(clippy::module_name_repetitions)]
pub trait ItemStatusEffects {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide `status_effect` ids.
    /// Returns `ErrorKind::PartialCodexStatusEffectConversion` if all fields have not been
    /// successfully converted.
    ///
    /// # Errors
    /// Errors if the array could not be converted in its entirety. Should the array be partially
    /// converted, partially converted content can be found in the error variant.
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error>;
    /// Convert the list of status effects to a list of effect names, matching those of the guide.
    fn to_guide_names(&self) -> Vec<&str>;
}

macro_rules! make_impl_for_status_effect_struct_vec {
    ($type:ty) => {
        impl ItemStatusEffects for Vec<$type> {
            fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error> {
                let (successes, failures): (Vec<_>, Vec<_>) =
                    codex_effect_name_iter_to_guide_id_results(
                        self.iter().map(|name| name.name.as_str()),
                        static_,
                    )
                    .partition_result();

                if failures.is_empty() {
                    Ok(successes)
                } else {
                    Err(Kind::PartialCodexStatusEffectsConversion(successes, failures).into())
                }
            }

            fn to_guide_names(&self) -> Vec<&str> {
                self.iter()
                    .map(|effect| codex_effect_name_to_guide_name(&effect.name))
                    .sorted()
                    .collect()
            }
        }
    };
}

make_impl_for_status_effect_struct_vec!(Cause);
make_impl_for_status_effect_struct_vec!(Give);
make_impl_for_status_effect_struct_vec!(Cure);
make_impl_for_status_effect_struct_vec!(Immunity);

/// Collection of items from the codex.
#[derive(Serialize, Deserialize, Clone, Default, Eq, PartialEq)]
pub struct Items {
    /// Items from the codex.
    pub items: Vec<Item>,
}

impl<'a> Items {
    /// Find the codex item associated with the given uri.
    #[must_use]
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a Item> {
        static URI_START: &str = "/codex/items/";
        if !needle.starts_with(URI_START) {
            return None;
        }

        let slug = &needle[URI_START.len()..needle.len() - 1];
        self.find_by_slug(slug)
    }

    /// Find the codex item associated with the given uri.
    ///
    /// # Errors
    /// Errors if there is no match.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a Item, Error> {
        self.find_by_uri(needle).ok_or_else(|| {
            Kind::Misc(format!("No match for codex item with uri '{needle}'")).into()
        })
    }

    /// Find the codex item associated with the given slug.
    #[must_use]
    pub fn find_by_slug(&'a self, needle: &str) -> Option<&'a Item> {
        self.items.iter().find(|item| item.slug == needle)
    }

    /// Find the codex item associated with the given slug.
    ///
    /// # Errors
    /// Errors if there is no match.
    pub fn get_by_slug(&'a self, needle: &str) -> Result<&'a Item, Error> {
        self.find_by_slug(needle).ok_or_else(|| {
            Kind::Misc(format!("No match for codex item with slug '{needle}'")).into()
        })
    }
}
