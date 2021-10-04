#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_with;

pub mod empty;
pub mod garbage;
pub mod get_ip;
pub mod haversine;
pub mod serialized_ip_info;
pub mod util;

use dotenv::dotenv;
use std::error::Error;

use rocket::fs::FileServer;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors};

use empty::*;
use garbage::*;
use get_ip::*;

#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let cors = setup_cors().await?;
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
        empty_php,
        backend_empty_php,
        get_backend_empty_php
    ];

    let routes = vec![routes, garbage_routes, empty_routes].concat();

    let mut rocketship = rocket::build().attach(cors).mount("/", routes);

    let asset_path = std::env::current_dir().unwrap().join("assets");
    if asset_path.exists() {
        let fileserver = FileServer::from(asset_path);

        rocketship = rocketship.mount("/", fileserver);
    }

    rocketship.launch().await?;

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
