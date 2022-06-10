/// Remove any added text that may appear in the guide for a name, but not in the game or the
/// codex.
pub(crate) fn sanitize_guide_name(name: &str) -> &str {
    if let Some(pos) = name.find('[') {
        name.split_at(pos - 1).0
    } else {
        name
    }
}
