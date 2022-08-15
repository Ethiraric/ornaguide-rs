#[macro_use]
extern crate rocket;

extern crate lazy_static;

use std::{net::IpAddr, str::FromStr};

use rocket::{routes, Config};

use crate::data::DATA;

mod cors;
mod data;
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

#[launch]
fn rocket() -> _ {
    let config = Config {
        port: 12346,
        address: IpAddr::from_str("0.0.0.0").unwrap(),
        ..Config::debug_default()
    };

    if let Err(e) = DATA.as_ref() {
        panic!("{}", e);
    }

    rocket::custom(&config)
        .attach(cors::Cors)
        .mount(
            "/api/v0.1",
            routes![
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
