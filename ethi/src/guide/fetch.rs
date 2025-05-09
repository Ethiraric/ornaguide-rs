use std::sync::Arc;

use futures::{StreamExt, TryStreamExt};
use ornaguide_rs::{
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    items::admin::{AdminItem, AdminItems},
    monsters::admin::AdminMonsters,
    pets::admin::AdminPets,
    skills::admin::AdminSkills,
};

use crate::{
    misc::{bar, block_on_this_thread},
    retry_once,
};

pub fn items(guide: &OrnaAdminGuide) -> Result<AdminItems, Error> {
    let sleep = crate::config::ornaguide_sleep()? as u64;
    let items = guide.admin_retrieve_items_list()?;
    let bar = Arc::new(bar(items.len() as u64));
    let ret = block_on_this_thread(
        futures::stream::iter(items.into_iter().map(|item| {
            let cloned_bar = bar.clone();
            async move {
                let admin_item = retry_once!(guide.async_admin_retrieve_item_by_id(item.id).await)?;
                cloned_bar.set_message(item.name.clone());
                cloned_bar.inc(1);
                if sleep > 0 {
                    tokio::time::sleep(std::time::Duration::from_secs(sleep)).await;
                }
                Result::<AdminItem, Error>::Ok(admin_item)
            }
        }))
        .buffered(if sleep > 0 { 1 } else { 10 })
        .try_collect::<Vec<AdminItem>>(),
    )?;
    bar.finish_with_message("AItems  fetched");
    Ok(AdminItems { items: ret })
}

pub fn monsters(guide: &OrnaAdminGuide) -> Result<AdminMonsters, Error> {
    let sleep = crate::config::ornaguide_sleep()? as u64;
    let monsters = guide.admin_retrieve_monsters_list()?;
    let mut ret = Vec::with_capacity(monsters.len());
    let bar = bar(monsters.len() as u64);
    for monster in &monsters {
        bar.set_message(monster.name.clone());
        ret.push(retry_once!(guide.admin_retrieve_monster_by_id(monster.id))?);
        bar.inc(1);
        if sleep > 0 {
            std::thread::sleep(std::time::Duration::from_secs(sleep));
        }
    }
    bar.finish_with_message("AMnstrs fetched");
    Ok(AdminMonsters { monsters: ret })
}

pub fn skills(guide: &OrnaAdminGuide) -> Result<AdminSkills, Error> {
    let sleep = crate::config::ornaguide_sleep()? as u64;
    let skills = guide.admin_retrieve_skills_list()?;
    let mut ret = Vec::with_capacity(skills.len());
    let bar = bar(skills.len() as u64);
    for skill in &skills {
        bar.set_message(skill.name.clone());
        ret.push(retry_once!(guide.admin_retrieve_skill_by_id(skill.id))?);
        bar.inc(1);
        if sleep > 0 {
            std::thread::sleep(std::time::Duration::from_secs(sleep));
        }
    }
    bar.finish_with_message("ASkills fetched");
    Ok(AdminSkills { skills: ret })
}

pub fn pets(guide: &OrnaAdminGuide) -> Result<AdminPets, Error> {
    let sleep = crate::config::ornaguide_sleep()? as u64;
    let pets = guide.admin_retrieve_pets_list()?;
    let mut ret = Vec::with_capacity(pets.len());
    let bar = bar(pets.len() as u64);
    for pet in &pets {
        bar.set_message(pet.name.clone());
        ret.push(retry_once!(guide.admin_retrieve_pet_by_id(pet.id))?);
        bar.inc(1);
        if sleep > 0 {
            std::thread::sleep(std::time::Duration::from_secs(sleep));
        }
    }
    bar.finish_with_message("APets   fetched");
    Ok(AdminPets { pets: ret })
}
