use crate::{
    codex::{CodexBoss, CodexMonster, CodexRaid},
    data::CodexGenericMonster,
    error::Error,
    guide::Static,
    items::admin::AdminItems,
    monsters::admin::{AdminMonster, AdminMonsters},
    pets::admin::AdminPets,
    skills::admin::AdminSkills,
};

/// Aggregate for guide data.
#[derive(Clone, Default, PartialEq)]
pub struct GuideData {
    /// Items from the guide.
    pub items: AdminItems,
    /// Monsters from the guide (includes bosses and raids).
    pub monsters: AdminMonsters,
    /// Skills from the guied.
    pub skills: AdminSkills,
    /// Pets from the guide.
    pub pets: AdminPets,
    /// Static data from the guide.
    pub static_: Static,
}

impl GuideData {
    /// Find the admin monster associated with the given codex monster.
    /// If there is no match, return an `Err`.
    pub fn find_match_for_codex_generic_monster<'a>(
        &'a self,
        needle: CodexGenericMonster,
    ) -> Result<&'a AdminMonster, Error> {
        match needle {
            CodexGenericMonster::Monster(monster) => {
                self.find_match_for_codex_regular_monster(monster)
            }
            CodexGenericMonster::Boss(boss) => self.find_match_for_codex_boss(boss),
            CodexGenericMonster::Raid(raid) => self.find_match_for_codex_raid(raid),
        }
    }

    /// Find the admin monster associated with the given codex monster.
    /// If there is no match, return an `Err`.
    pub fn find_match_for_codex_regular_monster<'a>(
        &'a self,
        needle: &CodexMonster,
    ) -> Result<&'a AdminMonster, Error> {
        self.monsters
            .monsters
            .iter()
            .find(|admin| {
                !admin.codex_uri.is_empty()
                    && admin.is_regular_monster()
                    && admin.codex_uri["/codex/monsters/".len()..].trim_end_matches('/')
                        == needle.slug
            })
            .ok_or_else(|| {
                Error::Misc(format!(
                    "No match for codex regular monster '{}'",
                    needle.slug
                ))
            })
    }

    /// Find the admin monster associated with the given codex boss.
    /// If there is no match, return an `Err`.
    pub fn find_match_for_codex_boss<'a>(
        &'a self,
        needle: &CodexBoss,
    ) -> Result<&'a AdminMonster, Error> {
        self.monsters
            .monsters
            .iter()
            .find(|admin| {
                !admin.codex_uri.is_empty()
                    && admin.is_boss(&self.static_.spawns)
                    && admin.codex_uri["/codex/bosses/".len()..].trim_end_matches('/')
                        == needle.slug
            })
            .ok_or_else(|| Error::Misc(format!("No match for codex boss '{}'", needle.slug)))
    }

    /// Find the admin monster associated with the given codex raid.
    /// If there is no match, return an `Err`.
    pub fn find_match_for_codex_raid<'a>(
        &'a self,
        needle: &CodexRaid,
    ) -> Result<&'a AdminMonster, Error> {
        self.monsters
            .monsters
            .iter()
            .find(|admin| {
                !admin.codex_uri.is_empty()
                    && admin.is_raid(&self.static_.spawns)
                    && admin.codex_uri["/codex/raids/".len()..].trim_end_matches('/') == needle.slug
            })
            .ok_or_else(|| Error::Misc(format!("No match for codex raid '{}'", needle.slug)))
    }
}
