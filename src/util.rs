use std::env::var;

use haversine::{distance, Location, Units};
use regex::Regex;

use crate::get_ip::Distance;

pub fn get_ip_type(ip: &str) -> (String, bool) {
    let private_regex: Regex = Regex::new(r"^172\.(1[6-9]|2\d|3[01])\.").unwrap();

    let mut is_special_ip: bool = true;
    let processed_string = if ip == "::1" {
        format!("{} - localhost IPv6 access", &ip)
    } else if ip.starts_with("fe80:") {
        format!("{} - link-local IPv6 access", &ip)
    } else if ip.starts_with("127.") {
        format!("{} - localhost IPv4 access", &ip)
    } else if ip.starts_with("10.") {
        format!("{} - private IPv4 access", &ip)
    } else if private_regex.is_match(&ip) {
        format!("{} - private IPv4 access", &ip)
    } else if ip.starts_with("192.168.") {
        format!("{} - private IPv4 access", &ip)
    } else if ip.starts_with("192.168.") {
        format!("{} - private IPv4 access", &ip)
    } else {
        is_special_ip = false;
        ip.to_string()
    };

    (processed_string, is_special_ip)
}

pub fn parse_location_string(location: String) -> Result<Location, String> {
    let location_parts: Vec<&str> = location.split(",").collect::<Vec<_>>();

    if location_parts.len() != 2 {
        return Err(format!("Unknown location format: {}", &location));
    }

    let latitude = location_parts[0].parse::<f64>().unwrap_or_default();
    let longitude = location_parts[1].parse::<f64>().unwrap_or_default();

    Ok(Location {
        latitude,
        longitude,
    })
}

pub fn get_client_server_distance_string(client_location: String, units: Distance) -> String {
    let client_location = parse_location_string(client_location).unwrap_or(Location {
        latitude: 0.0,
        longitude: 0.0,
    });

    let server_location = Location {
        latitude: var("SERVER_LATITUDE")
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap_or_default(),
        longitude: var("SERVER_LONGITUDE")
            .unwrap_or_default()
            .parse::<f64>()
            .unwrap_or_default(),
    };

    let haversine_units = match units {
        Distance::Km => Units::Kilometers,
        Distance::Mi => Units::Miles,
        Distance::Nm => Units::Kilometers,
    };

    let distance = distance(client_location, server_location, haversine_units);
    let distance = match units {
        Distance::Km => distance,
        Distance::Mi => distance,
        Distance::Nm => distance / 1.852,
    };
    format!("{:.2} {}", &distance, units)
}
