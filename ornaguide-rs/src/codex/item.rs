use crate::{
    data::GuideData,
    error::Error,
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

/// Stats of an item.
#[derive(Debug, Default, Serialize, Deserialize)]
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
    pub foresight: Option<i8>,
    /// The number of adorn slots at level 10, common quality.
    pub adornment_slots: Option<u8>,
    /// The elment of the item.
    pub element: Option<Element>,
}

/// The ability the item has in off-hand.
#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    /// The name of the ability.
    pub name: String,
    /// The description of the ability.
    pub description: String,
}

/// A monster dropping an item.
#[derive(Debug, Serialize, Deserialize)]
pub struct DroppedBy {
    /// The name of the monster.
    pub name: String,
    /// The uri to the monster.
    pub uri: String,
    /// The icon of the monster.
    pub icon: String,
}

/// A monster dropping an item.
#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeMaterial {
    /// The name of the material.
    pub name: String,
    /// The uri to the material.
    pub uri: String,
    /// The icon of the material.
    pub icon: String,
}

/// A debuff the item causes.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cause {
    /// The name of the debuff.
    pub name: String,
    /// The icon of the debuff.
    pub icon: String,
}

/// A buff the item gives.
#[derive(Debug, Serialize, Deserialize)]
pub struct Give {
    /// The name of the buff.
    pub name: String,
    /// The chance (0-100) of the effect happening.
    pub chance: i8,
    /// The icon of the buff.
    pub icon: String,
}

/// A debuff the item cures.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cure {
    /// The name of the buff.
    pub name: String,
    /// The icon of the buff.
    pub icon: String,
}

/// An debuff the item prevents.
#[derive(Debug, Serialize, Deserialize)]
pub struct Immunity {
    /// The name of the debuff.
    pub name: String,
    /// The icon of the debuff.
    pub icon: String,
}

/// An item on the codex.
#[derive(Debug, Serialize, Deserialize)]
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
    pub fn try_to_admin_item(&self, guide_data: &GuideData) -> Result<AdminItem, Error> {
        Ok(AdminItem {
            codex_uri: format!("/codex/items/{}/", self.slug),
            name: self.name.clone(),
            tier: self.tier,
            image_name: self.icon.clone(),
            description: if !self.description.is_empty() {
                self.description.clone()
            } else {
                ".".to_string()
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
                .ignore_failed_id_conversions()?,
            cures: self
                .cures
                .try_to_guide_ids(&guide_data.static_)
                .ignore_failed_id_conversions()?,
            gives: self
                .gives
                .try_to_guide_ids(&guide_data.static_)
                .ignore_failed_id_conversions()?,
            prevents: self
                .immunities
                .try_to_guide_ids(&guide_data.static_)
                .ignore_failed_id_conversions()?,
            materials: self
                .upgrade_materials
                .iter()
                .filter_map(|item| guide_data.items.find_by_uri(&item.uri).map(|item| item.id))
                .collect(),
            ..AdminItem::default()
        })
    }
}

impl ToString for Element {
    fn to_string(&self) -> String {
        match self {
            Element::Fire => "Fire".to_string(),
            Element::Water => "Water".to_string(),
            Element::Earthen => "Earthen".to_string(),
            Element::Lightning => "Lightning".to_string(),
            Element::Holy => "Holy".to_string(),
            Element::Dark => "Dark".to_string(),
            Element::Arcane => "Arcane".to_string(),
            Element::Dragon => "Dragon".to_string(),
            Element::Physical => "Physical".to_string(),
        }
    }
}

/// A trait to extend `Vec`s of `Cure`s, `Give`s, ....
pub trait ItemStatusEffects {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide status_effect ids.
    /// Returns `Error::PartialCodexStatusEffectConversion` if all fields have not been
    /// successfully converted.
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
                    Err(Error::PartialCodexStatusEffectsConversion(
                        successes, failures,
                    ))
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
#[derive(Serialize, Deserialize)]
pub struct Items {
    /// Items from the codex.
    pub items: Vec<Item>,
}

impl<'a> Items {
    /// Find the codex item associated with the given uri.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a Item> {
        static URI_START: &str = "/codex/items/";
        if !needle.starts_with(URI_START) {
            return None;
        }

        let slug = &needle[URI_START.len()..needle.len() - 1];
        self.items.iter().find(|item| item.slug == slug)
    }

    /// Find the codex item associated with the given uri.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a Item, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex item with uri '{}'", needle)))
    }
}
