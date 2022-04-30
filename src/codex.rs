use crate::error::Error;

pub(crate) mod html_list_parser;
pub(crate) mod html_skill_parser;

pub use html_skill_parser::{CodexSkill, StatusEffect};

#[derive(Debug)]
pub struct Skill {
    pub name: String,
    pub tier: u32,
    pub uri: String,
}

/// The public codex on `playorna.com`.
pub trait Codex {
    /// Retrieve the list of skills from the orna codex.
    fn codex_fetch_skill_list(&self) -> Result<Vec<Skill>, Error>;
    /// Retrieve the details about a skill from the orna codex.
    fn codex_fetch_skill(&self, skill_name: &str) -> Result<CodexSkill, Error>;
}
