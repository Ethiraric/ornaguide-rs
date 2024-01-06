use kuchiki::{parse_html, traits::TendrilSink, NodeRef};

use crate::{
    codex::{CodexSkill, SkillStatusEffect, SkillSummon},
    error::{Error, Kind},
    guide::html_utils::{descend_if_tag, is_html_tag_node, parse_name_and_chance, parse_tags},
    utils::html::{descend_iter, descend_to, node_to_text, parse_icon},
};

/// Parse the tier of the skill.
fn parse_tier(node: &NodeRef) -> Result<u8, Error> {
    let text = node_to_text(node);
    let text = text.trim();
    if let Some(pos) = text.find(':') {
        let (_, tier_with_star) = text.split_at(pos + 1);
        let mut it = tier_with_star.trim().chars();
        it.next(); // Skip over the star.
        Ok(it.as_str().parse()?)
    } else {
        Err(Kind::HTMLParsingError(format!(
            "Failed to find ':' when parsing skill tier: \"{text}\""
        ))
        .into())
    }
}

/// Parse a status effect section.
fn parse_status_effects(status_effects_root: &NodeRef) -> Result<Vec<SkillStatusEffect>, Error> {
    status_effects_root
        // Iterate over all `div`s.
        .following_siblings()
        .filter(is_html_tag_node)
        .map_while(|node| {
            descend_if_tag(&node, "div", |node, _| {
                // Don't attempt to parse what is not a `div`.
                let span = descend_to(node, "span", "skill status effect")?;
                let text = span.text_contents();
                // Split the name of the status effect from its chance.
                parse_name_and_chance(&text, "status effect").map(|(effect, chance)| {
                    SkillStatusEffect {
                        effect: effect.to_string(),
                        chance,
                    }
                })
            })
        })
        // Here, we have an `impl Iterator<Item = Result<SkillStatusEffect, Error>>`.
        // `rustc` is able to collect it directly.
        .collect()
}

/// Parse a summons section.
/// Summons are right after a `Summons:` `h4` tag.
///
/// The next siblings of that `h4` are `div`s that represent one summon. Within those `div`s are
/// other `div`s with the `drop` class. We will call them respectively outer `div`s and inner
/// `div`s.
///
/// The chance of all inner `div`s of an outer `div` share the same %. If we succeed our roll for
/// the outer `div`, we summon one of the inner `div`s monster.
///
/// ```html
/// <div>
///     <div class="drop"> <img></img> <span>Summon (X%)</span></div>
///     <div class="drop"> <img></img> <span>Summon (X%)</span></div>
/// </div>
/// <div>
///     <div class="drop"> <img></img> <span>Summon (X%)</span></div>
///     <div class="drop"> <img></img> <span>Summon (X%)</span></div>
///     <div class="drop"> <img></img> <span>Summon (X%)</span></div>
/// </div>
/// ```
fn parse_summons(summons_root: &NodeRef) -> Result<Vec<Vec<SkillSummon>>, Error> {
    summons_root
        // Iterate over all outer `div`s.
        .following_siblings()
        .filter(is_html_tag_node)
        .map_while(|node| {
            // `descend_if_tag` will return `None` when we hit another `h4`.
            // Keep on while it matches our outer `div`s.
            // The closure returns a `Result<Vec<SkillSummon>, Error>`.
            descend_if_tag(&node, "div", |node, _| {
                // Iterate over all inner `div`s.
                node.children()
                    .filter(is_html_tag_node)
                    .map(|node| -> Result<SkillSummon, Error> {
                        // From our inner `div`, get to the `span` with the text. Split summon name and
                        // chance of summoning.
                        let span = descend_to(&node, "span", "skill summons")?;
                        let text = span.text_contents();
                        parse_name_and_chance(&text, "skill summon").map(|(name, chance)| {
                            SkillSummon {
                                name: name.to_string(),
                                chance,
                            }
                        })
                    })
                    // Collect from our mapped iterator over all inner `div`s.
                    .collect::<Result<Vec<SkillSummon>, Error>>()
            })
        })
        // Here, we have an `impl Iterator<Item = Result<Vec<SkillSummon>, Error>>`.
        // `rustc` is able to collect it directly.
        .collect()
}

/// Parses a skill page from `playorna.com` and returns the details about the given skill.
pub fn parse_html_codex_skill(contents: &str, slug: String) -> Result<CodexSkill, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let description = descend_to(page.as_node(), ".codex-page-description", "page")?;
    let tier = descend_to(page.as_node(), ".codex-page-meta", "page")?;
    let tags = parse_tags(descend_iter(page.as_node(), ".codex-page-tag", "page")?)?;
    let mut causes = vec![];
    let mut gives = vec![];
    let mut summons = vec![];

    for h4 in descend_iter(page.as_node(), "h4", "page")? {
        match h4.text_contents().trim() {
            "Causes:" => {
                causes = parse_status_effects(h4.as_node())?;
            }
            "Gives:" => {
                gives = parse_status_effects(h4.as_node())?;
            }
            "Summons:" => {
                summons = parse_summons(h4.as_node())?;
            }
            x => panic!("{}", x),
        }
    }

    Ok(CodexSkill {
        name: node_to_text(name.as_node()),
        slug,
        icon: parse_icon(icon.as_node())?,
        description: node_to_text(description.as_node()),
        tier: parse_tier(tier.as_node())?,
        tags,
        causes,
        gives,
        summons,
    })
}

/// Parses a skill page from `playorna.com` and returns the details about the given skill.
/// The page needs not be in English and only some of the fields are selected.
/// Fields ignored:
///   - tags
///   - "causes"/"gives": Both are put into `causes`.
pub fn parse_html_codex_skill_translation(
    contents: &str,
    slug: String,
) -> Result<CodexSkill, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let description = descend_to(page.as_node(), ".codex-page-description", "page")?;
    let tier = descend_to(page.as_node(), ".codex-page-meta", "page")?;
    let mut causes = vec![];

    for h4 in descend_iter(page.as_node(), "h4", "page")? {
        causes.append(&mut parse_status_effects(h4.as_node())?);
    }

    Ok(CodexSkill {
        name: node_to_text(name.as_node()),
        slug,
        icon: parse_icon(icon.as_node())?,
        description: node_to_text(description.as_node()),
        tier: parse_tier(tier.as_node())?,
        tags: vec![],
        causes,
        gives: vec![],
        summons: vec![],
    })
}
