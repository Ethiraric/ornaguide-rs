#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

#[macro_use]
extern crate rocket;

extern crate lazy_static;

use std::{net::IpAddr, str::FromStr};

use rocket::{routes, Config};

mod assessat;
mod cors;
mod data;
mod deref;
mod error;
mod filter;
mod items;
mod misc;
mod monsters;
mod options;
mod pets;
mod rocket_utils;
mod sirscor;
mod skills;

fn main() {
    foo::main();
}

mod foo {
    //! This module exists for the sole purpose of having this lint be restricted to a portion of
    //! the project.
    //! For some reason, `clippy` does not understand it when applied to [`rocket`], whether it be
    //! before, after, or before and after `#[launch]`.
    #![allow(clippy::no_effect_underscore_binding)]

    use crate::{
        assessat, cors, data::DATA, items, monsters, pets, rocket, routes, sirscor, skills, Config,
        FromStr, IpAddr,
    };

    #[launch]
    pub fn rocket() -> _ {
        let config = Config {
            port: 12346,
            address: IpAddr::from_str("0.0.0.0").unwrap(),
            ..Config::default()
        };

        if let Err(e) = DATA.as_ref() {
            panic!("{}", e);
        }

        rocket::custom(&config)
            .attach(cors::Cors)
            .mount(
                "/api/v0.1",
                routes![
                    assessat::post,
                    items::options,
                    items::post,
                    monsters::options,
                    monsters::post,
                    pets::options,
                    pets::post,
                    skills::options,
                    skills::post,
                ],
            )
            .mount("/", routes![sirscor::get])
    }
}
