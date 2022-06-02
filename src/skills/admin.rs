use serde::{Deserialize, Serialize};

use crate::{error::Error, guide::html_form_parser::ParsedForm};

/// A skill fetched from the admin panel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminSkill {
    pub(crate) csrfmiddlewaretoken: String,
    pub id: u32,
    pub name: String,
    pub tier: u32,
    pub type_: u32,
    pub is_magic: bool,
    pub mana_cost: u32,
    pub description: String,
    pub element: Option<u32>,
    pub offhand: bool,
    pub cost: u64,
    pub bought: bool,
    pub skill_power: f32,
    pub strikes: u8,
    pub modifier_min: f32,
    pub modifier_max: f32,
    pub extra: String,
    pub buffed_by: Vec<u32>,
    pub causes: Vec<u32>,
    pub cures: Vec<u32>,
    pub gives: Vec<u32>,
}

impl Default for AdminSkill {
    fn default() -> Self {
        AdminSkill {
            csrfmiddlewaretoken: String::new(),
            id: 0,
            name: String::new(),
            tier: 0,
            type_: 0,
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
