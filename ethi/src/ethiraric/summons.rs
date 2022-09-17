use std::collections::HashMap;

use itertools::Itertools;
use ornaguide_rs::error::Error;
use serde::{Deserialize, Serialize};

use crate::misc::{parse_gdoc_bool, parse_maybe_empty_u8};

/// The number of raids that were summoned in the given record.
/// This structure is flattened in the `Record`, and is only used for clarity.
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Raids {
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_morrigan: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_yggdrasil: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub ashen_phoenix: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub fey_chimera: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub fey_cockatrice: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub fey_dragon: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub fey_gazer: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub fey_yeti: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub final_horseman: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub first_horseman: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub second_horseman: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub third_horseman: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub yggdrasil: u8,

    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_hel: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_judge_achlys: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_judge_charon: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_judge_rhada: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_kerberos: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_morrigan_event: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_naggeneen: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_quetzalcoatl: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_surtr: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub arisen_warlock_trevelyan: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub fallen_judge_achlys: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub fallen_judge_charon: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub fallen_judge_rhada: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub ferocious_cerus: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub kerberos: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub sister_macha: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub the_fool: u8,
    #[serde(deserialize_with = "parse_maybe_empty_u8")]
    pub the_mightiest_mimic: u8,
}

/// A Record in a summon data CSV file.
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Record {
    /// Name of the summoner.
    pub summoner: String,
    /// Tier of the summoner, at the time of summon.
    pub summoner_tier: u8,
    /// Date at which the WRBs were summoned.
    pub summon_date: String,
    /// `true` if the limited event WRBs were excluded from the summon pool.
    #[serde(deserialize_with = "parse_gdoc_bool")]
    pub limited_evt_excluded: bool,
    /// `true` if the summons were made in the beta server.
    #[serde(deserialize_with = "parse_gdoc_bool")]
    pub beta: bool,
    /// Name of the main event, at the time of summoning.
    pub event1: String,
    /// Name of the secondary event, at the time of summoning, if any.
    pub event2: String,
    /// Name of the tertiary event, at the time of summoning, if any.
    pub event3: String,
    /// Number of active events at the time of summoning.
    /// NOTE: There is no record of a tertiary event ever happening yet.
    pub nb_events: u8,
    /// Number of event WRBs summoned.
    pub event_summons: u16,
    /// Number of scrolls used by the summoner.
    pub nb_scrolls: u16,
    /// The raids that were summoned.
    #[serde(flatten)]
    pub raids: Raids,
}

/// Return the number of event raids added to the summon pool by the given event.
fn event_raids_for_event(event: &str) -> u8 {
    match event {
        "" => 0,
        "Cerus the Untamed" => 1,
        "Draconian Era" => 1,
        "Fool of April" => 1,
        "Lyonesse Legends" => 1,
        "Nothren Legends" => 2,
        "Return of Kerberos" => 2,
        "Riftfall" => 4,
        "Sisters of Morrigan" => 2,
        "The Mischievous Clurichauns" => 1,
        "The Mimics Are Loose" => 1,
        _ => panic!("Unknown event: {}", event),
    }
}

/// Parse data from an extract of the summoning data.
/// Compute some statistics.
/// This is a WIP zone.
/// Currently: Determine the probability of summoning an event raid depending on the number of
/// event raids in the pool and the number of active events.
pub fn summons(file: &str) -> Result<(), Error> {
    let mut res: HashMap<(u8, u8), (u16, u16)> = HashMap::new();

    for record in csv::Reader::from_path(file)
        .unwrap()
        .into_deserialize::<Record>()
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| Error::Misc(format!("{}", e)))?
        .into_iter()
        // There is a bunch of lines after the last record. Ignore them.
        .filter(|r| !r.summoner.is_empty())
        // Filter out T7-9 data for now.
        .filter(|r| r.summoner_tier == 10 || r.summoner_tier == 11)
        // We're interested in event data, for now.
        .filter(|r| !r.limited_evt_excluded)
        // This event makes it hard to track event raids.
        .filter(|r| r.event2 != "Sisters of Morrigan")
    {
        let n_raids = event_raids_for_event(&record.event1) + event_raids_for_event(&record.event2);
        let (evt, tot) = res.get(&(record.nb_events, n_raids)).unwrap_or(&(0, 0));
        res.insert(
            (record.nb_events, n_raids),
            (evt + record.event_summons, tot + record.nb_scrolls),
        );
    }

    for ((nb_evt, nb_raids), (evt, tot)) in res.iter().sorted_by_key(|(n, _)| *n) {
        println!(
            "{} events, {} raids: {:4}/{:4} ({:5.2}%)",
            nb_evt,
            nb_raids,
            evt,
            tot,
            (*evt as f32) / (*tot as f32) * 100.0
        )
    }
    Ok(())
}
