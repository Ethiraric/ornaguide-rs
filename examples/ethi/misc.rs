use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;

use crate::output::OrnaData;

pub fn bar(len: u64) -> ProgressBar {
    let bar = ProgressBar::new(len);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("{msg:15!} {eta:>3} [{wide_bar}] {pos:>4}/{len:4}")
            .progress_chars("=> "),
    );
    bar
}

pub fn sanitize_guide_name(name: &str) -> &str {
    if let Some(pos) = name.find('[') {
        name.split_at(pos - 1).0
    } else {
        name
    }
}

/// From 2 sorted slices, list elements that only appear in the first and second slice.
/// Elements that belong to both the slices are not returned.
pub fn diff_sorted_slices<'a, T: PartialEq + PartialOrd>(
    a: &'a [T],
    b: &'a [T],
) -> (Vec<&'a T>, Vec<&'a T>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let mut ait = a.iter().peekable();
    let mut bit = b.iter().peekable();

    loop {
        match (ait.peek(), bit.peek()) {
            (Some(a), Some(b)) => {
                if a == b {
                    ait.next();
                    bit.next();
                } else if a < b {
                    left.push(*a);
                    ait.next();
                } else {
                    right.push(*b);
                    bit.next();
                }
            }
            (Some(_), None) => {
                left.extend(ait);
                break;
            }
            (None, Some(_)) => {
                right.extend(bit);
                break;
            }
            (None, None) => break,
        }
    }

    (left, right)
}

/// A trait to extend `Vec<u32>` specifically.
/// Use with caution, as this should only be used on `Vec`s that hold `u32`s representing skill
/// ids.
pub trait VecSkillIds {
    fn guide_skill_ids_to_codex_uri<'a>(&self, data: &'a OrnaData) -> Vec<&'a str>;
}

impl VecSkillIds for Vec<u32> {
    fn guide_skill_ids_to_codex_uri<'a>(&self, data: &'a OrnaData) -> Vec<&'a str> {
        self.iter()
            .filter_map(|id| {
                data.guide
                    .skills
                    .skills
                    .iter()
                    .find(|skill| skill.id == *id)
                    .map(|skill| skill.codex_uri.as_str())
                    .filter(|uri| !uri.is_empty())
            })
            .sorted()
            .collect()
    }
}
