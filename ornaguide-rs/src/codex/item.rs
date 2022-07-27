use crate::{
    error::Error,
    guide::{html_utils::Tag, Static},
    misc::{codex_effect_name_iter_to_guide_id_results, codex_effect_name_to_guide_name},
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
