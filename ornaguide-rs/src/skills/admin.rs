use serde::{Deserialize, Serialize};

use crate::{error::Error, guide::html_form_parser::ParsedForm, misc::sanitize_guide_name};

/// A skill fetched from the admin panel.
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
    /// The id of the type of the skill (Buff, Attack, AoE Debuff, ...).
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
        let mut item = AdminSkill {
            csrfmiddlewaretoken: form.csrfmiddlewaretoken,
            ..Default::default()
        };

        for (key, value) in form.fields.into_iter() {
            match key.as_str() {
                "codex" => item.codex_uri = value,
                "name" => item.name = value,
                "tier" => item.tier = value.parse()?,
                "type" => item.type_ = value.parse()?,
                "is_magic" => item.is_magic = value == "on",
                "mana_cost" => item.mana_cost = value.parse()?,
                "description" => item.description = value,
                "element" => {
                    item.element = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                "offhand" => item.offhand = value == "on",
                "cost" => item.cost = value.parse()?,
                "bought" => item.bought = value == "on",
                "skill_power" => item.skill_power = value.parse()?,
                "strikes" => item.strikes = value.parse()?,
                "modifier_min" => item.modifier_min = value.parse()?,
                "modifier_max" => item.modifier_max = value.parse()?,
                "extra" => item.extra = value,
                "buffed_by" => item.buffed_by.push(value.parse()?),
                "causes" => item.causes.push(value.parse()?),
                "cures" => item.cures.push(value.parse()?),
                "gives" => item.gives.push(value.parse()?),
                key => {
                    return Err(Error::ExtraField(key.to_string(), value));
                }
            }
        }

        Ok(item)
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
        for x in item.causes.iter() {
            push("causes", x.to_string());
        }
        for x in item.cures.iter() {
            push("cures", x.to_string());
        }
        for x in item.gives.iter() {
            push("gives", x.to_string());
        }

        form
    }
}

/// Collection of skills from the guide's admin view.
#[derive(Serialize, Deserialize, Clone, Default, PartialEq)]
pub struct AdminSkills {
    /// Skills from the guide's admin view.
    pub skills: Vec<AdminSkill>,
}

impl<'a> AdminSkills {
    /// Find the admin skill corresponding to the given id.
    pub fn find_by_id(&'a self, needle: u32) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| skill.id == needle)
    }

    /// Find the admin skill corresponding to the given id.
    /// If there is no match, return an `Err`.
    pub fn get_by_id(&'a self, needle: u32) -> Result<&'a AdminSkill, Error> {
        self.find_by_id(needle)
            .ok_or_else(|| Error::Misc(format!("No match for admin skill with id #{}", needle)))
    }

    /// Find the admin skill corresponding to the given codex URI.
    pub fn find_by_uri(&'a self, needle: &str) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| skill.codex_uri == needle)
    }

    /// Find the admin skill corresponding to the given codex URI.
    /// If there is no match, return an `Err`.
    pub fn get_by_uri(&'a self, needle: &str) -> Result<&'a AdminSkill, Error> {
        self.find_by_uri(needle).ok_or_else(|| {
            Error::Misc(format!(
                "No match for admin skill with codex_uri {}",
                needle
            ))
        })
    }

    /// Find the admin skill associated with the given slug
    pub fn find_by_slug(&'a self, needle: &str) -> Option<&'a AdminSkill> {
        self.skills.iter().find(|skill| {
            !skill.codex_uri.is_empty()
                && skill.codex_uri["/codex/spells/".len()..].trim_end_matches('/') == needle
        })
    }

    /// Find the admin skill associated with the given slug.
    /// If there is no match, return an `Err`.
    pub fn get_by_slug(&'a self, needle: &str) -> Result<&'a AdminSkill, Error> {
        self.find_by_slug(needle).ok_or_else(|| {
            Error::Misc(format!(
                "No match for admin skill with codex slug '{}'",
                needle
            ))
        })
    }

    /// Find the admin offhand skill with the given name.
    pub fn find_offhand_from_name(&'a self, needle: &str) -> Option<&'a AdminSkill> {
        self.skills
            .iter()
            .find(|skill| sanitize_guide_name(&skill.name) == needle && skill.offhand)
    }

    /// Find the admin offhand skill with the given name.
    /// If there is no match, return an `Err`.
    pub fn get_offhand_from_name(&'a self, needle: &str) -> Result<&'a AdminSkill, Error> {
        self.find_offhand_from_name(needle).ok_or_else(|| {
            Error::Misc(format!(
                "No match for offhand admin skill with name '{}'",
                needle
            ))
        })
    }
}
