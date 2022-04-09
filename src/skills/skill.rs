use crate::error::Error;

pub use crate::skills::raw::{
    RawSkill, SkillBuffedBy, SkillLearnedBy, SkillMonsterUse, SkillPetUse,
};

/// An skill in Orna. This enum splits into types the different skills.
#[derive(Clone)]
pub enum Skill {
    /// A debuff skill.
    Debuff(DebuffSkill),
    /// A multi-round attack spell.
    MultiRoundAttack(MultiRoundAttackSkill),
    /// A multi-round magic spell.
    MultiRoundMagic(MultiRoundMagicSkill),
    /// A passive skill.
    Passive(PassiveSkill),
    /// A buff skill.
    Buff(BuffSkill),
    /// A magic skill.
    Magic(MagicSkill),
    /// An attack skill.
    Attack(AttackSkill),
    /// An aoe attack skill.
    AoeAttack(AoeAttackSkill),
    /// A healing skill.
    Healing(HealingSkill),
    /// A ward skill.
    Ward(WardSkill),
    /// An aoe magic skill.
    AoeMagic(AoeMagicSkill),
    /// An other skill.
    Other(OtherSkill),
    /// An aoe buff.
    AoeBuff(AoeBuffSkill),
    /// An aoe debuff.
    AoeDebuff(AoeDebuffSkill),
}

impl Skill {
    pub fn get_id(&self) -> u32 {
        match self {
            Skill::Debuff(x) => x.id,
            Skill::MultiRoundAttack(x) => x.id,
            Skill::MultiRoundMagic(x) => x.id,
            Skill::Passive(x) => x.id,
            Skill::Buff(x) => x.id,
            Skill::Magic(x) => x.id,
            Skill::Attack(x) => x.id,
            Skill::AoeAttack(x) => x.id,
            Skill::Healing(x) => x.id,
            Skill::Ward(x) => x.id,
            Skill::AoeMagic(x) => x.id,
            Skill::Other(x) => x.id,
            Skill::AoeBuff(x) => x.id,
            Skill::AoeDebuff(x) => x.id,
        }
    }
}

impl TryFrom<RawSkill> for Skill {
    type Error = Error;

    fn try_from(raw_skill: RawSkill) -> Result<Self, Self::Error> {
        match raw_skill.type_.as_str() {
            "Debuff" => Ok(Self::Debuff(DebuffSkill::try_from(raw_skill)?)),
            "Multi-round Attack" => Ok(Self::MultiRoundAttack(MultiRoundAttackSkill::try_from(
                raw_skill,
            )?)),
            "Multi-round Magic" => Ok(Self::MultiRoundMagic(MultiRoundMagicSkill::try_from(
                raw_skill,
            )?)),
            "Passive" => Ok(Self::Passive(PassiveSkill::try_from(raw_skill)?)),
            "Buff" => Ok(Self::Buff(BuffSkill::try_from(raw_skill)?)),
            "Magic" => Ok(Self::Magic(MagicSkill::try_from(raw_skill)?)),
            "Attack" => Ok(Self::Attack(AttackSkill::try_from(raw_skill)?)),
            "AoE Attack" => Ok(Self::AoeAttack(AoeAttackSkill::try_from(raw_skill)?)),
            "Healing" => Ok(Self::Healing(HealingSkill::try_from(raw_skill)?)),
            "Ward" => Ok(Self::Ward(WardSkill::try_from(raw_skill)?)),
            "AoE Magic" => Ok(Self::AoeMagic(AoeMagicSkill::try_from(raw_skill)?)),
            "Other" => Ok(Self::Other(OtherSkill::try_from(raw_skill)?)),
            "AoE Buff" => Ok(Self::AoeBuff(AoeBuffSkill::try_from(raw_skill)?)),
            "AoE Debuff" => Ok(Self::AoeDebuff(AoeDebuffSkill::try_from(raw_skill)?)),
            _ => Err(Error::InvalidField(
                "Skill".to_string(),
                "type".to_string(),
                Some(raw_skill.type_),
            )),
        }
    }
}

/// A debuff skill in Orna.
#[derive(Clone)]
pub struct DebuffSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub is_magic: bool,
    pub bought: bool,
    pub cost: Option<u64>,
    pub element: Option<String>,
    pub mana_cost: Option<u32>,
    pub causes: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for DebuffSkill {
    type Error = Error;

    /// Create a `Debuff` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Debuff`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if skill.type_ != "Debuff" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(skill.name.clone(), field.to_string());

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            is_magic: skill.is_magic,
            bought: skill.bought,
            cost: skill.cost,
            element: skill.element,
            mana_cost: skill.mana_cost,
            causes: skill.causes.ok_or_else(missing_field("causes"))?,
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// A multi-round attack skill in Orna.
#[derive(Clone)]
pub struct MultiRoundAttackSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub bought: bool,
    pub element: Option<String>,
    pub mana_cost: Option<u32>,
    pub causes: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for MultiRoundAttackSkill {
    type Error = Error;

    /// Create a `MultiRoundAttack` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Multi-round Attack`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "Multi-round Attack" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            bought: skill.bought,
            element: skill.element,
            mana_cost: skill.mana_cost,
            causes: skill.causes.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// A multi-round magic skill in Orna.
#[derive(Clone)]
pub struct MultiRoundMagicSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub bought: bool,
    pub element: Option<String>,
    pub mana_cost: Option<u32>,
    pub causes: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for MultiRoundMagicSkill {
    type Error = Error;

    /// Create a `MultiRoundMagic` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Multi-round Magic`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "Multi-round Magic" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            bought: skill.bought,
            element: skill.element,
            mana_cost: skill.mana_cost,
            causes: skill.causes.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// A passive skill in Orna.
#[derive(Clone)]
pub struct PassiveSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub buffed_by: Vec<SkillBuffedBy>,
    pub causes: Vec<String>,
    pub gives: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for PassiveSkill {
    type Error = Error;

    /// Create a `Passive` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Passive`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "Passive" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            buffed_by: skill.buffed_by.unwrap_or_default(),
            causes: skill.causes.unwrap_or_default(),
            gives: skill.gives.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// A buff skill in Orna.
#[derive(Clone)]
pub struct BuffSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub bought: bool,
    pub cost: Option<u64>,
    pub mana_cost: Option<u32>,
    pub cures: Vec<String>,
    pub causes: Vec<String>,
    pub gives: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for BuffSkill {
    type Error = Error;

    /// Create a `Buff` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Buff`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "Buff" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            bought: skill.bought,
            cost: skill.cost,
            mana_cost: skill.mana_cost,
            cures: skill.cures.unwrap_or_default(),
            causes: skill.causes.unwrap_or_default(),
            gives: skill.gives.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// A magic skill in Orna.
#[derive(Clone)]
pub struct MagicSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub bought: bool,
    pub cost: Option<u64>,
    pub element: Option<String>,
    pub mana_cost: Option<u32>,
    pub causes: Vec<String>,
    pub gives: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for MagicSkill {
    type Error = Error;

    /// Create a `Magic` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Magic`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "Magic" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            bought: skill.bought,
            cost: skill.cost,
            element: skill.element,
            mana_cost: skill.mana_cost,
            causes: skill.causes.unwrap_or_default(),
            gives: skill.gives.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// An attack skill in Orna.
#[derive(Clone)]
pub struct AttackSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub bought: bool,
    pub cost: Option<u64>,
    pub element: Option<String>,
    pub mana_cost: Option<u32>,
    pub causes: Vec<String>,
    pub gives: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for AttackSkill {
    type Error = Error;

    /// Create an `Attack` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Attack`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "Attack" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            bought: skill.bought,
            cost: skill.cost,
            element: skill.element,
            mana_cost: skill.mana_cost,
            causes: skill.causes.unwrap_or_default(),
            gives: skill.gives.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// An aoe attack skill in Orna.
#[derive(Clone)]
pub struct AoeAttackSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub mana_cost: u32,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for AoeAttackSkill {
    type Error = Error;

    /// Create an `AoeAoeAttack` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `AoeAttack`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if skill.type_ != "AoE Attack" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(skill.name.clone(), field.to_string());

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            mana_cost: skill.mana_cost.ok_or_else(missing_field("mana_cost"))?,
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// A healing skill in Orna.
#[derive(Clone)]
pub struct HealingSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub bought: bool,
    pub cost: Option<u64>,
    pub mana_cost: Option<u32>,
    pub cures: Vec<String>,
    pub gives: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for HealingSkill {
    type Error = Error;

    /// Create a `Healing` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Healing`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "Healing" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            bought: skill.bought,
            cost: skill.cost,
            mana_cost: skill.mana_cost,
            cures: skill.cures.unwrap_or_default(),
            gives: skill.gives.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// A ward skill in Orna.
#[derive(Clone)]
pub struct WardSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub is_magic: bool,
    pub bought: bool,
    pub cost: Option<u64>,
    pub element: Option<String>,
    pub mana_cost: Option<u32>,
    pub gives: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for WardSkill {
    type Error = Error;

    /// Create a `Ward` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Ward`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "Ward" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            is_magic: skill.is_magic,
            tier: skill.tier,
            bought: skill.bought,
            cost: skill.cost,
            element: skill.element,
            mana_cost: skill.mana_cost,
            gives: skill.gives.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// An aoe magic skill in Orna.
#[derive(Clone)]
pub struct AoeMagicSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub element: Option<String>,
    pub mana_cost: u32,
    pub causes: Option<Vec<String>>,
    pub learned_by: Vec<SkillLearnedBy>,
    pub pets_use: Vec<SkillPetUse>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for AoeMagicSkill {
    type Error = Error;

    /// Create an `AoeAoeMagic` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `AoeMagic`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if skill.type_ != "AoE Magic" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(skill.name.clone(), field.to_string());

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            element: skill.element,
            tier: skill.tier,
            mana_cost: skill.mana_cost.ok_or_else(missing_field("mana_cost"))?,
            causes: skill.causes,
            learned_by: skill.learned_by.unwrap_or_default(),
            pets_use: skill.pets_use.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}

/// An other skill in Orna.
#[derive(Clone)]
pub struct OtherSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub mana_cost: u32,
    pub learned_by: Vec<SkillLearnedBy>,
}

impl TryFrom<RawSkill> for OtherSkill {
    type Error = Error;

    /// Create an `Other` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `Other`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if skill.type_ != "Other" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(skill.name.clone(), field.to_string());

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            mana_cost: skill.mana_cost.ok_or_else(missing_field("mana_cost"))?,
            learned_by: skill.learned_by.unwrap_or_default(),
        })
    }
}

/// An aoe buff skill in Orna.
#[derive(Clone)]
pub struct AoeBuffSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub mana_cost: Option<u32>,
    pub gives: Vec<String>,
    pub learned_by: Vec<SkillLearnedBy>,
}

impl TryFrom<RawSkill> for AoeBuffSkill {
    type Error = Error;

    /// Create an `AoeBuff` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `AoE Buff`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "AoE Buff" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            mana_cost: skill.mana_cost,
            gives: skill.gives.unwrap_or_default(),
            learned_by: skill.learned_by.unwrap_or_default(),
        })
    }
}

/// An aoe debuff skill in Orna.
#[derive(Clone)]
pub struct AoeDebuffSkill {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub description: String,
    pub mana_cost: Option<u32>,
    pub causes: Vec<String>,
    pub monsters_use: Vec<SkillMonsterUse>,
}

impl TryFrom<RawSkill> for AoeDebuffSkill {
    type Error = Error;

    /// Create an `AoeDebuff` from a `RawSkill`.
    /// The `RawSkill`'s `type` field must be `AoE Debuff`.
    fn try_from(skill: RawSkill) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if skill.type_ != "AoE Debuff" {
            return Err(InvalidField(
                skill.name,
                "type".to_string(),
                Some(skill.type_),
            ));
        }

        Ok(Self {
            name: skill.name.clone(),
            id: skill.id,
            description: skill.description,
            tier: skill.tier,
            mana_cost: skill.mana_cost,
            causes: skill.causes.unwrap_or_default(),
            monsters_use: skill.monsters_use.unwrap_or_default(),
        })
    }
}
