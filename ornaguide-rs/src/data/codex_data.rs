use crate::{
    codex::{CodexBosses, CodexFollowers, CodexItems, CodexMonsters, CodexRaids, CodexSkills},
    data::CodexGenericMonster,
};

/// Aggregate for codex data.
#[derive(Clone, Default, Eq, PartialEq)]
pub struct CodexData {
    /// Items from the codex.
    pub items: CodexItems,
    /// Raids from the codex.
    pub raids: CodexRaids,
    /// Monsters from the codex.
    pub monsters: CodexMonsters,
    /// Bosses from the codex.
    pub bosses: CodexBosses,
    /// Skills from the codex.
    pub skills: CodexSkills,
    /// Followers from the codex.
    pub followers: CodexFollowers,
}

impl<'a> CodexData {
    /// Find which monster/boss/raid corresponds to the given URI.
    /// The URI must be of the form `/codex/{kind}/{slug}/` or empty.
    pub fn find_generic_monster_from_uri(&'a self, uri: &str) -> Option<CodexGenericMonster<'a>> {
        if uri.is_empty() {
            return None;
        }

        // Strip `/codex/` and trailing slash from the uri.
        let uri = uri[7..].trim_end_matches('/');
        if let Some(pos) = uri.find('/') {
            let kind = &uri[0..pos];
            let slug = &uri[pos + 1..];
            match kind {
                "monsters" => self
                    .monsters
                    .monsters
                    .iter()
                    .find(|monster| monster.slug == slug)
                    .map(CodexGenericMonster::Monster),
                "bosses" => self
                    .bosses
                    .bosses
                    .iter()
                    .find(|boss| boss.slug == slug)
                    .map(CodexGenericMonster::Boss),
                "raids" => self
                    .raids
                    .raids
                    .iter()
                    .find(|raid| raid.slug == slug)
                    .map(CodexGenericMonster::Raid),
                _ => {
                    println!("Unknown kind for generic monster {uri}");
                    None
                }
            }
        } else {
            println!("Failed to find generic monster for {uri}");
            None
        }
    }

    /// Return an iterator over all monsters / bosses / raids, wrapped in the
    /// `CodexGenericMonster` enum.
    pub fn iter_all_monsters(&'a self) -> impl Iterator<Item = CodexGenericMonster<'a>> {
        self.monsters
            .monsters
            .iter()
            // List monsters, wrap them in a generic type.
            .map(CodexGenericMonster::Monster)
            // List bosses, wrap them in the same generic type and chain the iterators.
            .chain(self.bosses.bosses.iter().map(CodexGenericMonster::Boss))
            // List raids, wrap them in the same generic type and chain the iterators.
            .chain(self.raids.raids.iter().map(CodexGenericMonster::Raid))
    }
}
