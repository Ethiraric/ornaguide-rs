use crate::guide::Static;

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
pub(crate) fn codex_effect_name_to_guide_name(name: &str) -> &str {
    match name {
        "Bloodshift" => "Bloodshift [temp]",
        "Dark Immune" => "Dark Immune [temp]",
        "Earth Immune" => "Earth Immune [temp]",
        "Fire Immune" => "Fire Immune [temp]",
        "Foresight ↑" => "Foresight ↑ [temp]",
        "Foresight ↓" => "Foresight ↓ [temp]",
        "Holy Immune" => "Holy Immune [temp]",
        "Lightning Immune" => "Lightning Immune [temp]",
        "Lyon's Mark" => "Lyon's Mark [temp]",
        "Target ↑" => "Target ↑ [temp]",
        "Target ↑↑" => "Target ↑↑ [temp]",
        "Target ↓" => "Target ↓ [temp]",
        "Target ↓↓" => "Target ↓↓ [temp]",
        "Tree of Demise" => "Tree of Demise [temp]",
        "Tree of Life" => "Tree of Life [temp]",
        "Water Immune" => "Water Immune [temp]",
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
