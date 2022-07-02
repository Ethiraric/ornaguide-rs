use crate::{error::Error, guide::Static};

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
pub(crate) fn codex_effect_name_iter_to_guide_id_results<
    'a,
    Iter: 'a + Iterator<Item = &'a str>,
>(
    it: Iter,
    static_: &'a Static,
) -> impl Iterator<Item = Result<u32, Error>> + 'a {
    it.map(codex_effect_name_to_guide_name).map(|effect_name| {
        static_
            .status_effects
            .iter()
            .find(|effect| effect.name == *effect_name)
            .map(|effect| effect.id)
            .ok_or_else(|| {
                Error::Misc(format!(
                    "Failed to find a status effect for codex status_effect {}",
                    effect_name
                ))
            })
    })
}
