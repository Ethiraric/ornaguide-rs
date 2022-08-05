use std::{
    fs::OpenOptions,
    io::{BufWriter, Write},
};

use ornaguide_rs::{data::OrnaData, error::Error};

#[allow(dead_code)]
pub fn items_to_csv(data: OrnaData) -> Result<(), Error> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("../thecraigger.csv")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer,"name,tier,type,image_name,hp,hp_affected_by_quality,mana,mana_affected_by_quality,attack,attack_affected_by_quality,magic,magic_affected_by_quality,defense,defense_affected_by_quality,resistance,resistance_affected_by_quality,dexterity,dexterity_affected_by_quality,ward,ward_affected_by_quality,crit,crit_affected_by_quality,foresight,view_distance,follower_stats,follower_act,status_infliction,status_protection,mana_saver,base_adornment_slots,rarity,element,two_handed,orn_bonus,gold_bonus,drop_bonus,exp_bonus,boss,arena,ability")?;
    for item in data.guide.items.items.iter() {
        writeln!(
            writer,
            r#""{}",{},"{}","{}",{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},"{}","{}",{},{},{},{},{},{},{},"{}""#,
            item.name,
            item.tier,
            data.guide
                .static_
                .item_types
                .iter()
                .find(|t| t.id == item.type_)
                .unwrap()
                .name,
            item.image_name,
            item.hp,
            item.hp_affected_by_quality,
            item.mana,
            item.mana_affected_by_quality,
            item.attack,
            item.attack_affected_by_quality,
            item.magic,
            item.magic_affected_by_quality,
            item.defense,
            item.defense_affected_by_quality,
            item.resistance,
            item.resistance_affected_by_quality,
            item.dexterity,
            item.dexterity_affected_by_quality,
            item.ward,
            item.ward_affected_by_quality,
            item.crit,
            item.crit_affected_by_quality,
            item.foresight,
            item.view_distance,
            item.follower_stats,
            item.follower_act,
            item.status_infliction,
            item.status_protection,
            item.mana_saver,
            item.base_adornment_slots,
            item.rarity,
            item.element
                .map(|id| data
                    .guide
                    .static_
                    .elements
                    .iter()
                    .find(|el| el.id == id)
                    .unwrap()
                    .name
                    .as_str())
                .unwrap_or(""),
            item.two_handed,
            item.orn_bonus,
            item.gold_bonus,
            item.drop_bonus,
            item.exp_bonus,
            item.boss,
            item.arena,
            item.ability
                .map(|id| data
                    .guide
                    .skills
                    .skills
                    .iter()
                    .find(|sk| sk.id == id)
                    .unwrap()
                    .name
                    .as_str())
                .unwrap_or("")
        )?;
    }
    Ok(())
}
