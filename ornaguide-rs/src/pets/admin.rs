use serde::{Deserialize, Serialize};

use crate::{error::Error, guide::html_form_parser::ParsedForm};

/// The kind of currency a pet costs.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum CostType {
    Orn,
    Gold,
}

/// A pet fetched from the admin panel.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdminPet {
    /// The CSRF token that was given on the page where the pet was fetched.
    #[serde(skip)]
    pub(crate) csrfmiddlewaretoken: String,
    /// Id of the pet on the guide.
    pub id: u32,
    /// The URI of the pet on the codex.
    /// URI matches `/codex/followers/{slug}/` with the trailing slash.
    pub codex_uri: String,
    /// The name of the pet on the guide.
    pub name: String,
    /// The tier of the pet.
    pub tier: u8,
    /// Path to the image of the pet.
    pub image_name: String,
    /// In-game description of the pet.
    pub description: String,
    /// Pet attack chance (%).
    pub attack: u8,
    /// Pet heal chance (%).
    pub heal: u8,
    /// Pet buff chance (%).
    pub buff: u8,
    /// Pet debuff chance (%).
    pub debuff: u8,
    /// Pet spell chance (%).
    pub spell: u8,
    /// Pet protect chance (%).
    pub protect: u8,
    /// Pet cost.
    pub cost: u64,
    /// Pet cost type (Orns or Gold).
    pub cost_type: CostType,
    /// Whether the pet is limited (i.e.: tied to an event).
    pub limited: bool,
    /// Handwritten note from the guide team on availability.
    pub limited_details: String,
    /// Ids of skills the pet knows.
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

/// Collection of pets from the guide's admin view.
#[derive(Serialize, Deserialize)]
pub struct AdminPets {
    /// Pets from the guide's admin view.
    pub pets: Vec<AdminPet>,
}

impl<'a> AdminPets {
    /// Find the admin pet associated with the given slug.
    pub fn find_by_slug(&'a self, needle: &str) -> Option<&'a AdminPet> {
        self.pets.iter().find(|pet| {
            !pet.codex_uri.is_empty()
                && pet.codex_uri["/codex/followers/".len()..].trim_end_matches('/') == needle
        })
    }

    /// Find the admin pet associated with the given codex follower.
    /// If there is no match, return an `Err`.
    pub fn get_by_slug(&'a self, needle: &str) -> Result<&'a AdminPet, Error> {
        self.find_by_slug(needle).ok_or_else(|| {
            Error::Misc(format!(
                "No match for admin pet with codex slug '{}'",
                needle
            ))
        })
    }

    /// Find the admin pet associated with the given id.
    pub fn find_by_id(&'a self, needle: u32) -> Option<&'a AdminPet> {
        self.pets.iter().find(|pet| pet.id == needle)
    }

    /// Find the admin pet associated with the given id.
    /// If there is no match, return an `Err`.
    pub fn get_by_id(&'a self, needle: u32) -> Result<&'a AdminPet, Error> {
        self.find_by_id(needle)
            .ok_or_else(|| Error::Misc(format!("No match for admin pet with id #{}", needle)))
    }
}
