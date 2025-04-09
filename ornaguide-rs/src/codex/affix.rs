use serde::{Deserialize, Serialize};

use crate::error::{Error, Kind};

/// Extra effects an item gives.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Affix {
    /// The XP bonus of the item (at common quality).
    ExpBonus(i16),
    /// The gold bonus of the item (at common quality).
    GoldBonus(u8),
    /// The orn bonus of the item (at common quality).
    OrnBonus(u8),
    /// The luck bonus of the item (at common quality).
    LuckBonus(u16),
    /// How much View Distance the item gives (%).
    ViewDistance(u8),
    /// How much stats to the follower the item gives (%).
    FollowerAct(i8),
    /// How much stats to the follower the item gives (%).
    FollowerStats(i8),
    /// How much stats to the summons the item gives (%).
    SummonStats(i8),
    /// How much Monster Attraction the item gives (%).
    MonsterAttraction(u8),
    /// The number of Ward turn at the start of the battle the item grants.
    WardStart(u8),
    /// How much Status Reflection the item gives (%).
    StatusReflection(u8),
    /// How much Beast Taming the item gives (%).
    BeastTaming(u8),
    /// How much Monster Power the item gives (%).
    MonsterPower(u8),
    /// How much Monster Encounters the item gives (%).
    MonsterEncounters(u8),
    /// How much Damage to Ward the item gives (%).
    DamageToWard(u8),
    /// How much Gifts the item gives (%).
    Gifts(u8),
    /// How much Apex the item gives (%).
    Apex(u8),
    /// How much Apex Rate the item gives (%).
    ApexRate(u8),
    /// How much Apex Start the item gives (%).
    ApexStart(u8),
    /// How much Manaflask Charge the item gives (%).
    ManaflaskCharge(u8),
    /// How much Multi-target Damage the item gives (%).
    MultitargetDamage(u8),
    /// How much Chain Damage Chance the item gives (%).
    ChainDamageChance(u8),
    /// How much Buff Duration the item gives (%).
    BuffDuration(u8),
    /// How much Ally Effect Chance the item gives (%).
    AllyEffectChance(u8),
    /// How much Effect Damage the item gives (%).
    EffectDamage(u8),
    /// How much Assassin the item gives (%).
    Assassin(u8),
    /// How much Defend Power the item gives (%).
    DefendPower(u8),
    /// How much Self Damage Reduction the item gives (%).
    ///
    /// Although a reduction, a lower value means more reduction.
    SelfDamageReduction(i8),
    /// How much Collateral Chance the item gives (%).
    CollateralChance(u8),
    /// How much Collateral Damage the item gives (%).
    CollateralDamage(u8),
    /// How much Status Protection the item gives.
    StatusProtection(u8),
    /// How much Def/Res Penetration the item gives (%).
    DefresPenetration(u8),
    /// How much HP Regen the item gives (%).
    HpRegen(u8),
    /// How much Healing the item gives (%).
    Healing(u8),
    /// How much Accuracy the item gives (%).
    Accuracy(u8),
    /// How much Avidity the item gives (tenth of %).
    Avidity(u16),
    /// How much Bestial Bond the item gives (%).
    BestialBond(u8),
    /// How much Critical Chain the item gives (%).
    CriticalChain(u8),
    /// How much Crit Damage the item gives (%).
    CritDamage(i8),
    /// How much Double Handed the item gives (%).
    DoubleHanded(u8),
    /// How much Mana-Ward Recovery the item gives (%).
    ManaWardRecovery(u8),
    /// How much HP-Ward Recovery the item gives (%).
    HpWardRecovery(u8),
    /// How much Summon Pacts the item gives (%).
    SummonPacts(u8),
    /// How much Summon Protection the item gives (%).
    SummonProtection(u8),
    /// How much Mana Reduction the item gives (%).
    ManaReduction(i8),
    /// How much Hybrid Damage the item gives (%).
    HybridDamage(u8),
    /// How much Life Siphon the item gives (%).
    LifeSiphon(u8),
    /// How much Debuff Fade the item gives (%).
    DebuffFade(u8),
    /// How much Faction Damage the item gives (%).
    FactionDamage(u8),
    /// How much Earthen Damage the item gives (%).
    EarthenDamage(i8),
    /// How much Water Damage the item gives (%).
    WaterDamage(i8),
    /// How much Lightning Damage the item gives (%).
    LightningDamage(i8),
    /// How much Fire Damage the item gives (%).
    FireDamage(i8),
    /// How much Arcane Damage the item gives (%).
    ArcaneDamage(i8),
    /// How much Dragon Damage the item gives (%).
    DragonDamage(i8),
    /// How much Dark Damage the item gives (%).
    DarkDamage(i8),
    /// How much Holy Damage the item gives (%).
    HolyDamage(i8),
    /// How much Parapet the item gives (%).
    Parapet(u8),
    /// How much Area Defense the item gives (%).
    AreaDefense(u8),
    /// How much Godforge the item gives (%).
    Godforge(u8),
    /// How much Raid Rewards the item gives (%).
    RaidRewards(u8),
    /// How much Blacksmith Time the item gives (%).
    BlacksmithTime(i8),
    /// How much Line Catches the item gives (%).
    LineCatches(u8),
    /// How much Ult Defense the item gives (%).
    UltDefense(u8),
    /// How much Damage Limit Break the item gives (%).
    DamageLimitBreak(u8),
    /// How much Follower/Summon AI the item gives (%).
    FollowerSummonAI(u8),
    /// How much Ward Power the item gives (%).
    WardPower(u8),
    /// How much Elemental Weaknesses the item gives (%).
    ElementalWeaknesses(u8),
    /// How much Weapon Proficiency the item gives (%).
    WeaponProficiency(u8),
    /// How much No Follower Bonus the item gives (%).
    NoFollowerBonus(u8),
    /// How much Ward Absorption the item gives (%).
    WardAbsorption(u8),
    /// How much Instant Summon the item gives (%).
    InstantSummon(u8),
    /// How many Ward Turns the item gives.
    WardTurns(u8),
    /// How much Mana Regen the item gives (tenth of %).
    ManaRegen(u16),
    /// How much Quest Rewards the item gives (%).
    QuestRewards(u8),
    /// How much Ward Recovery the item gives (%).
    WardRecovery(u8),
    /// How much Turn Reduction the item gives (%).
    TurnReduction(u8),
    /// How much Debuff Duration the item gives (%).
    DebuffDuration(u8),
    /// How much Questing the item gives (%).
    Questing(u8),
    /// How much Dark Res the item gives (%).
    ///
    /// Although a resistance, a lower value means more resistance.
    DarkRes(i8),
    /// How much Holy Res the item gives (%).
    ///
    /// Although a resistance, a lower value means more resistance.
    HolyRes(i8),
    /// How many Summon Turns the item gives.
    SummonTurns(i8),
    /// How much Drop Quality the item gives (%).
    DropQuality(u8),
    /// How much Memory Hunting the item gives (%).
    MemoryHunting(u8),
    /// How much Dungeon Cooldown the item gives (%).
    DungeonCooldown(i8),
}

impl Affix {
    /// Parse the HTML text content of the stat node into an [`Affix`].
    ///
    /// Most affixes look like "xxx: x" or "xxx: x%". `stat` refers to the left-most part of the
    /// `:` (with the `:` included) and `value` to the right part (with the `:` excluded).
    ///
    /// # Errors
    /// Returns an error if the stat is unknown or parsing the value failed.
    #[allow(clippy::too_many_lines)]
    pub fn parse_from_codex_html(stat: &str, value: &str) -> Result<Self, Error> {
        match stat {
            // Boni (scales with quality)
            "EXP Bonus:" => Ok(Self::ExpBonus(value.parse()?)),
            "Gold Bonus:" => Ok(Self::GoldBonus(value.parse()?)),
            "Luck Bonus:" => Ok(Self::LuckBonus(value.parse()?)),
            "Orn Bonus:" => Ok(Self::OrnBonus(value.parse()?)),

            // Boni
            "Blacksmith Time:" => Ok(Self::BlacksmithTime(value.parse()?)),
            "Drop Quality:" => Ok(Self::DropQuality(value.parse()?)),
            "Dungeon Cooldown:" => Ok(Self::DungeonCooldown(value.parse()?)),
            "Gifts:" => Ok(Self::Gifts(value.parse()?)),
            "Godforge:" => Ok(Self::Godforge(value.parse()?)),
            "Line Catches:" => Ok(Self::LineCatches(value.parse()?)),
            "Memory Hunting:" => Ok(Self::MemoryHunting(value.parse()?)),
            "Monster attraction:" => Ok(Self::MonsterAttraction(value.parse()?)),
            "Monster Encounters:" => Ok(Self::MonsterEncounters(value.parse()?)),
            "Monster Power:" => Ok(Self::MonsterPower(value.parse()?)),
            "Questing:" => Ok(Self::Questing(value.parse()?)),
            "Quest Rewards:" => Ok(Self::QuestRewards(value.parse()?)),
            "Raid Rewards:" => Ok(Self::RaidRewards(value.parse()?)),

            // Elemental
            "Arcane Damage:" => Ok(Self::ArcaneDamage(value.parse()?)),
            "Dark Damage:" => Ok(Self::DarkDamage(value.parse()?)),
            "Dragon Damage:" => Ok(Self::DragonDamage(value.parse()?)),
            "Earthen Damage:" => Ok(Self::EarthenDamage(value.parse()?)),
            "Fire Damage:" => Ok(Self::FireDamage(value.parse()?)),
            "Holy Damage:" => Ok(Self::HolyDamage(value.parse()?)),
            "Lightning Damage:" => Ok(Self::LightningDamage(value.parse()?)),
            "Water Damage:" => Ok(Self::WaterDamage(value.parse()?)),

            "Dark Res:" => Ok(Self::DarkRes(value.parse()?)),
            "Holy Res:" => Ok(Self::HolyRes(value.parse()?)),

            // Ward
            "HP-Ward Recovery:" => Ok(Self::HpWardRecovery(value.parse()?)),
            "Mana-Ward Recovery:" => Ok(Self::ManaWardRecovery(value.parse()?)),
            "Ward Absorption:" => Ok(Self::WardAbsorption(value.parse()?)),
            "Ward Power:" => Ok(Self::WardPower(value.parse()?)),
            "Ward Recovery:" => Ok(Self::WardRecovery(value.parse()?)),
            "Ward Start:" => {
                if let Some(n_turns) = value.strip_suffix(" turns") {
                    Ok(Self::WardStart(n_turns.parse()?))
                } else {
                    Err(
                        Kind::HTMLParsingError(format!("Invalid Ward Start value: \"{value}\""))
                            .into(),
                    )
                }
            }
            "Ward Turns:" => Ok(Self::WardTurns(value.parse()?)),

            // Class identity
            "Apex:" => Ok(Self::Apex(value.parse()?)),
            "Apex Rate:" => Ok(Self::ApexRate(value.parse()?)),
            "Apex Start:" => Ok(Self::ApexStart(value.parse()?)),
            "Avidity:" => Ok(Self::Avidity(parse_tenth_percent_value(value)?)),
            "Manaflask Charge:" => Ok(Self::ManaflaskCharge(value.parse()?)),

            // Buffs / Debuffs
            "Assassin:" => Ok(Self::Assassin(value.parse()?)),
            "Buff Duration:" => Ok(Self::BuffDuration(value.parse()?)),
            "Debuff Duration:" => Ok(Self::DebuffDuration(value.parse()?)),
            "Debuff Fade:" => Ok(Self::DebuffFade(value.parse()?)),
            "Effect Damage:" => Ok(Self::EffectDamage(value.parse()?)),
            "Self Damage Reduction:" => Ok(Self::SelfDamageReduction(value.parse()?)),
            "Status Protection:" => Ok(Self::StatusProtection(value.parse()?)),
            "Status Reflection:" => Ok(Self::StatusReflection(value.parse()?)),

            // Follower / Summons
            "Beast Taming:" => Ok(Self::BeastTaming(value.parse()?)),
            "Bestial Bond:" => Ok(Self::BestialBond(value.parse()?)),
            "Follower Act:" => Ok(Self::FollowerAct(value.parse()?)),
            "Follower Stats:" => {
                // Steady Hands of Selene has a value of "-10.0%".
                Ok(Self::FollowerStats(parse_percent_with_maybe_a_dot(value)?))
            }
            "Follower/Summon AI:" => Ok(Self::FollowerSummonAI(value.parse()?)),
            "Instant Summon:" => Ok(Self::InstantSummon(value.parse()?)),
            "No Follower Bonus:" => Ok(Self::NoFollowerBonus(value.parse()?)),
            "Summon Pacts:" => Ok(Self::SummonPacts(value.parse()?)),
            "Summon Protection:" => Ok(Self::StatusProtection(value.parse()?)),
            "Summon Stats:" => Ok(Self::SummonStats(value.parse()?)),
            "Summon Turns:" => Ok(Self::SummonTurns(value.parse()?)),

            // Stat buffs
            "Accuracy:" => Ok(Self::Accuracy(value.parse()?)),
            "Area Defense:" => Ok(Self::AreaDefense(value.parse()?)),
            "Crit damage:" => Ok(Self::CritDamage(value.parse()?)),
            "Double Handed:" => Ok(Self::DoubleHanded(value.parse()?)),
            "Hybrid Damage:" => Ok(Self::HybridDamage(value.parse()?)),
            "Weapon Proficiency:" => Ok(Self::WeaponProficiency(value.parse()?)),

            // Multi-target and damage improvement
            "Chain Damage Chance:" => Ok(Self::ChainDamageChance(value.parse()?)),
            "Collateral Chance:" => Ok(Self::CollateralChance(value.parse()?)),
            "Collateral Damage:" => Ok(Self::CollateralDamage(value.parse()?)),
            "Damage Limit Break:" => Ok(Self::DamageLimitBreak(value.parse()?)),
            "Damage to Ward:" => Ok(Self::DamageToWard(value.parse()?)),
            "Def/Res Penetration:" => Ok(Self::DefresPenetration(value.parse()?)),
            "Elemental Weaknesses:" => Ok(Self::ElementalWeaknesses(value.parse()?)),
            "Faction Damage:" => Ok(Self::FactionDamage(value.parse()?)),
            "Multi-target Damage:" => Ok(Self::MultitargetDamage(value.parse()?)),

            // Defensive and regen
            "Defend Power:" => Ok(Self::DefendPower(value.parse()?)),
            "Healing:" => Ok(Self::Healing(value.parse()?)),
            "HP Regen:" => Ok(Self::HpRegen(value.parse()?)),
            "Life Siphon:" => Ok(Self::LifeSiphon(value.parse()?)),
            "Mana Reduction:" => {
                // Fallen Sky Shoes has a value of "-50.0%".
                Ok(Self::ManaReduction(parse_percent_with_maybe_a_dot(value)?))
            }
            "Mana Regen:" => Ok(Self::ManaRegen(parse_tenth_percent_value(value)?)),
            "Parapet:" => Ok(Self::Parapet(value.parse()?)),
            "Turn Reduction:" => Ok(Self::TurnReduction(value.parse()?)),
            "Ult Defense:" => Ok(Self::UltDefense(value.parse()?)),

            "Ally Effect Chance:" => Ok(Self::AllyEffectChance(value.parse()?)),
            "Critical Chain:" => Ok(Self::CriticalChain(value.parse()?)),
            "View distance:" | "View Distance:" => Ok(Self::ViewDistance(value.parse()?)),

            _ => Err(
                Kind::HTMLParsingError(format!("Failed to parse affix: \"{stat}:{value}\"")).into(),
            ),
        }
    }
}

/// Parse a % value and return it in tenth of % (e.g.: `"1%"` -> 10, `"0.5%"` -> 5).
fn parse_tenth_percent_value(text: &str) -> Result<u16, Error> {
    if let Some(dot_pos) = text.find('.') {
        if dot_pos == text.len() - 2 {
            let text = text.trim_start_matches('+');
            let mut res = 0u16;
            for c in text.chars().filter(|c| *c != '.') {
                if let Some(digit) = c.to_digit(10) {
                    res = res * 10 + u16::try_from(digit).unwrap();
                } else {
                    return Err(Kind::HTMLParsingError(format!(
                        "Invalid 10th-percent value: \"{text}\""
                    ))
                    .into());
                }
            }
            Ok(res)
        } else {
            Err(Kind::HTMLParsingError(format!("Invalid 10th-percent value: \"{text}\"")).into())
        }
    } else {
        Ok(text.parse::<u16>()? * 10u16)
    }
}

/// Parse a percent value which may contain a `.`.
fn parse_percent_with_maybe_a_dot(value: &str) -> Result<i8, Error> {
    let neg = value.starts_with('-');
    let value = value.trim_start_matches('-');
    let v = parse_tenth_percent_value(value)?;
    assert!(v % 10 == 0, "Value should be a raw %");
    let v = i8::try_from(v / 10).unwrap();
    Ok(if neg { -v } else { v })
}
