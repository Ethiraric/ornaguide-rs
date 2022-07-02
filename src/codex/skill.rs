use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    codex::Tag,
    error::Error,
    guide::Static,
    misc::{codex_effect_name_iter_to_guide_id_results, codex_effect_name_to_guide_name},
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
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error>;
    /// Convert the list of status effects to a list of effect names, matching those of the guide.
    fn to_guide_names(&self) -> Vec<String>;
}

impl SkillStatusEffects for Vec<SkillStatusEffect> {
    fn try_to_guide_ids(&self, static_: &Static) -> Result<Vec<u32>, Error> {
        codex_effect_name_iter_to_guide_id_results(
            self.iter().map(|name| name.effect.as_str()),
            static_,
        )
        .collect::<Result<Vec<_>, Error>>()
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
    pub fn try_to_admin_skill(&self, static_: &Static) -> Result<AdminSkill, Error> {
        Ok(AdminSkill {
            name: self.name.clone(),
            tier: self.tier,
            codex_uri: format!("/codex/spells/{}/", self.slug),
            description: if !self.description.is_empty() {
                self.description.clone()
            } else {
                ".".to_string()
            },
            bought: self.bought_at_arcanist(),
            causes: self.causes.try_to_guide_ids(static_)?,
            gives: self.gives.try_to_guide_ids(static_)?,
            ..AdminSkill::default()
        })
    }
}
