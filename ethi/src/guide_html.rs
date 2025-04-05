use itertools::Itertools;
use ornaguide_rs::{data::OrnaData, monsters::admin::AdminMonster};

#[allow(dead_code, clippy::too_many_lines)]
pub fn monster_to_former_entry_html(monster: &AdminMonster, data: &OrnaData) -> String {
    format!(
        r#"# {} ★{}
<table class="former-entry">
<tr>
    <td>
        <center>
            <img src="/static/orna/img/{}" alt="{}" /></br>
            {}
            {}
            {}
        </center>
    </td>
    <td>
        {}
        {}
    </td>
    <td>
        <b>Abilities</b> </br>
        <ul>
            <li>{}</li>
        </ul>
    </td>
    <td>
        <b>Drops</b> </br>
        <ul>
            <li>{}</li>
        </ul>
    </td>
</tr>
</table>
"#,
        monster.name,
        monster.tier,
        monster.image_name,
        monster.name,
        if monster.level > 0 {
            format!("<b>Level {}</b></br>", monster.level)
        } else {
            String::new()
        },
        if monster.is_world_raid(&data.guide.static_.spawns) {
            "<b>World Raid</b></br>"
        } else {
            ""
        },
        if monster.is_kingdom_raid(&data.guide.static_.spawns) {
            "<b>Kingdom Raid</b></br>"
        } else {
            ""
        },
        if monster.resistant_to.is_empty() {
            String::new()
        } else {
            format!(
                r"
        <b>Resists</b></br>
        <ul>
            <li>{}</li>
        </ul>",
                monster
                    .resistant_to
                    .iter()
                    .map(|id| &data
                        .guide
                        .static_
                        .elements
                        .iter()
                        .find(|element| element.id == *id)
                        .unwrap()
                        .name)
                    .join("</li>\n            <li>")
            )
        },
        if monster.immune_to_status.is_empty() {
            String::new()
        } else {
            format!(
                r"
        <b>Immune to</b></br>
        <ul>
            <li>{}</li>
        </ul>",
                monster
                    .immune_to_status
                    .iter()
                    .map(|id| &data
                        .guide
                        .static_
                        .status_effects
                        .iter()
                        .find(|effect| effect.id == *id)
                        .unwrap()
                        .name)
                    .join("</li>\n            <li>")
            )
        },
        monster
            .skills
            .iter()
            .map(|id| data.guide.skills.find_by_id(*id).unwrap())
            .map(|skill| format!("<a href=\"/skills?show={}\">{}</a>", skill.id, skill.name))
            .join("</li>\n            <li>"),
        monster
            .drops
            .iter()
            .map(|id| data.guide.items.find_by_id(*id).unwrap())
            .map(|item| format!("<a href=\"/items?show={}\">{}</a>", item.id, item.name))
            .join("</li>\n            <li>")
    )
}
