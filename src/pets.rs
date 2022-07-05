//! Contains the different pet structures that are fetched from the API.
//!
//! The [`raw`] submodule contains the generic structure that we retrieve from the API. All fields
//! that _may_ be `null` in the json for _any_ pet of any kind is an `Option`.
//!
//! The [`pet`] submodule contains utilities to remove some of the `Option`s and categorize the
//! pets into the same categories that are in the in-game inventory.
//!
//! The [`admin`] submodule contains classes for the administration view of the guide.
//!
//! The Rust pets from the [`raw`] and [`pet`] modules are publicly used in this module.

pub mod admin;
// pub mod pet;
// pub mod raw;
