use crate::{
    error::{Error},
    guide::{AdminGuide, OrnaAdminGuide},
    items::admin::AdminItems,
    monsters::admin::AdminMonsters,
    pets::admin::AdminPets,
    skills::admin::AdminSkills,
};

use crate::retry_once;

/// List items from the guide and retrieve them sequentially.
pub fn items(guide: &OrnaAdminGuide) -> Result<AdminItems, Error> {
    Ok(AdminItems {
        items: guide
            .admin_retrieve_items_list()?
            .into_iter()
            .map(|item| retry_once!(guide.admin_retrieve_item_by_id(item.id)))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// List monsters from the guide and retrieve them sequentially.
pub fn monsters(guide: &OrnaAdminGuide) -> Result<AdminMonsters, Error> {
    Ok(AdminMonsters {
        monsters: guide
            .admin_retrieve_monsters_list()?
            .into_iter()
            .map(|monster| retry_once!(guide.admin_retrieve_monster_by_id(monster.id)))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// List skills from the guide and retrieve them sequentially.
pub fn skills(guide: &OrnaAdminGuide) -> Result<AdminSkills, Error> {
    Ok(AdminSkills {
        skills: guide
            .admin_retrieve_skills_list()?
            .into_iter()
            .map(|skill| retry_once!(guide.admin_retrieve_skill_by_id(skill.id)))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// List pets from the guide and retrieve them sequentially.
pub fn pets(guide: &OrnaAdminGuide) -> Result<AdminPets, Error> {
    Ok(AdminPets {
        pets: guide
            .admin_retrieve_pets_list()?
            .into_iter()
            .map(|pet| retry_once!(guide.admin_retrieve_pet_by_id(pet.id)))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}
