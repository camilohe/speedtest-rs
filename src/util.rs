use std::env::var;

use crate::haversine::{distance, Location, Units};
use regex::Regex;

pub fn get_ip_type(ip: &str) -> (String, bool) {
    let private_regex = Regex::new(r"^172\.(1[6-9]|2\d|3[01])\.").unwrap();
    let cgnat_regex = Regex::new(r"^100\.([6-9][0-9]|1[0-2][0-7])\.").unwrap();

    let mut is_special_ip: bool = true;

    let processed_string = match ip {
        n if n == "::1" => format!("{} - localhost IPv6 access", n),
        n if n.starts_with("fe80:") => format!("{} - link-local IPv6 access", n),
        n if n.starts_with("127.") => format!("{} - localhost IPv4 access", n),
        n if n.starts_with("10.") => format!("{} - private IPv4 access", n),
        n if private_regex.is_match(n) => format!("{} - private IPv4 access", n),
        n if n.starts_with("192.168.") => format!("{} - private IPv4 access", n),
        n if n.starts_with("169.254.") => format!("{} - link-local IPv4 access", n),
        n if cgnat_regex.is_match(n) => format!("{} - CGNAT IPv4 access", n),
        _ => {
            is_special_ip = false;
            ip.to_string()
        }
    };

    (processed_string, is_special_ip)
}

pub fn parse_location_string(location: String) -> Result<Location, String> {
    let location_parts = location.split(",").collect::<Vec<_>>();

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

pub fn get_client_server_distance_string(client_location: String, units: Units) -> String {
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

    let distance = distance(client_location, server_location, &units);

    format!("{:.2} {}", distance, units)
}
