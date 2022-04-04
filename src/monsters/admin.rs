use crate::{error::Error, guide::html_parser::ParsedForm};

/// An item fetched from the admin panel.
#[derive(Clone, Debug, Default)]
pub struct AdminMonster {
    pub(crate) csrfmiddlewaretoken: String,
    pub id: u32,
    pub name: String,
    pub tier: u32,
    pub family: Option<u32>,
    pub image_name: String,
    pub boss: bool,
    pub level: u32,
    pub notes: String,
    pub spawns: Vec<u32>,
    pub weak_to: Vec<u32>,
    pub resistant_to: Vec<u32>,
    pub immune_to: Vec<u32>,
    pub immune_to_status: Vec<u32>,
    pub vulnerable_to_status: Vec<u32>,
    pub drops: Vec<u32>,
    pub skills: Vec<u32>,
}

impl TryFrom<ParsedForm> for AdminMonster {
    type Error = Error;

    fn try_from(form: ParsedForm) -> Result<Self, Self::Error> {
        let mut item = AdminMonster {
            csrfmiddlewaretoken: form.csrfmiddlewaretoken,
            ..Default::default()
        };

        for (key, value) in form.fields.into_iter() {
            match key.as_str() {
                "name" => item.name = value,
                "tier" => item.tier = value.parse()?,
                "family" => {
                    item.family = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                "image_name" => item.image_name = value,
                "boss" => item.boss = value == "on",
                "level" => item.level = value.parse()?,
                "notes" => item.notes = value,
                "spawns" => item.spawns.push(value.parse()?),
                "weak_to" => item.weak_to.push(value.parse()?),
                "resistant_to" => item.resistant_to.push(value.parse()?),
                "immune_to" => item.immune_to.push(value.parse()?),
                "immune_to_status" => item.immune_to_status.push(value.parse()?),
                "vulnerable_to_status" => item.vulnerable_to_status.push(value.parse()?),
                "drops" => item.drops.push(value.parse()?),
                "skills" => item.skills.push(value.parse()?),
                key => {
                    return Err(Error::ExtraField(key.to_string(), value));
                }
            }
        }

        Ok(item)
    }
}

impl From<AdminMonster> for ParsedForm {
    fn from(item: AdminMonster) -> Self {
        let mut form = ParsedForm {
            csrfmiddlewaretoken: item.csrfmiddlewaretoken,
            ..ParsedForm::default()
        };

        let mut push = |key: &str, value: String| form.fields.push((key.to_string(), value));

        push("name", item.name);
        push("tier", item.tier.to_string());
        push(
            "family",
            item.family
                .map(|family| family.to_string())
                .unwrap_or_else(String::new),
        );
        push("image_name", item.image_name);
        if item.boss {
            push("boss", "on".to_string());
        }
        push("level", item.level.to_string());
        push("notes", item.notes);

        for x in item.spawns.iter() {
            push("spawns", x.to_string());
        }
        for x in item.weak_to.iter() {
            push("weak_to", x.to_string());
        }
        for x in item.resistant_to.iter() {
            push("resistant_to", x.to_string());
        }
        for x in item.immune_to.iter() {
            push("immune_to", x.to_string());
        }
        for x in item.immune_to_status.iter() {
            push("immune_to_status", x.to_string());
        }
        for x in item.vulnerable_to_status.iter() {
            push("vulnerable_to_status", x.to_string());
        }
        for x in item.drops.iter() {
            push("drops", x.to_string());
        }
        for x in item.skills.iter() {
            push("skills", x.to_string());
        }

        form
    }
}
