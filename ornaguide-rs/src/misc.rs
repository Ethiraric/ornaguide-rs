use crate::{
    error::{Error, Kind},
    guide::Static,
};

/// Remove any added text that may appear in the guide for a name, but not in the game or the
/// codex.
pub(crate) fn sanitize_guide_name(name: &str) -> &str {
    if let Some(pos) = name.find('[') {
        name.split_at(pos - 1).0
    } else {
        name
    }
}
/// Rename effects whose name is different from the codex to the guide.
#[must_use]
pub fn codex_effect_name_to_guide_name(name: &str) -> &str {
    match name {
        "Bloodshift" => "Bloodshift [temp]",
        "Brynhild" => "Call of Brynhild",
        "Darkblight" => "Darkblight [temp]",
        "Dark Immune" => "Dark Immune [temp]",
        "Dark Sigil" => "Dark Sigil [temp]",
        "Defending" => "Defending [Magical]",
        "Dragon Sigil" => "Dragon Sigil [temp]",
        "Drakeblight" => "Drakeblight [temp]",
        "Dumbr" => "Call of Dumbr",
        "Earthblight" => "Earthblight [temp]",
        "Earth Immune" => "Earth Immune [temp]",
        "Earth Sigil" => "Earth Sigil [temp]",
        "Fireblight" => "Fireblight [temp]",
        "Fire Immune" => "Fire Immune [temp]",
        "Fire Sigil" => "Fire Sigil [temp]",
        "Foresight ↑" => "Foresight ↑ [temp]",
        "Foresight ↓" => "Foresight ↓ [temp]",
        "Holyblight" => "Holyblight [temp]",
        "Holy Immune" => "Holy Immune [temp]",
        "Holy Sigil" => "Holy Sigil [temp]",
        "Idun" => "Call of Idun",
        "Jord" => "Call of Jord",
        "Lightningblight" => "Lightningblight [temp]",
        "Lightning Immune" => "Lightning Immune [temp]",
        "Lightning Sigil" => "Lightning Sigil [temp]",
        "Lyon's Mark" => "Lyon's Mark [temp]",
        "Skadi" => "Call of Skadi",
        "Target ↑" => "Target ↑ [temp]",
        "Target ↑↑" => "Target ↑↑ [temp]",
        "Target ↓" => "Target ↓ [temp]",
        "Target ↓↓" => "Target ↓↓ [temp]",
        "Tree of Demise" => "Tree of Demise [temp]",
        "Tree of Life" => "Tree of Life [temp]",
        "Waterblight" => "Waterblight [temp]",
        "Water Immune" => "Water Immune [temp]",
        "Water Sigil" => "Water Sigil [temp]",
        "Windblight" => "Windblight [temp]",
        "Windswept" => "Windswept [temp]",
        _ => name,
    }
}

/// Convert an iterator of codex effects to an iterator of result of guide effect id.
/// On the right side of `Result` is the name of the effect if it wasn't found.
pub fn codex_effect_name_iter_to_guide_id_results<'a, Iter: 'a + Iterator<Item = &'a str>>(
    it: Iter,
    static_: &'a Static,
) -> impl Iterator<Item = Result<u32, String>> + 'a {
    it.map(codex_effect_name_to_guide_name).map(|effect_name| {
        static_
            .status_effects
            .iter()
            .find(|effect| effect.name == *effect_name)
            .map(|effect| effect.id)
            .ok_or_else(|| effect_name.to_string())
    })
}

/// Run the given expression, and retry it once if it returns an `Err`.
/// This macro cannot be called if the given expression moves a variable, as there would be no way
/// of re-trying.
#[macro_export]
macro_rules! retry_once {
    ($expr:expr) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) => $expr,
        }
    };
}

/// A trait to extend `Result<Vec<u32>, Error>`, when it comes from an attempt to convert an array
/// of elements to an array of guide ids.
pub trait VecIdConversionResult {
    /// If the conversion errored because some or all elements could not be converted, return an
    /// `Ok` with the elements that could be converted.
    /// If all elements failed to be converted, return an `Ok(Vec::new())`.
    #[must_use]
    fn ignore_failed_id_conversions(self) -> Self;
}

impl VecIdConversionResult for Result<Vec<u32>, Error> {
    fn ignore_failed_id_conversions(self) -> Self {
        match self {
            Ok(x) => Ok(x),
            Err(Error {
                kind:
                    Kind::PartialCodexStatusEffectsConversion(found, _)
                    | Kind::PartialCodexSkillsConversion(found, _)
                    | Kind::PartialCodexItemDroppedBysConversion(found, _)
                    | Kind::PartialCodexItemUpgradeMaterialsConversion(found, _)
                    | Kind::PartialCodexFollowerAbilitiesConversion(found, _)
                    | Kind::PartialCodexMonsterAbilitiesConversion(found, _)
                    | Kind::PartialCodexEventsConversion(found, _),
                ..
            }) => Ok(found),
            x => x,
        }
    }
}

/// Truncate a string until a given char is encountered.
#[must_use]
pub fn truncate_str_until(s: &str, c: char) -> Option<&str> {
    s.find(c)
        .map(|pos| s.split_at(pos + 1))
        .map(|(_, right)| right)
}

/// Parse a single stat from an entry using `FromStr::parse`.
/// Add meaningful context to the error.
#[macro_export]
macro_rules! parse_stat {
    ($entry:ident, $field:ident, $value:ident) => {
        $entry.$field = $value.parse().map_err(Error::from).map_err(|e| {
            e.ctx_push(format!(
                concat!(
                    "While parsing ",
                    stringify!($field),
                    ": got {} (item name: {})"
                ),
                $value, $entry.name
            ))
        })?
    };
}

/// Parse a single `Option` stat from an entry using `FromStr::parse`.
/// Add meaningful context to the error.
#[macro_export]
macro_rules! parse_stat_opt {
    ($entry:ident, $field:ident, $value:ident) => {
        $entry.$field = if $value.is_empty() {
            None
        } else {
            Some($value.parse().map_err(Error::from).map_err(|e| {
                e.ctx_push(format!(
                    concat!(
                        "While parsing ",
                        stringify!($field),
                        ": got {} (item name: {})"
                    ),
                    $value, $entry.name,
                ))
            })?)
        }
    };
}

/// Parse and push a single stat from an entry using `FromStr::parse`.
/// Add meaningful context to the error.
#[macro_export]
macro_rules! parse_stat_vec {
    ($entry:ident, $field:ident, $value:ident) => {
        $entry
            .$field
            .push($value.parse().map_err(Error::from).map_err(|e| {
                e.ctx_push(format!(
                    concat!(
                        "While parsing ",
                        stringify!($field),
                        ": got {} (item name: {})"
                    ),
                    $value, $entry.name,
                ))
            })?)
    };
}
