use ipinfo::{IpDetails, IpInfo, IpInfoConfig};
use regex::Regex;
use rocket::serde::{json::Json, Serialize};
use rocket_client_addr::ClientRealAddr;

use std::env::var;

use crate::serialized_ip_info::IpDetailsDef;
use crate::util::get_ip_type;

#[derive(FromFormField, PartialEq)]
pub enum Distance {
    Km,
    Mi,
    Nm,
}

#[allow(dead_code)]
#[derive(FromForm)]
pub struct GetIPOptions {
    #[field(default = true)]
    isp: bool,
    #[field(default = Distance::Km)]
    distance: Distance,
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize)]
#[serde(crate = "rocket::serde", rename_all = "camelCase")]
pub struct GetIPResponse {
    processed_string: Option<String>,
    #[serde_as(as = "Option<IpDetailsDef>")]
    raw_isp_info: Option<IpDetails>,
}

#[get("/getIP?<opts..>")]
pub async fn get_ip(client_addr: &ClientRealAddr, opts: GetIPOptions) -> Json<GetIPResponse> {
    let mut result = GetIPResponse {
        processed_string: None,
        raw_isp_info: None,
    };
    let ip = client_addr.ip.to_string().replace("::ffff:", "");

    let (processed_string, is_special_ip) = get_ip_type(&ip);
    result.processed_string = Some(processed_string);

    if is_special_ip {
        return Json(result);
    }

    if opts.isp {
        let ipinfo = get_ipinfo(&ip).await;
        result.raw_isp_info = Some(ipinfo.to_owned());

        let org_regex = Regex::new(r"AS\d+\s").unwrap();
        let org = &ipinfo.org;

        let mut isp = org_regex
            .replace_all(org.as_deref().unwrap(), "")
            .as_ref()
            .to_string();

        if isp.is_empty() {
            isp = String::from("Unknown ISP");
        }

        if !ipinfo.country.is_empty() {
            isp = format!("{}, {}", &isp, &ipinfo.country);
        }

        result.processed_string = Some(format!("{} - {}", &result.processed_string.unwrap(), &isp));
    }

    Json(result)
}

#[get("/getIP.php?<opts..>")]
pub async fn get_ip_php(client_addr: &ClientRealAddr, opts: GetIPOptions) -> Json<GetIPResponse> {
    get_ip(client_addr, opts).await
}

#[get("/backend/getIP.php?<opts..>")]
pub async fn get_backend_ip_php(
    client_addr: &ClientRealAddr,
    opts: GetIPOptions,
) -> Json<GetIPResponse> {
    get_ip(client_addr, opts).await
}

pub async fn get_ipinfo(ip: &str) -> IpDetails {
    let config = IpInfoConfig {
        token: Some(var("IPINFO_TOKEN").unwrap_or(String::new())),
        ..Default::default()
    };

    let mut ipinfo_client = IpInfo::new(config).expect("should construct");

    let res = ipinfo_client.lookup(&[ip]).unwrap();
    let ipinfo = res.get(ip).unwrap();

    ipinfo.to_owned()
}
