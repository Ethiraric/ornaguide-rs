use crate::{error::Error, guide::html_parser::ParsedForm};

/// An item fetched from the admin panel.
#[derive(Clone, Debug)]
pub struct AdminItem {
    pub(crate) csrfmiddlewaretoken: String,
    pub id: u32,
    pub name: String,
    pub tier: u32,
    pub type_: u32,
    pub image_name: String,
    pub description: String,
    pub hp: i32,
    pub hp_affected_by_quality: bool,
    pub mana: i32,
    pub mana_affected_by_quality: bool,
    pub attack: i32,
    pub attack_affected_by_quality: bool,
    pub magic: i32,
    pub magic_affected_by_quality: bool,
    pub defense: i32,
    pub defense_affected_by_quality: bool,
    pub resistance: i32,
    pub resistance_affected_by_quality: bool,
    pub dexterity: i32,
    pub dexterity_affected_by_quality: bool,
    pub ward: i32,
    pub ward_affected_by_quality: bool,
    pub crit: i32,
    pub crit_affected_by_quality: bool,
    pub view_distance: bool,
    pub element: Option<u32>,
    pub equipped_by: Vec<u32>,
    pub two_handed: bool,
    pub orn_bonus: f32,
    pub gold_bonus: f32,
    pub drop_bonus: f32,
    pub spawn_bonus: f32,
    pub exp_bonus: Vec<f32>,
    pub boss: bool,
    pub arena: bool,
    pub category: Option<u32>,
    pub causes: Vec<u32>,
    pub cures: Vec<u32>,
    pub gives: Vec<u32>,
    pub prevents: Vec<u32>,
    pub materials: Vec<u32>,
    pub price: u32,
    pub ability: Option<u32>,
}

impl Default for AdminItem {
    fn default() -> Self {
        AdminItem {
            csrfmiddlewaretoken: String::new(),
            id: 0,
            name: String::new(),
            tier: 0,
            type_: 0,
            image_name: String::new(),
            description: String::new(),
            hp: 0,
            hp_affected_by_quality: false,
            mana: 0,
            mana_affected_by_quality: false,
            attack: 0,
            attack_affected_by_quality: false,
            magic: 0,
            magic_affected_by_quality: false,
            defense: 0,
            defense_affected_by_quality: false,
            resistance: 0,
            resistance_affected_by_quality: false,
            dexterity: 0,
            dexterity_affected_by_quality: false,
            ward: 0,
            ward_affected_by_quality: false,
            crit: 0,
            crit_affected_by_quality: false,
            view_distance: false,
            element: None,
            equipped_by: Vec::new(),
            two_handed: false,
            orn_bonus: 0.0,
            gold_bonus: 0.0,
            drop_bonus: 0.0,
            spawn_bonus: 0.0,
            exp_bonus: Vec::new(),
            boss: false,
            arena: false,
            category: None,
            causes: Vec::new(),
            cures: Vec::new(),
            gives: Vec::new(),
            prevents: Vec::new(),
            materials: Vec::new(),
            price: 0,
            ability: None,
        }
    }
}

impl TryFrom<ParsedForm> for AdminItem {
    type Error = Error;

    fn try_from(form: ParsedForm) -> Result<Self, Self::Error> {
        let mut item = AdminItem {
            csrfmiddlewaretoken: form.csrfmiddlewaretoken,
            ..Default::default()
        };

        for (key, value) in form.fields.into_iter() {
            match key.as_str() {
                "name" => item.name = value,
                "tier" => item.tier = value.parse()?,
                "type" => item.type_ = value.parse()?,
                "image_name" => item.image_name = value,
                "description" => item.description = value,
                "hp" => item.hp = value.parse()?,
                "hp_affected_by_quality" => item.hp_affected_by_quality = value == "on",
                "mana" => item.mana = value.parse()?,
                "mana_affected_by_quality" => item.mana_affected_by_quality = value == "on",
                "attack" => item.attack = value.parse()?,
                "attack_affected_by_quality" => item.attack_affected_by_quality = value == "on",
                "magic" => item.magic = value.parse()?,
                "magic_affected_by_quality" => item.magic_affected_by_quality = value == "on",
                "defense" => item.defense = value.parse()?,
                "defense_affected_by_quality" => item.defense_affected_by_quality = value == "on",
                "resistance" => item.resistance = value.parse()?,
                "resistance_affected_by_quality" => {
                    item.resistance_affected_by_quality = value == "on"
                }
                "dexterity" => item.dexterity = value.parse()?,
                "dexterity_affected_by_quality" => {
                    item.dexterity_affected_by_quality = value == "on"
                }
                "ward" => item.ward = value.parse()?,
                "ward_affected_by_quality" => item.ward_affected_by_quality = value == "on",
                "crit" => item.crit = value.parse()?,
                "crit_affected_by_quality" => item.crit_affected_by_quality = value == "on",
                "view_distance" => item.view_distance = value == "on",
                "element" => {
                    item.element = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                "equipped_by" => item.equipped_by.push(value.parse()?),
                "two_handed" => item.two_handed = value == "on",
                "orn_bonus" => item.orn_bonus = value.parse()?,
                "gold_bonus" => item.gold_bonus = value.parse()?,
                "drop_bonus" => item.drop_bonus = value.parse()?,
                "spawn_bonus" => item.spawn_bonus = value.parse()?,
                "exp_bonus" => item.exp_bonus.push(value.parse()?),
                "boss" => item.boss = value == "on",
                "arena" => item.arena = value == "on",
                "category" => {
                    item.category = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                "causes" => item.causes.push(value.parse()?),
                "cures" => item.cures.push(value.parse()?),
                "gives" => item.gives.push(value.parse()?),
                "prevents" => item.prevents.push(value.parse()?),
                "materials" => item.materials.push(value.parse()?),
                "price" => item.price = value.parse()?,
                "ability" => {
                    item.ability = if value.is_empty() {
                        None
                    } else {
                        Some(value.parse()?)
                    }
                }
                _ => {}
            }
        }

        Ok(item)
    }
}

impl From<AdminItem> for ParsedForm {
    fn from(item: AdminItem) -> Self {
        let mut form = ParsedForm {
            csrfmiddlewaretoken: item.csrfmiddlewaretoken,
            ..ParsedForm::default()
        };

        let mut push = |key: &str, value: String| form.fields.push((key.to_string(), value));

        push("name", item.name);
        push("tier", item.tier.to_string());
        push("type", item.type_.to_string());
        push("image_name", item.image_name);
        push("description", item.description);

        push("hp", item.hp.to_string());
        if item.hp_affected_by_quality {
            push("hp_affected_by_quality", "on".to_string());
        }
        push("mana", item.mana.to_string());
        if item.mana_affected_by_quality {
            push("mana_affected_by_quality", "on".to_string());
        }
        push("attack", item.attack.to_string());
        if item.attack_affected_by_quality {
            push("attack_affected_by_quality", "on".to_string());
        }
        push("magic", item.magic.to_string());
        if item.magic_affected_by_quality {
            push("magic_affected_by_quality", "on".to_string());
        }
        push("defense", item.defense.to_string());
        if item.defense_affected_by_quality {
            push("defense_affected_by_quality", "on".to_string());
        }
        push("resistance", item.resistance.to_string());
        if item.resistance_affected_by_quality {
            push("resistance_affected_by_quality", "on".to_string());
        }
        push("dexterity", item.dexterity.to_string());
        if item.dexterity_affected_by_quality {
            push("dexterity_affected_by_quality", "on".to_string());
        }
        push("ward", item.ward.to_string());
        if item.ward_affected_by_quality {
            push("ward_affected_by_quality", "on".to_string());
        }
        push("crit", item.crit.to_string());
        if item.crit_affected_by_quality {
            push("crit_affected_by_quality", "on".to_string());
        }

        if item.view_distance {
            push("view_distance", "on".to_string());
        }
        if let Some(element) = item.element {
            push("element", element.to_string());
        } else {
            push("element", String::new());
        }
        for x in item.equipped_by.iter() {
            push("equipped_by", x.to_string());
        }
        if item.two_handed {
            push("two_handed", "on".to_string());
        }

        push("orn_bonus", item.orn_bonus.to_string());
        push("gold_bonus", item.gold_bonus.to_string());
        push("drop_bonus", item.drop_bonus.to_string());
        push("spawn_bonus", item.spawn_bonus.to_string());
        for x in item.exp_bonus.iter() {
            push("exp_bonus", x.to_string());
        }

        if item.boss {
            push("boss", "on".to_string());
        }
        if item.arena {
            push("arena", "on".to_string());
        }
        if let Some(category) = item.category {
            push("category", category.to_string());
        } else {
            push("category", String::new());
        }

        for x in item.causes.iter() {
            push("causes", x.to_string());
        }
        for x in item.cures.iter() {
            push("cures", x.to_string());
        }
        for x in item.gives.iter() {
            push("gives", x.to_string());
        }
        for x in item.prevents.iter() {
            push("prevents", x.to_string());
        }

        for x in item.materials.iter() {
            push("materials", x.to_string());
        }
        push("price", item.price.to_string());
        if let Some(ability) = item.ability {
            push("ability", ability.to_string());
        } else {
            push("ability", String::new());
        }

        form
    }
}
