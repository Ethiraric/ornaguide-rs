use crate::error::Error;

pub use crate::monsters::raw::{MonsterBuff, MonsterDrop, MonsterQuest, MonsterSkill, RawMonster};

/// A monster in Orna. This enum splits into the different categories of monsters.
#[derive(Clone)]
pub enum Monster {
    /// A regular monster.
    Monster(MonsterMonster),
    /// A boss monster.
    Boss(BossMonster),
    /// A raid monster.
    Raid(RaidMonster),
}

/// A regular monster in Orna.
#[derive(Clone)]
pub struct MonsterMonster {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub image: String,
    pub drops: Option<Vec<MonsterDrop>>,
    pub skills: Option<Vec<MonsterSkill>>,
    pub quests: Option<Vec<MonsterQuest>>,
    pub spawns: Option<Vec<String>>,
    pub resistant_to: Option<Vec<String>>,
    pub weak_to: Option<Vec<String>>,
    pub immune_to: Option<Vec<String>>,
    pub buffs: Option<Vec<MonsterBuff>>,
    pub immune_to_status: Option<Vec<String>>,
    pub vulnerable_to_status: Option<Vec<String>>,
}

impl TryFrom<RawMonster> for Monster {
    type Error = Error;

    fn try_from(raw_monster: RawMonster) -> Result<Self, Self::Error> {
        if !raw_monster.boss {
            Ok(Self::Monster(MonsterMonster::try_from(raw_monster)?))
        } else if !raw_monster.is_raid() {
            Ok(Self::Boss(BossMonster::try_from(raw_monster)?))
        } else {
            Ok(Self::Raid(RaidMonster::try_from(raw_monster)?))
        }
    }
}

impl TryFrom<RawMonster> for MonsterMonster {
    type Error = Error;

    /// Create a `Monster` from a `RawMonster`.
    /// The `RawMonster`'s `boss` field must be `false`.
    fn try_from(monster: RawMonster) -> Result<Self, Self::Error> {
        use Error::InvalidField;

        if monster.boss {
            return Err(InvalidField(
                monster.name,
                "boss".to_string(),
                Some("true".to_string()),
            ));
        }

        Ok(Self {
            name: monster.name,
            id: monster.id,
            tier: monster.tier,
            image: monster.image,
            drops: monster.drops,
            skills: monster.skills,
            quests: monster.quests,
            spawns: monster.spawns,
            resistant_to: monster.resistant_to,
            weak_to: monster.weak_to,
            immune_to: monster.immune_to,
            buffs: monster.buffs,
            immune_to_status: monster.immune_to_status,
            vulnerable_to_status: monster.vulnerable_to_status,
        })
    }
}

/// A boss monster in Orna.
#[derive(Clone)]
pub struct BossMonster {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub image: String,
    pub drops: Option<Vec<MonsterDrop>>,
    pub skills: Vec<MonsterSkill>,
    pub level: u32,
    pub spawns: Option<Vec<String>>,
    pub resistant_to: Option<Vec<String>>,
    pub weak_to: Option<Vec<String>>,
    pub immune_to: Option<Vec<String>>,
    pub buffs: Option<Vec<MonsterBuff>>,
    pub immune_to_status: Option<Vec<String>>,
    pub vulnerable_to_status: Option<Vec<String>>,
}

impl TryFrom<RawMonster> for BossMonster {
    type Error = Error;

    /// Create a `Boss` from a `RawMonster`.
    /// The `RawMonster`'s `boss` field must be `true` and its `spawns` must not contain `Kingdom
    /// Raid` or `World Raid`.
    fn try_from(monster: RawMonster) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if !monster.boss {
            return Err(InvalidField(
                monster.name,
                "boss".to_string(),
                Some("false".to_string()),
            ));
        }
        if monster.is_raid() {
            return Err(InvalidField(
                monster.name,
                "spawns".to_string(),
                Some(format!("{:?}", monster.spawns)),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(monster.name.clone(), field.to_string());

        Ok(Self {
            name: monster.name.clone(),
            id: monster.id,
            tier: monster.tier,
            image: monster.image,
            drops: monster.drops,
            skills: monster.skills.ok_or_else(missing_field("skills"))?,
            level: monster.level.ok_or_else(missing_field("level"))?,
            spawns: monster.spawns,
            resistant_to: monster.resistant_to,
            weak_to: monster.weak_to,
            immune_to: monster.immune_to,
            buffs: monster.buffs,
            immune_to_status: monster.immune_to_status,
            vulnerable_to_status: monster.vulnerable_to_status,
        })
    }
}

/// A raid monster in Orna.
#[derive(Clone)]
pub struct RaidMonster {
    pub name: String,
    pub id: u32,
    pub tier: u32,
    pub image: String,
    pub drops: Vec<MonsterDrop>,
    pub skills: Vec<MonsterSkill>,
    pub level: u32,
    pub spawns: Option<Vec<String>>,
    pub resistant_to: Option<Vec<String>>,
    pub weak_to: Option<Vec<String>>,
    pub immune_to: Option<Vec<String>>,
    pub buffs: Option<Vec<MonsterBuff>>,
    pub immune_to_status: Option<Vec<String>>,
    pub vulnerable_to_status: Option<Vec<String>>,
}

impl TryFrom<RawMonster> for RaidMonster {
    type Error = Error;

    /// Create a `Raid` from a `RawMonster`.
    /// The `RawMonster`'s `boss` field must be `true` and its `spawns` must contain `Kingdom Raid`
    /// or `World Raid`.
    fn try_from(monster: RawMonster) -> Result<Self, Self::Error> {
        use Error::{InvalidField, MissingField};

        if !monster.boss {
            return Err(InvalidField(
                monster.name,
                "boss".to_string(),
                Some("false".to_string()),
            ));
        }
        if !monster.is_raid() {
            return Err(InvalidField(
                monster.name,
                "spawns".to_string(),
                Some(format!("{:?}", monster.spawns)),
            ));
        }

        let missing_field =
            |field: &'static str| || MissingField(monster.name.clone(), field.to_string());

        Ok(Self {
            name: monster.name.clone(),
            id: monster.id,
            tier: monster.tier,
            image: monster.image,
            drops: monster.drops.ok_or_else(missing_field("drops"))?,
            skills: monster.skills.ok_or_else(missing_field("skills"))?,
            level: monster.level.ok_or_else(missing_field("level"))?,
            spawns: monster.spawns,
            resistant_to: monster.resistant_to,
            weak_to: monster.weak_to,
            immune_to: monster.immune_to,
            buffs: monster.buffs,
            immune_to_status: monster.immune_to_status,
            vulnerable_to_status: monster.vulnerable_to_status,
        })
    }
}
