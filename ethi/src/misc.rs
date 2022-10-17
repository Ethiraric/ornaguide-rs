use std::{
    fs::File,
    io::{BufReader, Read},
};

use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use ornaguide_rs::{data::OrnaData, error::Error};
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Deserializer,
};

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
    /// Convert the `Vec` of skill ids to a sorted `Vec` of codex URIs for the skills.
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

/// A trait to extend `Vec<u32>` specifically.
/// Use with caution, as this should only be used on `Vec`s that hold `u32`s representing status
/// effect ids.
pub trait VecStatusEffectIds {
    /// Convert the `Vec` of status effect ids to a sorted `Vec` of codex URIs for the status
    /// effects.
    fn guide_status_effect_ids_to_guide_names<'a>(&self, data: &'a OrnaData) -> Vec<&'a str>;
}

impl VecStatusEffectIds for Vec<u32> {
    fn guide_status_effect_ids_to_guide_names<'a>(&self, data: &'a OrnaData) -> Vec<&'a str> {
        self.iter()
            .map(|id| {
                data.guide
                    .static_
                    .status_effects
                    .iter()
                    .find(|status| status.id == *id)
                    .unwrap_or_else(|| panic!("Failed to find status effect {}", id))
                    .name
                    .as_str()
            })
            .sorted()
            .collect()
    }
}

/// Run the given expression, and retry it once if it returns an `Err`.
/// This macro cannot be called if the given expression moves a variable, as there would be no way
/// of re-trying.
#[macro_export]
macro_rules! retry_once {
    ($expr:expr) => {
        match $expr {
            Ok(x) => Ok(x),
            Err(_) => $expr,
        }
    };
}

/// Same as `serde_json::from_reader`, but adds the name to the error message, if any.
pub fn json_read<R, T>(rdr: R, path: &str) -> Result<T, Error>
where
    R: Read,
    T: serde::de::DeserializeOwned,
{
    serde_json::from_reader(rdr).map_err(|err| Error::SerdeJson(err, path.to_string()))
}

/// Same as a `serde_json::from_reader` with a file, but adds the filename to the error message, if any.
#[allow(dead_code)]
pub fn json_from_file<R, T>(path: &str) -> Result<T, Error>
where
    R: Read,
    T: serde::de::DeserializeOwned,
{
    json_read(BufReader::new(File::open(path)?), path)
}

/// Parse the given value as a Google Doc boolean value.
/// Maps `"TRUE"` to `true`, `"FALSE"` to `false`, and any other value to an error.
pub fn parse_gdoc_bool<'de, D>(de: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&str>::deserialize(de)?;
    match s {
        "TRUE" => Ok(true),
        "FALSE" => Ok(false),
        _ => Err(serde::de::Error::invalid_value(
            Unexpected::Str(s),
            &"TRUE or FALSE",
        )),
    }
}

/// Parse the given value as an `u8`. If the value is an empty string, return `0` instead.
pub fn parse_maybe_empty_u8<'de, D>(de: D) -> Result<u8, D::Error>
where
    D: Deserializer<'de>,
{
    struct V {}
    impl<'de> Visitor<'de> for V {
        type Value = u8;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "A u8 or an empty string (maps to 0)")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            if v > u8::MAX as u64 {
                Err(serde::de::Error::invalid_value(
                    Unexpected::Unsigned(v),
                    &"a value fitting in [0;255]",
                ))
            } else {
                Ok(v as u8)
            }
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            match v {
                "" => Ok(0),
                _ => Err(serde::de::Error::invalid_value(
                    Unexpected::Str(v),
                    &"empty string in place of u8",
                )),
            }
        }
    }

    de.deserialize_any(V {})
}

/// Turns a PascalCase name into a kebab-case one.
#[allow(dead_code)]
fn kebab_casify(slug: &str) -> String {
    let mut new_slug = String::new();
    let mut capital = true;
    for c in slug.chars() {
        if c.is_uppercase() {
            if !capital {
                new_slug.push('-');
            }
            new_slug.push(c.to_ascii_lowercase());
            capital = true;
        } else {
            new_slug.push(c);
            capital = false;
        }
    }
    new_slug
}
