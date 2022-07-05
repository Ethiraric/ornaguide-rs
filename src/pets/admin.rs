use serde::{Deserialize, Serialize};

use crate::{error::Error, guide::html_form_parser::ParsedForm};

/// The kind of currency a pet costs.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CostType {
    Orn,
    Gold,
}

/// An item fetched from the admin panel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminPet {
    pub(crate) csrfmiddlewaretoken: String,
    pub id: u32,
    pub codex_uri: String,
    pub name: String,
    pub tier: u8,
    pub image_name: String,
    pub description: String,
    pub attack: u8,
    pub heal: u8,
    pub buff: u8,
    pub debuff: u8,
    pub spell: u8,
    pub protect: u8,
    pub cost: u64,
    pub cost_type: CostType,
    pub limited: bool,
    pub limited_details: String,
    pub skills: Vec<u32>,
}

impl Default for AdminPet {
    fn default() -> Self {
        AdminPet {
            csrfmiddlewaretoken: String::new(),
            id: 0,
            codex_uri: String::new(),
            name: String::new(),
            tier: 0,
            image_name: String::new(),
            description: String::new(),
            attack: 0,
            heal: 0,
            buff: 0,
            debuff: 0,
            spell: 0,
            protect: 0,
            cost: 0,
            cost_type: CostType::Gold,
            limited: false,
            limited_details: String::new(),
            skills: Vec::new(),
        }
    }
}
impl TryFrom<ParsedForm> for AdminPet {
    type Error = Error;

    fn try_from(form: ParsedForm) -> Result<Self, Self::Error> {
        let mut pet = AdminPet {
            csrfmiddlewaretoken: form.csrfmiddlewaretoken,
            ..Default::default()
        };

        for (key, value) in form.fields.into_iter() {
            match key.as_str() {
                "codex" => pet.codex_uri = value,
                "name" => pet.name = value,
                "tier" => pet.tier = value.parse()?,
                "image_name" => pet.image_name = value,
                "description" => pet.description = value,
                "attack" => pet.attack = value.parse()?,
                "heal" => pet.heal = value.parse()?,
                "buff" => pet.buff = value.parse()?,
                "debuff" => pet.debuff = value.parse()?,
                "spell" => pet.spell = value.parse()?,
                "protect" => pet.protect = value.parse()?,
                "cost" => pet.cost = value.parse()?,
                "cost_type" => {
                    pet.cost_type = if value.parse::<u8>()? == 1 {
                        CostType::Orn
                    } else {
                        CostType::Gold
                    }
                }
                "limited" => pet.limited = value == "on",
                "limited_details" => pet.limited_details = value,
                "skills" => pet.skills.push(value.parse()?),
                key => {
                    return Err(Error::ExtraField(key.to_string(), value));
                }
            }
        }

        Ok(pet)
    }
}

impl From<AdminPet> for ParsedForm {
    fn from(pet: AdminPet) -> Self {
        let mut form = ParsedForm {
            csrfmiddlewaretoken: pet.csrfmiddlewaretoken,
            ..ParsedForm::default()
        };

        let mut push = |key: &str, value: String| form.fields.push((key.to_string(), value));

        push("codex", pet.codex_uri);
        push("name", pet.name);
        push("tier", pet.tier.to_string());
        push("image_name", pet.image_name);
        push("description", pet.description);
        push("attack", pet.attack.to_string());
        push("heal", pet.heal.to_string());
        push("buff", pet.buff.to_string());
        push("debuff", pet.debuff.to_string());
        push("spell", pet.spell.to_string());
        push("protect", pet.protect.to_string());
        push("cost", pet.cost.to_string());
        push(
            "cost_type",
            match pet.cost_type {
                CostType::Orn => "1",
                CostType::Gold => "2",
            }
            .to_string(),
        );
        if pet.limited {
            push("limited", "on".to_string());
        }
        push("limited_details", pet.limited_details.to_string());
        for x in pet.skills.iter() {
            push("skills", x.to_string());
        }

        form
    }
}
