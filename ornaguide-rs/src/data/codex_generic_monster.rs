use crate::{
    codex::{CodexBoss, CodexMonster, CodexRaid, MonsterAbility, MonsterDrop},
    data::GuideData,
    error::{Error},
    guide::html_utils::Tag,
    monsters::admin::AdminMonster,
};

use itertools::Itertools;

/// A monster from the codex, which may be a regular monster, a boss or a raid.
#[derive(Debug, Clone, Copy)]
pub enum CodexGenericMonster<'a> {
    /// A regular monster.
    Monster(&'a CodexMonster),
    /// A boss.
    Boss(&'a CodexBoss),
    /// A raid.
    Raid(&'a CodexRaid),
}

#[allow(dead_code)]
impl<'a> CodexGenericMonster<'a> {
    // Return the URI of the monster.
    // URI matches `/codex/{kind}/{slug}/`.
    pub fn uri(&self) -> String {
        match self {
            CodexGenericMonster::Monster(x) => format!("/codex/monsters/{}/", x.slug),
            CodexGenericMonster::Boss(x) => format!("/codex/bosses/{}/", x.slug),
            CodexGenericMonster::Raid(x) => format!("/codex/raids/{}/", x.slug),
        }
    }

    // Return the slug of the monster.
    pub fn slug(&self) -> &str {
        match self {
            CodexGenericMonster::Monster(x) => &x.slug,
            CodexGenericMonster::Boss(x) => &x.slug,
            CodexGenericMonster::Raid(x) => &x.slug,
        }
    }

    /// Return the name of the monster.
    pub fn name(&self) -> &'a String {
        match self {
            CodexGenericMonster::Monster(x) => &x.name,
            CodexGenericMonster::Boss(x) => &x.name,
            CodexGenericMonster::Raid(x) => &x.name,
        }
    }

    /// Return the icon of the monster.
    pub fn icon(&self) -> &'a String {
        match self {
            CodexGenericMonster::Monster(x) => &x.icon,
            CodexGenericMonster::Boss(x) => &x.icon,
            CodexGenericMonster::Raid(x) => &x.icon,
        }
    }

    /// Return the events to which the monster belongs.
    pub fn events(&self) -> &'a Vec<String> {
        match self {
            CodexGenericMonster::Monster(x) => x.events.as_ref(),
            CodexGenericMonster::Boss(x) => x.events.as_ref(),
            CodexGenericMonster::Raid(x) => x.events.as_ref(),
        }
    }

    /// Return the family of the monster, if any.
    pub fn family(&self) -> Option<&'a String> {
        match self {
            CodexGenericMonster::Monster(x) => Some(&x.family),
            CodexGenericMonster::Boss(x) => Some(&x.family),
            CodexGenericMonster::Raid(_) => None,
        }
    }

    /// Return the rarity of the monster, if any.
    pub fn rarity(&self) -> Option<&'a String> {
        match self {
            CodexGenericMonster::Monster(x) => Some(&x.rarity),
            CodexGenericMonster::Boss(x) => Some(&x.rarity),
            CodexGenericMonster::Raid(_) => None,
        }
    }

    /// Return the tier of the monster.
    pub fn tier(&self) -> u8 {
        match self {
            CodexGenericMonster::Monster(x) => x.tier,
            CodexGenericMonster::Boss(x) => x.tier,
            CodexGenericMonster::Raid(x) => x.tier,
        }
    }

    /// Return the tags attached to the monster.
    pub fn tags(&self) -> &'a Vec<Tag> {
        static EMPTY_VEC: Vec<Tag> = Vec::new();
        match self {
            CodexGenericMonster::Monster(_) => &EMPTY_VEC,
            CodexGenericMonster::Boss(_) => &EMPTY_VEC,
            CodexGenericMonster::Raid(x) => &x.tags,
        }
    }

    /// Return the tags attached to the monster as guide spawns
    pub fn tags_as_guide_spawns(&self) -> Vec<&'static str> {
        static WRB_STR: &str = "World Raid";
        static KRB_STR: &str = "Kingdom Raid";
        self.tags()
            .iter()
            .filter_map(|tag| match tag {
                Tag::WorldRaid => Some(WRB_STR),
                Tag::KingdomRaid => Some(KRB_STR),
                // TODO(ethiraric, 27/07/2022): Include Other Realm Raid as a spawn?
                Tag::OtherRealmsRaid => None,
                _ => panic!("Unknown tag {:?} for monster", tag),
            })
            .sorted()
            .collect()
    }

    /// Return the abilities of the monster.
    pub fn abilities(&self) -> &'a Vec<MonsterAbility> {
        match self {
            CodexGenericMonster::Monster(x) => &x.abilities,
            CodexGenericMonster::Boss(x) => &x.abilities,
            CodexGenericMonster::Raid(x) => &x.abilities,
        }
    }

    /// Return the list of drops of the monster.
    pub fn drops(&self) -> &'a Vec<MonsterDrop> {
        match self {
            CodexGenericMonster::Monster(x) => &x.drops,
            CodexGenericMonster::Boss(x) => &x.drops,
            CodexGenericMonster::Raid(x) => &x.drops,
        }
    }

    /// Try to convert `self` to an `AdminMonster`.
    ///
    ///  - An unknown family will be ignored, rather than returning an error.
    ///  - Unknown events are ignored, rather than returning an error.
    ///  - Unknown spawns are ignored, rather than returning an error.
    ///  - Unknown drops are ignored, rather than returning an error.
    ///  - Unknown skills are ignored, rather than returning an error.
    pub fn try_to_admin_monster(&self, guide_data: &GuideData) -> Result<AdminMonster, Error> {
        match self {
            CodexGenericMonster::Monster(x) => x.try_to_admin_monster(guide_data),
            CodexGenericMonster::Boss(x) => x.try_to_admin_monster(guide_data),
            CodexGenericMonster::Raid(x) => x.try_to_admin_monster(guide_data),
        }
    }
}
