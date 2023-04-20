use crate::{
    codex::{
        Codex, CodexBosses, CodexFollowers, CodexItems, CodexMonsters, CodexRaids, CodexSkills,
        Sluggable,
    },
    error::{Error},
    guide::OrnaAdminGuide,
};

/// Retrieve all items from the codex.
pub fn items(guide: &OrnaAdminGuide) -> Result<CodexItems, Error> {
    Ok(CodexItems {
        items: guide
            .codex_fetch_item_list()?
            .into_iter()
            .map(|item| guide.codex_fetch_item(item.slug()))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// Retrieve all searchable monsters from the codex.
/// This does not fetch monsters from non-active events.
pub fn monsters(guide: &OrnaAdminGuide) -> Result<CodexMonsters, Error> {
    Ok(CodexMonsters {
        monsters: guide
            .codex_fetch_monster_list()?
            .into_iter()
            .map(|monster| guide.codex_fetch_monster(monster.slug()))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// Retrieve all searchable bosses from the codex.
/// This does not fetch bosses from non-active events.
pub fn bosses(guide: &OrnaAdminGuide) -> Result<CodexBosses, Error> {
    Ok(CodexBosses {
        bosses: guide
            .codex_fetch_boss_list()?
            .into_iter()
            .map(|boss| guide.codex_fetch_boss(boss.slug()))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// Retrieve all searchable raids from the codex.
/// This does not fetch raids from non-active events.
pub fn raids(guide: &OrnaAdminGuide) -> Result<CodexRaids, Error> {
    Ok(CodexRaids {
        raids: guide
            .codex_fetch_raid_list()?
            .into_iter()
            .map(|raid| guide.codex_fetch_raid(raid.slug()))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// Retrieve all skills from the codex.
pub fn skills(guide: &OrnaAdminGuide) -> Result<CodexSkills, Error> {
    Ok(CodexSkills {
        skills: guide
            .codex_fetch_skill_list()?
            .into_iter()
            .map(|skill| guide.codex_fetch_skill(skill.slug()))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}

/// Retrieve all searchable followers from the codex.
/// This does not fetch followers from non-active events.
pub fn followers(guide: &OrnaAdminGuide) -> Result<CodexFollowers, Error> {
    Ok(CodexFollowers {
        followers: guide
            .codex_fetch_follower_list()?
            .into_iter()
            .map(|follower| guide.codex_fetch_follower(follower.slug()))
            .collect::<Result<Vec<_>, Error>>()?,
    })
}
