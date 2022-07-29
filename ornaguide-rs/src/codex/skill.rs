use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    codex::Tag,
    error::Error,
    guide::Static,
    misc::{
        codex_effect_name_iter_to_guide_id_results, codex_effect_name_to_guide_name,
        VecIdConversionResult,
    },
    skills::admin::AdminSkill,
};

/// A status effect caused or given by a skill.
#[derive(Debug, Serialize, Deserialize)]
pub struct SkillStatusEffect {
    /// The name of the effect.
    pub effect: String,
    /// The chance (0-100) of the effect happening.
    pub chance: i8,
}

/// A trait to extend `Vec<SkillStatusEffect>` specifically.
pub trait SkillStatusEffects {
    /// Try to convert `self` to a `Vec<u32>`, with `u32`s being the guide status_effect ids.
    /// Returns `Error::PartialCodexStatusEffectConversion` if all fields have not been
    /// successfully converted.
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error>;
    /// Convert the list of status effects to a list of effect names, matching those of the guide.
    fn to_guide_names(&self) -> Vec<String>;
}

impl SkillStatusEffects for Vec<SkillStatusEffect> {
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error> {
        let (successes, failures): (Vec<_>, Vec<_>) = codex_effect_name_iter_to_guide_id_results(
            self.iter().map(|name| name.effect.as_str()),
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

    fn to_guide_names(&self) -> Vec<String> {
        self.iter()
            .map(|cause| codex_effect_name_to_guide_name(&cause.effect).to_string())
            .sorted()
            .collect()
    }
}

/// A skill on the codex.
#[derive(Debug, Serialize, Deserialize)]
pub struct CodexSkill {
    /// The name of the skill.
    pub name: String,
    /// The slug to the skill.
    pub slug: String,
    /// The icon of the skill.
    pub icon: String,
    /// The description of the skill.
    pub description: String,
    /// The tier of the skill.
    pub tier: u8,
    /// Tags attached to the skill.
    pub tags: Vec<Tag>,
    /// The effects the skill causes to the opponent.
    pub causes: Vec<SkillStatusEffect>,
    /// The effects the skill gives to the caster.
    pub gives: Vec<SkillStatusEffect>,
}

impl CodexSkill {
    /// Return true if the skill is an off-hand skill.
    pub fn is_offhand(&self) -> bool {
        self.tags.contains(&Tag::OffHandAbility)
    }

    /// Return true if the skill is bought at the arcanist.
    pub fn bought_at_arcanist(&self) -> bool {
        self.tags.contains(&Tag::FoundInArcanists)
    }

    /// Try to convert `self` to an `AdminSkill`.
    /// Unknown status effects are ignored, rather than returning an error.
    pub fn try_to_admin_skill(&self, static_: &Static) -> Result<AdminSkill, Error> {
        Ok(AdminSkill {
            codex_uri: format!("/codex/spells/{}/", self.slug),
            name: if self.is_offhand() {
                format!("{} [off-hand]", self.name)
            } else if self.slug.starts_with("Zwei") {
                format!("{} [zwei]", self.name)
            } else {
                self.name.clone()
            },
            tier: self.tier,
            description: if !self.description.is_empty() {
                self.description.clone()
            } else {
                ".".to_string()
            },
            offhand: self.is_offhand(),
            bought: self.bought_at_arcanist(),
            causes: self
                .causes
                .try_to_guide_ids(static_)
                .ignore_failed_id_conversions()?,
            gives: self
                .gives
                .try_to_guide_ids(static_)
                .ignore_failed_id_conversions()?,
            ..AdminSkill::default()
        })
    }
}

/// Collection of skills from the codex.
#[derive(Serialize, Deserialize)]
pub struct CodexSkills {
    /// Skills from the codex.
    pub skills: Vec<CodexSkill>,
}

impl<'a> CodexSkills {
    /// Find the codex skill associated with the given URI.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a CodexSkill> {
        static URI_START: &str = "/codex/spells/";
        if !needle.starts_with(URI_START) {
            return None;
        }

        let slug = &needle[URI_START.len()..needle.len() - 1];
        self.skills.iter().find(|skill| skill.slug == slug)
    }

    /// Find the codex skill associated with the given URI.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a CodexSkill, Error> {
        self.find_by_uri(needle)
            .ok_or_else(|| Error::Misc(format!("No match for codex skill with uri '{}'", needle)))
    }
}
