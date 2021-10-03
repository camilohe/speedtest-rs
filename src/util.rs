use regex::Regex;

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
