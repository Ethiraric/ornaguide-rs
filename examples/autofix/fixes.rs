use ornaguide_rs::{
    error::Error,
    guide::{AdminGuide, OrnaAdminGuide},
    items::RawItem,
    skills::RawSkill,
};

pub struct Fixes {
    dry_run: bool,
    guide: OrnaAdminGuide,
}

impl Fixes {
    /// Creates a new `Fixes`.
    pub fn new(dry_run: bool, guide: OrnaAdminGuide) -> Self {
        Self { dry_run, guide }
    }

    /// Set materials of the given item on the guide.
    /// This erases the previous materials list.
    pub fn set_item_materials_to(
        &self,
        raw_item: &RawItem,
        materials: &[u32],
    ) -> Result<(), Error> {
        println!(
            "Setting materials of item #{} {} to {:?}",
            raw_item.id, raw_item.name, materials
        );
        if self.dry_run {
            return Ok(());
        }

        let mut item = self.guide.admin_retrieve_item_by_id(raw_item.id)?;
        if item.materials.len() != materials.len()
            || !materials.iter().all(|mat| item.materials.contains(mat))
        {
            item.materials = materials.to_vec();
            self.guide.admin_save_item(item)
        } else {
            println!("Guide is okay. Please refresh cache.",);
            Ok(())
        }
    }

    /// Add a material to an item on the guide.
    /// This preserves the previous materials list.
    pub fn add_item_materials(&self, raw_item: &RawItem, materials: &[u32]) -> Result<(), Error> {
        println!(
            "Adding materials {:?} to item #{} {}",
            materials, raw_item.id, raw_item.name
        );
        if self.dry_run {
            return Ok(());
        }

        let mut item = self.guide.admin_retrieve_item_by_id(raw_item.id)?;
        let mut edited = false;
        for mat in materials {
            if !item.materials.contains(mat) {
                item.materials.push(*mat);
                edited = true;
            }
        }
        if edited {
            self.guide.admin_save_item(item)
        } else {
            println!(
                "Guide already has materials {:?} for item #{} {}. Please refresh cache.",
                materials, raw_item.id, raw_item.name
            );
            Ok(())
        }
    }

    /// Set the `is_magic` field from a skill.
    pub fn set_skill_is_magic(&self, raw_skill: &RawSkill, is_magic: bool) -> Result<(), Error> {
        println!(
            "Setting is_magic to {} for skill #{} {}",
            is_magic, raw_skill.id, raw_skill.name
        );
        if self.dry_run {
            return Ok(());
        }

        let mut skill = self.guide.admin_retrieve_skill_by_id(raw_skill.id)?;
        if skill.is_magic != is_magic {
            skill.is_magic = is_magic;
            self.guide.admin_save_skill(skill)
        } else {
            println!(
                "is_magic for skill #{} {} is already {}",
                raw_skill.id, raw_skill.name, is_magic
            );
            Ok(())
        }
    }
}
