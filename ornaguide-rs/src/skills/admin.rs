use serde::{Deserialize, Serialize};

use crate::{
    error::{Error, Kind},
    guide::html_form_parser::ParsedForm,
    misc::sanitize_guide_name,
    parse_stat, parse_stat_opt, parse_stat_vec,
};

/// A skill fetched from the admin panel.
#[allow(clippy::module_name_repetitions)]
#[derive(Clone, Debug, Serialize, Deserialize, Derivative)]
#[derivative(PartialEq)]
pub struct AdminSkill {
    /// The CSRF token that was given on the page where the skill was fetched.
    #[serde(skip)]
    #[derivative(PartialEq = "ignore")]
    pub(crate) csrfmiddlewaretoken: String,
    /// Id of the skill on the guide.
    pub id: u32,
    /// The URI of the skill on the codex.
    /// URI matches `/codex/spells/{slug}/` with the trailing slash.
    pub codex_uri: String,
    /// The name of the skill on the guide.
    pub name: String,
    /// The tier of the skill.
    pub tier: u8,
    /// The id of the type of the skill (Buff, Attack, `AoE` Debuff, ...).
    pub type_: u32,
    /// Whether the skill is a magic one.
    pub is_magic: bool,
    /// The mana cost of the skill.
    pub mana_cost: u32,
    /// The in-game description of the skill.
    pub description: String,
    /// ID of the element of the skill.
    pub element: Option<u32>,
    /// Whether the skill is an off-hand skill.
    /// Off-hand skills have their own entry, that is distinct from the non-off-hand ones.
    pub offhand: bool,
    /// The gold cost of the skill if it can be bought at the arcanist.
    pub cost: u64,
    /// Whether the skill can be bought at an arcanist.
    pub bought: bool,
    /// M1 of the skill.
    pub skill_power: f32,
    /// Number of times the skill strikes.
    pub strikes: u8,
    /// Min M2 of the skill.
    pub modifier_min: f32,
    /// Max M2 of the skill.
    pub modifier_max: f32,
    /// Handwritten notes from the guide team on the item.
    pub extra: String,
    /// Ids of monsters who buff this skill (if a passive that requires kills).
    pub buffed_by: Vec<u32>,
    /// Ids of status effects the skill inflicts.
    pub causes: Vec<u32>,
    /// Ids of status effects the skill cures.
    pub cures: Vec<u32>,
    /// Ids of status effects the skill gives.
    pub gives: Vec<u32>,
}

impl AdminSkill {
    /// Return the slug of the skill.
    /// If the skill has no `codex_uri`, return an empty string.
    #[must_use]
    pub fn slug(&self) -> &str {
        if self.codex_uri.is_empty() {
            ""
        } else {
            &self.codex_uri["/codex/skills/".len()..self.codex_uri.len() - 1]
        }
    }
}

impl Default for AdminSkill {
    fn default() -> Self {
        AdminSkill {
            csrfmiddlewaretoken: String::new(),
            id: 0,
            codex_uri: String::new(),
            name: String::new(),
            tier: 0,
            type_: 16, // Corresponds to "TBD" on guide.
            is_magic: false,
            mana_cost: 0,
            description: String::new(),
            element: None,
            offhand: false,
            cost: 0,
            bought: false,
            skill_power: 1.0,
            strikes: 1,
            modifier_min: 1.0,
            modifier_max: 1.0,
            extra: String::new(),
            buffed_by: Vec::new(),
            causes: Vec::new(),
            cures: Vec::new(),
            gives: Vec::new(),
        }
    }
}

impl TryFrom<ParsedForm> for AdminSkill {
    type Error = Error;

    fn try_from(form: ParsedForm) -> Result<Self, Self::Error> {
        let mut skill = AdminSkill {
            csrfmiddlewaretoken: form.csrfmiddlewaretoken,
            ..Default::default()
        };

        for (key, value) in form.fields {
            // Helper macros to parse and add meaningful error messages.
            macro_rules! stat {
                ($field:ident) => {
                    parse_stat!(skill, $field, value)
                };
            }
            macro_rules! opt {
                ($field:ident) => {
                    parse_stat_opt!(skill, $field, value)
                };
            }
            macro_rules! push {
                ($field:ident) => {
                    parse_stat_vec!(skill, $field, value)
                };
            }

            match key.as_str() {
                "codex" => skill.codex_uri = value,
                "name" => skill.name = value,
                "tier" => stat!(tier),
                "type" => stat!(type_),
                "is_magic" => skill.is_magic = value == "on",
                "mana_cost" => stat!(mana_cost),
                "description" => skill.description = value,
                "element" => opt!(element),
                "offhand" => skill.offhand = value == "on",
                "cost" => stat!(cost),
                "bought" => skill.bought = value == "on",
                "skill_power" => stat!(skill_power),
                "strikes" => stat!(strikes),
                "modifier_min" => stat!(modifier_min),
                "modifier_max" => stat!(modifier_max),
                "extra" => skill.extra = value,
                "buffed_by" => push!(buffed_by),
                "causes" => push!(causes),
                "cures" => push!(cures),
                "gives" => push!(gives),
                key => {
                    return Err(Kind::ExtraField(key.to_string(), value).into());
                }
            }
        }

        Ok(skill)
    }
}

impl From<AdminSkill> for ParsedForm {
    fn from(item: AdminSkill) -> Self {
        let mut form = ParsedForm {
            csrfmiddlewaretoken: item.csrfmiddlewaretoken,
            ..ParsedForm::default()
        };

        let mut push = |key: &str, value: String| form.fields.push((key.to_string(), value));

        push("name", item.name);
        push("codex", item.codex_uri);
        push("tier", item.tier.to_string());
        push("type", item.type_.to_string());
        if item.is_magic {
            push("is_magic", "on".to_string());
        }
        push("mana_cost", item.mana_cost.to_string());
        push("description", item.description);
        if let Some(element) = item.element {
            push("element", element.to_string());
        } else {
            push("element", String::new());
        }
        if item.offhand {
            push("offhand", "on".to_string());
        }
        push("cost", item.cost.to_string());
        if item.bought {
            push("bought", "on".to_string());
        }
        push("skill_power", item.skill_power.to_string());
        push("strikes", item.strikes.to_string());
        push("modifier_min", item.modifier_min.to_string());
        push("modifier_max", item.modifier_max.to_string());
        push("extra", item.extra);
        for x in &item.causes {
            push("causes", x.to_string());
        }
        for x in &item.cures {
            push("cures", x.to_string());
        }
        for x in &item.gives {
            push("gives", x.to_string());
        }

        form
    }
}

/// Collection of skills from the guide's admin view.
#[allow(clippy::module_name_repetitions)]
#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct AdminSkills {
    /// Skills from the guide's admin view.
    pub skills: Vec<AdminSkill>,
}

impl<'a> AdminSkills {
    /// Find the admin skill corresponding to the given id.
    #[must_use]
    pub fn find_by_id(&'a self, needle: u32) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| skill.id == needle)
    }

    /// Find the admin skill corresponding to the given id.
    ///
    /// # Errors
    /// Errors if there is no match.
    pub fn get_by_id(&'a self, needle: u32) -> Result<&'a AdminSkill, Error> {
        self.find_by_id(needle)
            .ok_or_else(|| Kind::Misc(format!("No match for admin skill with id #{needle}")).into())
    }

    /// Find the admin skill corresponding to the given codex URI.
    #[must_use]
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| skill.codex_uri == needle)
    }

    /// Find the admin skill corresponding to the given codex URI.
    ///
    /// # Errors
    /// Errors if there is no match.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a AdminSkill, Error> {
        self.find_by_uri(needle).ok_or_else(|| {
            Kind::Misc(format!("No match for admin skill with codex_uri {needle}")).into()
        })
    }

    /// Find the admin skill associated with the given slug
    #[must_use]
    pub fn find_by_slug(&'a self, needle: &str) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| {
            !skill.codex_uri.is_empty()
                && skill.codex_uri["/codex/spells/".len()..].trim_end_matches('/') == needle
        })
    }

    /// Find the admin skill associated with the given slug.
    ///
    /// # Errors
    /// Errors if there is no match.
    pub fn get_by_slug(&'a self, needle: &str) -> Result<&'a AdminSkill, Error> {
        self.find_by_slug(needle).ok_or_else(|| {
            Kind::Misc(format!(
                "No match for admin skill with codex slug '{needle}'"
            ))
            .into()
        })
    }

    /// Find the admin offhand skill with the given name.
    #[must_use]
    pub fn find_offhand_from_name(&'a self, needle: &str) -> Option<&'a AdminSkill> {
        self.skills
            .iter()
            .find(|skill| sanitize_guide_name(&skill.name) == needle && skill.offhand)
    }

    /// Find the admin offhand skill with the given name.
    ///
    /// # Errors
    /// Errors if there is no match.
    pub fn get_offhand_from_name(&'a self, needle: &str) -> Result<&'a AdminSkill, Error> {
        self.find_offhand_from_name(needle).ok_or_else(|| {
            Kind::Misc(format!(
                "No match for offhand admin skill with name '{needle}'"
            ))
            .into()
        })
    }
}
