use std::io::BufWriter;

use kuchiki::{parse_html, traits::TendrilSink, ElementData, NodeData, NodeRef};

use crate::{
    codex::{
        affix::Affix,
        item::{
            Ability, Cause, Cure, DroppedBy, Element, Give, Immunity, Item, Place, Stats,
            UpgradeMaterial,
        },
    },
    error::{Error, Kind},
    guide::html_utils::parse_tags,
    utils::html::{
        descend_iter, descend_to, get_attribute_from_node, icon_url_to_path, node_to_text,
        parse_icon, try_descend_to,
    },
};

/// Aggregation of all fields that are tagged with the `codex-page-meta` class.
/// This is kind of a dumpster class where all kinds of fields are put in.
struct CodexMeta {
    /// Whether the item is exotic.
    exotic: bool,
    /// The tier of the item.
    tier: u8,
    /// The rarity of the item, if any.
    rarity: Option<String>,
    /// Which classes may use the item.
    useable_by: Option<String>,
    /// Equipment slot on which the item is equipped.
    place: Option<Place>,
    /// The type of a weapon.
    weapon_type: Option<String>,
}

impl Default for CodexMeta {
    fn default() -> Self {
        Self {
            exotic: false,
            tier: 1,
            rarity: None,
            useable_by: None,
            place: None,
            weapon_type: None,
        }
    }
}

/// Parse the tier of the skill.
fn parse_tier(text: &str) -> Result<u8, Error> {
    if let Some(pos) = text.find(':') {
        let (_, tier_with_star) = text.split_at(pos + 1);
        let mut it = tier_with_star.trim().chars();
        it.next(); // Skip over the star.
        Ok(it.as_str().parse()?)
    } else {
        Err(Kind::HTMLParsingError(format!(
            "Failed to find ':' when parsing item tier: \"{text}\""
        ))
        .into())
    }
}

/// Parse all `codex-page-meta` nodes and fill a `CodexMeta` struct.
fn parse_codex_page_meta(page: &NodeRef) -> Result<CodexMeta, Error> {
    let mut ret = CodexMeta::default();
    for meta_node in descend_iter(page, ".codex-page-meta", "page")? {
        // First, check if the node is an `exotic` node.
        if let Ok(Some(exotic_node)) =
            try_descend_to(meta_node.as_node(), ".exotic", "codex-page-meta")
        {
            let contents = exotic_node.as_node().text_contents();
            let contents = contents.trim();
            if contents == "Exotic" {
                ret.exotic = true;
            } else {
                return Err(Kind::HTMLParsingError(format!(
                    "Invalid exotic node contents: {contents}"
                ))
                .into());
            }
        } else {
            let contents = meta_node.text_contents();
            let contents = contents.trim();
            // If not, it may be a Tier node.
            if contents.starts_with("Tier:") {
                ret.tier = parse_tier(contents)?;
            }
            // If not, it may be a Rarity node.
            else if let Some(rarity) = contents.strip_prefix("Rarity:") {
                // TODO(ethiraric, 14/11/2022): Make it an enum.
                ret.rarity = Some(rarity.trim().to_string());
            }
            // If not, it may be a Useable By node.
            else if let Some(useable_by) = contents.strip_prefix("Useable by:") {
                // TODO(ethiraric, 14/11/2022): Make it a `Vec<Enum>`.
                ret.useable_by = Some(useable_by.trim().to_string());
            }
            // If not, it may be a Place node.
            else if let Some(place) = contents.strip_prefix("Place:") {
                ret.place = Some(place.trim().parse()?);
            }
            // If not, it may be a Type node.
            else if let Some(weapon_type) = contents.strip_prefix("Type:") {
                // TODO(ethiraric, 06/04/2025): Make it an enum.
                ret.weapon_type = Some(weapon_type.trim().to_string());
            } else {
                let mut buf = BufWriter::new(Vec::new());
                meta_node.as_node().serialize(&mut buf)?;
                return Err(Kind::HTMLParsingError(format!(
                    "Unknown codex-page-meta: {}",
                    String::from_utf8(buf.into_inner()?)?
                ))
                .into());
            }
        }
    }
    Ok(ret)
}

/// Parse a `<a>` node to a `name`, `uri`, `icon` tuple.
fn a_to_name_uri_icon(a: &NodeRef) -> Result<(String, String, String), Error> {
    let uri = get_attribute_from_node(a, "href", "item <a>")?;
    let img = descend_to(a, "img", "item <a>")?;
    let icon = icon_url_to_path(&get_attribute_from_node(
        img.as_node(),
        "src",
        "item <a> img",
    )?);
    let name = node_to_text(a);
    Ok((name, uri, icon))
}

/// Parse a `<div>` node to a `name`, `icon` tuple.
fn div_to_name_icon(div: &NodeRef) -> Result<(String, String), Error> {
    let img = descend_to(div, "img", "item <a>")?;
    let icon = icon_url_to_path(&get_attribute_from_node(
        img.as_node(),
        "src",
        "item <div> img",
    )?);
    let name = node_to_text(div);
    Ok((name, icon))
}

/// Parse a list of `name`, `uri`, `icon` tuples.
fn parse_name_uri_icon_list(
    iter_node: &NodeRef,
) -> impl Iterator<Item = Result<(String, String, String), Error>> {
    iter_node
        .following_siblings()
        .filter(|node| matches!(node.data(), NodeData::Element(_)))
        .map_while(|node| {
            if let NodeData::Element(ElementData {
                name,
                attributes: _attributes,
                template_contents: _,
            }) = node.data()
            {
                let tag = name.local.to_string();
                match &*tag {
                    "h4" | "hr" => None,
                    "div" => Some(
                        descend_to(&node, "a", "div drop or ability")
                            .and_then(|node| a_to_name_uri_icon(node.as_node())),
                    ),
                    _ => Some(Err(Kind::HTMLParsingError(format!(
                        "Unknown node tag when parsing drop or ability: {}",
                        &tag
                    ))
                    .into())),
                }
            } else {
                panic!("Cannot happen due to previous filter");
            }
        })
}

/// Parse a list of `name`, `icon` tuples.
fn parse_name_icon_list(
    iter_node: &NodeRef,
) -> impl Iterator<Item = Result<(String, String), Error>> {
    iter_node
        .following_siblings()
        .filter(|node| matches!(node.data(), NodeData::Element(_)))
        .map_while(|node| {
            if let NodeData::Element(ElementData {
                name,
                attributes: _attributes,
                template_contents: _,
            }) = node.data()
            {
                let tag = name.local.to_string();
                match &*tag {
                    "h4" | "hr" => None,
                    "div" => Some(div_to_name_icon(&node)),
                    _ => Some(Err(Kind::HTMLParsingError(format!(
                        "Unknown node tag when parsing drop or ability: {}",
                        &tag
                    ))
                    .into())),
                }
            } else {
                panic!("Cannot happen due to previous filter");
            }
        })
}

/// Parse a stat of the item.
///
/// Takes as input the "xxx: x%" string.
fn parse_stat(text: &str, stats: &mut Stats) -> Result<(), Error> {
    if let Some(pos) = text.find(':') {
        let (stat, value) = text.split_at(pos + 1);
        let stat = stat.trim();
        let value = value.trim().trim_end_matches('%').trim_start_matches('+');
        match stat {
            // Base stats
            "Attack:" => stats.attack = Some(value.parse()?),
            "Magic:" => stats.magic = Some(value.parse()?),
            "Defense:" => stats.defense = Some(value.parse()?),
            "Resistance:" => stats.resistance = Some(value.parse()?),
            "HP:" => stats.hp = Some(value.parse()?),
            "Mana:" => stats.mana = Some(value.parse()?),
            "Ward:" => stats.ward = Some(value.parse()?),
            "Dexterity:" => stats.dexterity = Some(value.parse()?),
            "Crit:" => stats.crit = Some(value.parse()?),
            "Foresight:" => stats.foresight = Some(value.parse()?),
            // Weird things that we may consider base stats?
            "Adornment Slots:" => stats.adornment_slots = Some(value.parse()?),
            stat => {
                stats
                    .affixes
                    .push(Affix::parse_from_codex_html(stat, value)?);
            }
        }
    } else if let Some(skill_name) = text.strip_prefix('+') {
        stats.skills_granted.push(skill_name.to_string());
    } else {
        match text {
            "Fire" => stats.element = Some(Element::Fire),
            "Water" => stats.element = Some(Element::Water),
            "Earthen" => stats.element = Some(Element::Earthen),
            "Lightning" => stats.element = Some(Element::Lightning),
            "Holy" => stats.element = Some(Element::Holy),
            "Dark" => stats.element = Some(Element::Dark),
            "Arcane" => stats.element = Some(Element::Arcane),
            "Dragon" => stats.element = Some(Element::Dragon),
            "Physical" => stats.element = Some(Element::Physical),
            "Two handed" => stats.two_handed = true,
            _ => {
                return Err(Kind::HTMLParsingError(format!(
                    "Failed to find ':' when parsing stat: \"{text}\""
                ))
                .into());
            }
        }
    }
    Ok(())
}

/// Parse the stats of the item.
///
/// This function goes through the page in search of the `codex-stat` tag and parses each of them
/// as a stat of the item.
fn parse_stats(node: Option<&NodeRef>) -> Result<Option<Stats>, Error> {
    if let Some(node) = node {
        let mut stats = Stats::default();
        for node in descend_iter(node, ".codex-stat", "codex stats node")? {
            let text = node_to_text(node.as_node());
            let text = text.trim();

            // Arisen Colada has a single `codex-stat` tag with "Collateral Chance: +2% /
            // Collateral Damage: +20%". We must however NOT split by '/', as one of the affixes is
            // "Def/Res Penetration" and we don't want to split that '/'.
            for stat in text.split(" / ") {
                parse_stat(stat, &mut stats)?;
            }
        }
        Ok(Some(stats))
    } else {
        Ok(None)
    }
}

/// From a string `Status (x%)`, return a tuple `("Status", x)`.
fn split_status_chance(text: &str) -> Result<(String, i8), Error> {
    if let Some(pos) = text.find('(') {
        let (status, chance) = text.split_at(pos);
        Ok((
            status.trim().to_string(),
            chance
                .trim()
                .trim_start_matches('(')
                .trim_end_matches(')')
                .trim_end_matches('%')
                .parse()?,
        ))
    } else {
        Err(Kind::HTMLParsingError(format!(
            "Failed to find '(' when parsing status effect: \"{text}\""
        ))
        .into())
    }
}

/// Parse causes from the `h4` abilities node.
fn parse_causes(iter_node: &NodeRef) -> Result<Vec<Cause>, Error> {
    parse_name_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, icon)| Cause { name, icon }))
        .collect()
}

/// Parse gives from the `h4` abilities node.
fn parse_gives(iter_node: &NodeRef) -> Result<Vec<Give>, Error> {
    parse_name_icon_list(iter_node)
        .map(|tupleresult| {
            tupleresult.and_then(|(text, icon)| {
                split_status_chance(&text).map(|(name, chance)| Give { name, chance, icon })
            })
        })
        .collect()
}

/// Parse cures from the `h4` abilities node.
fn parse_cures(iter_node: &NodeRef) -> Result<Vec<Cure>, Error> {
    parse_name_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, icon)| Cure { name, icon }))
        .collect()
}

/// Parse immunitiees from the `h4` abilities node.
fn parse_immunities(iter_node: &NodeRef) -> Result<Vec<Immunity>, Error> {
    parse_name_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, icon)| Immunity { name, icon }))
        .collect()
}

/// Parse dropped by from the `h4` abilities node.
fn parse_dropped_by(iter_node: &NodeRef) -> Result<Vec<DroppedBy>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| DroppedBy { name, uri, icon }))
        .collect()
}

/// Parse drops the `h4` drops node.
fn parse_upgrade_materials(iter_node: &NodeRef) -> Result<Vec<UpgradeMaterial>, Error> {
    parse_name_uri_icon_list(iter_node)
        .map(|tupleresult| tupleresult.map(|(name, uri, icon)| UpgradeMaterial { name, uri, icon }))
        .collect()
}

fn parse_ability(node: Option<&NodeRef>) -> Result<Option<Ability>, Error> {
    if let Some(node) = node {
        if let Some(previous) = node.previous_sibling().and_then(|n| n.previous_sibling()) {
            let text = node_to_text(&previous);
            let text = text.trim();
            if let Some(pos) = text.find(':') {
                let (left, right) = text.split_at(pos + 1);
                let left = left.trim();
                let right = right.trim();
                if left == "Ability:" {
                    Ok(Some(Ability {
                        name: right.to_string(),
                        description: node_to_text(node),
                    }))
                } else {
                    Err(Kind::HTMLParsingError(format!(
                        "Failed to find 'Ability:' when parsing: \"{text}\""
                    ))
                    .into())
                }
            } else {
                Err(Kind::HTMLParsingError(format!(
                    "Failed to find ':' when parsing ability name: \"{text}\""
                ))
                .into())
            }
        } else {
            Err(Kind::HTMLParsingError(
                "Failed to find previous node when parsing ability".to_string(),
            )
            .into())
        }
    } else {
        Ok(None)
    }
}

/// Parses an item page from `playorna.com` and returns the details about the given item.
pub fn parse_html_codex_item(contents: &str, slug: &str) -> Result<Item, Error> {
    parse_html_codex_item_impl(contents, slug.into())
        .map_err(|e| e.ctx_push(format!("parsing {slug}")))
}

/// Parses an item page from `playorna.com` and returns the details about the given item.
fn parse_html_codex_item_impl(contents: &str, slug: String) -> Result<Item, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let mut description_it = descend_iter(page.as_node(), ".codex-page-description", "page")?;
    let codex_page_meta = parse_codex_page_meta(page.as_node())?;
    let stats_parent = try_descend_to(page.as_node(), ".codex-stats", "page")?;

    let mut causes = vec![];
    let mut cures = vec![];
    let mut gives = vec![];
    let mut immunities = vec![];
    let mut dropped_by = vec![];
    let mut upgrade_materials = vec![];

    let tags = parse_tags(descend_iter(page.as_node(), ".codex-page-tag", "page")?)?;

    let description = if let Some(description) = description_it.next() {
        node_to_text(description.as_node())
    } else {
        return Err(Kind::HTMLParsingError("Failed to find description".to_string()).into());
    };

    // Parse stats.
    let mut stats = parse_stats(stats_parent.as_ref().map(kuchiki::NodeDataRef::as_node))?;
    // Though `place` is in the `codex-page-meta` section, it is in the `stat` structure.
    if let Some(place) = codex_page_meta.place {
        if let Some(stats) = stats.as_mut() {
            stats.place = Some(place);
        } else {
            stats = Some(Stats {
                place: Some(place),
                ..Default::default()
            });
        }
    }

    for h4 in descend_iter(page.as_node(), "h4", "page")? {
        match h4.text_contents().trim() {
            "Causes:" => {
                causes = parse_causes(h4.as_node())?;
            }
            "Gives:" => {
                gives = parse_gives(h4.as_node())?;
            }
            "Cures:" => {
                cures = parse_cures(h4.as_node())?;
            }
            "Immunities:" => {
                immunities = parse_immunities(h4.as_node())?;
            }
            "Dropped by:" => {
                dropped_by = parse_dropped_by(h4.as_node())?;
            }
            "Upgrade materials:" => {
                upgrade_materials = parse_upgrade_materials(h4.as_node())?;
            }
            x => return Err(Kind::HTMLParsingError(format!("Unexpected h4: \"{x}\"")).into()),
        }
    }

    Ok(Item {
        slug,
        name: node_to_text(name.as_node()),
        icon: parse_icon(icon.as_node())?,
        description,
        tier: codex_page_meta.tier,
        stats,
        ability: parse_ability(
            description_it
                .next()
                .as_ref()
                .map(kuchiki::NodeDataRef::as_node),
        )?,
        causes,
        cures,
        gives,
        immunities,
        dropped_by,
        upgrade_materials,
        tags,
    })
}

/// Parses an item page from `playorna.com` and returns the details about the given item.
/// The page needs not be in English and only some of the fields are selected.
/// Fields ignored:
///   - tier
///   - stats
///   - causes
///   - cures
///   - gives
///   - immunities
///   - dropped_by
///   - upgrade_materials
///   - tags
///   - ability
#[allow(clippy::doc_markdown)]
pub fn parse_html_codex_item_translation(contents: &str, slug: String) -> Result<Item, Error> {
    let html = parse_html().one(contents);

    let name = descend_to(&html, ".herotext", "html")?;
    let page = descend_to(&html, ".codex-page", "html")?;
    let icon = descend_to(page.as_node(), ".codex-page-icon", "page")?;
    let mut description_it = descend_iter(page.as_node(), ".codex-page-description", "page")?;

    let description = if let Some(description) = description_it.next() {
        node_to_text(description.as_node())
    } else {
        return Err(Kind::HTMLParsingError("Failed to find description".to_string()).into());
    };

    Ok(Item {
        slug,
        name: node_to_text(name.as_node()),
        icon: parse_icon(icon.as_node())?,
        description,
        tier: 0,
        stats: None,
        ability: None,
        causes: vec![],
        cures: vec![],
        gives: vec![],
        immunities: vec![],
        dropped_by: vec![],
        upgrade_materials: vec![],
        tags: vec![],
    })
}
