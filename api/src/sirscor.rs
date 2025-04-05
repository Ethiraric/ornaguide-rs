use std::fmt::Write;

use ornaguide_rs::{
    data::OrnaData, items::admin::AdminItem, monsters::admin::AdminMonster, pets::admin::AdminPet,
    skills::admin::AdminSkill,
};

use crate::{
    data::DATA,
    rocket_utils::{entity_to_li, make_list, Html, STYLE},
};

/// Write an `li` HTML tag for the given item to the given string.
fn item_to_li(item: &AdminItem, response: &mut String) -> Result<(), std::fmt::Error> {
    entity_to_li("item", item.id, &item.name, response)
}

/// Write an `li` HTML tag for the given skill to the given string.
fn skill_to_li(skill: &AdminSkill, response: &mut String) -> Result<(), std::fmt::Error> {
    entity_to_li("skill", skill.id, &skill.name, response)
}

/// Write an `li` HTML tag for the given monster to the given string.
fn monster_to_li(monster: &AdminMonster, response: &mut String) -> Result<(), std::fmt::Error> {
    entity_to_li("monster", monster.id, &monster.name, response)
}

/// Write an `li` HTML tag for the given pet to the given string.
fn pet_to_li(pet: &AdminPet, response: &mut String) -> Result<(), std::fmt::Error> {
    entity_to_li("pet", pet.id, &pet.name, response)
}

/// Look for items that have no `equipped_by`. Add an HTML list of them in `response`.
fn check_item_empty_equipped_by(
    data: &OrnaData,
    response: &mut String,
) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide
            .items
            .items
            .iter()
            .filter(|item| item.equipped_by.is_empty()),
        "Missing <pre>equipped_by</pre>",
        item_to_li,
        response,
    )
}

/// Look for items that have no `rarity`. Add an HTML list of them in `response`.
fn check_item_missing_rarity(
    data: &OrnaData,
    response: &mut String,
) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide
            .items
            .items
            .iter()
            .filter(|item| item.rarity == "NO"),
        "Missing <pre>rarity</pre>",
        item_to_li,
        response,
    )
}

/// Look for items that have a `type` set to TBD. Add an HTML list of them in `response`.
fn check_item_tbd_type(data: &OrnaData, response: &mut String) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide
            .items
            .items
            .iter()
            .filter(|item| item.type_ == 13), // TBD
        "Missing <pre>type</pre> (TBD)",
        item_to_li,
        response,
    )
}

/// Look for weapons that have no category. Add an HTML list of them in `response`.
fn check_item_missing_category(
    data: &OrnaData,
    response: &mut String,
) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide.items.items.iter().filter(|item| {
            item.type_ == 2 // Weapon
               && item.category.is_none()
        }),
        "Missing <pre>category</pre>",
        item_to_li,
        response,
    )
}

/// Look for items that are bought but have no price. Add an HTML list of them in `response`.
fn check_item_missing_price(data: &OrnaData, response: &mut String) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide.items.items.iter().filter(|item| {
            !item.codex_uri.is_empty()
                && data
                    .codex
                    .items
                    .find_by_uri(&item.codex_uri).is_some_and(ornaguide_rs::codex::CodexItem::found_in_shops)
                && item.price == 0
        }),
        "Missing <pre>price</pre>",
        item_to_li,
        response,
    )
}

/// Look for skills that have a `type` set to TBD. Add an HTML list of them in `response`.
fn check_skill_tbd_type(data: &OrnaData, response: &mut String) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide
            .skills
            .skills
            .iter()
            .filter(|skill| skill.type_ == 16), // TBD
        "Missing <pre>type</pre> (TBD)",
        skill_to_li,
        response,
    )
}

/// Look for skills that are bought from the arcanist but have no price. Add an HTML list of them
/// in `response`.
fn check_skill_missing_price(
    data: &OrnaData,
    response: &mut String,
) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide
            .skills
            .skills
            .iter()
            .filter(|skill| skill.bought && skill.cost == 0),
        "Missing <pre>price</pre>",
        skill_to_li,
        response,
    )
}

/// Look for bosses / raid bosses that have no HP value. Add an HTML list of them in `response`.
fn check_monster_missing_hp(data: &OrnaData, response: &mut String) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide
            .monsters
            .monsters
            .iter()
            .filter(|monster| monster.is_raid(&data.guide.static_.spawns) && monster.hp == 0),
        "Missing <pre>hp</pre>",
        monster_to_li,
        response,
    )
}

/// Look for pets that have no cost. Add an HTML list of them in `response`.
fn check_pet_missing_price(data: &OrnaData, response: &mut String) -> Result<(), std::fmt::Error> {
    make_list(
        data.guide.pets.pets.iter().filter(|pet| pet.cost == 0),
        "Missing <pre>cost</pre>",
        pet_to_li,
        response,
    )
}

#[get("/sirscor")]
pub fn get() -> Html<String> {
    let data = match DATA.as_ref() {
        Ok(x) => x,
        Err(x) => {
            return format!("Error: {x}").into();
        }
    };

    let mut response = format!("<html>{STYLE}<body>");

    Ok(())
        .and_then(|()| writeln!(&mut response, "<h1>Items</h1>"))
        .and_then(|()| check_item_empty_equipped_by(data, &mut response))
        .and_then(|()| check_item_missing_rarity(data, &mut response))
        .and_then(|()| check_item_tbd_type(data, &mut response))
        .and_then(|()| check_item_missing_category(data, &mut response))
        .and_then(|()| check_item_missing_price(data, &mut response))
        .and_then(|()| writeln!(&mut response, "<hr/><h1>Skills</h1>"))
        .and_then(|()| check_skill_tbd_type(data, &mut response))
        .and_then(|()| check_skill_missing_price(data, &mut response))
        .and_then(|()| writeln!(&mut response, "<hr/><h1>Monsters</h1>"))
        .and_then(|()| check_monster_missing_hp(data, &mut response))
        .and_then(|()| writeln!(&mut response, "<hr/><h1>Pets</h1>"))
        .and_then(|()| check_pet_missing_price(data, &mut response))
        .and_then(|()| writeln!(&mut response, "</body></html>"))
        .map_or_else(|err| format!("Error: {err}"), move |()| response)
        .into()
}
