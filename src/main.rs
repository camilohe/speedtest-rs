#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_with;

pub mod empty;
pub mod garbage;
pub mod get_ip;
pub mod serialized_ip_info;
pub mod util;

use dotenv::dotenv;
use std::error::Error;
use std::path::Path;

use rocket::fs::{relative, FileServer};
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

use empty::*;
use garbage::*;
use get_ip::*;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let cors = setup_cors().await.unwrap();
    let routes = routes![get_ip::get_ip, get_backend_ip_php];

    let garbage_routes = routes![
        garbage::garbage,
        garbage_php,
        backend_garbage,
        backend_garbage_php
    ];

    let empty_routes = routes![
        empty::empty,
        backend_empty,
        get_empty,
        empty_options,
        empty_php,
        backend_empty_php,
        get_backend_empty_php
    ];

    let asset_path = relative!("assets");
    if Path::new(asset_path).exists() {
        let fileserver = FileServer::from(asset_path);

        rocket::build()
            .attach(cors)
            .mount("/", routes)
            .mount("/", empty_routes)
            .mount("/", garbage_routes)
            .mount("/", fileserver)
            .launch()
            .await?;
    } else {
        rocket::build()
            .attach(cors)
            .mount("/", routes)
            .mount("/", empty_routes)
            .mount("/", garbage_routes)
            .launch()
            .await?;
    }

    Ok(())
}

async fn setup_cors() -> Result<Cors, Box<dyn Error>> {
    let allowed_origins = AllowedOrigins::all();
    let allowed_methods = vec![Method::Get, Method::Post]
        .into_iter()
        .map(From::from)
        .collect();
    let allowed_headers = AllowedHeaders::some(&["Content-Encoding", "Content-Type"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        ..Default::default()
    }
    .to_cors()?;

    Ok(cors)
}
