use std::collections::HashMap;

use rocket::serde::{self, json, Serialize};

use ipinfo::{
    AbuseDetails, AsnDetails, CarrierDetails, CompanyDetails, DomainsDetails, IpDetails,
    PrivacyDetails,
};
use serde_with::SerializeAs;

#[derive(Serialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "camelCase",
    remote = "AsnDetails"
)]
pub struct AsnDetailsDef {
    pub asn: String,
    pub name: String,
    pub domain: String,
    pub route: String,
    pub asn_type: String,
}

impl SerializeAs<AsnDetails> for AsnDetailsDef {
    fn serialize_as<S>(value: &AsnDetails, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        AsnDetailsDef::serialize(value, serializer)
    }
}

#[derive(Serialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "camelCase",
    remote = "CompanyDetails"
)]
pub struct CompanyDetailsDef {
    pub name: String,
    pub domain: String,
    pub company_type: String,
}

impl SerializeAs<CompanyDetails> for CompanyDetailsDef {
    fn serialize_as<S>(value: &CompanyDetails, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        CompanyDetailsDef::serialize(value, serializer)
    }
}

#[derive(Serialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "camelCase",
    remote = "CarrierDetails"
)]
pub struct CarrierDetailsDef {
    pub name: String,
    pub mcc: String,
    pub mnc: String,
}

impl SerializeAs<CarrierDetails> for CarrierDetailsDef {
    fn serialize_as<S>(value: &CarrierDetails, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        CarrierDetailsDef::serialize(value, serializer)
    }
}

#[derive(Serialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "camelCase",
    remote = "PrivacyDetails"
)]
pub struct PrivacyDetailsDef {
    pub vpn: bool,
    pub proxy: bool,
    pub tor: bool,
    pub hosting: bool,
}

impl SerializeAs<PrivacyDetails> for PrivacyDetailsDef {
    fn serialize_as<S>(value: &PrivacyDetails, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        PrivacyDetailsDef::serialize(value, serializer)
    }
}

#[derive(Serialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "camelCase",
    remote = "AbuseDetails"
)]
pub struct AbuseDetailsDef {
    pub address: String,
    pub country: String,
    pub email: String,
    pub name: String,
    pub network: String,
    pub phone: String,
}

impl SerializeAs<AbuseDetails> for AbuseDetailsDef {
    fn serialize_as<S>(value: &AbuseDetails, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        AbuseDetailsDef::serialize(value, serializer)
    }
}

#[derive(Serialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "camelCase",
    remote = "DomainsDetails"
)]
pub struct DomainsDetailsDef {
    pub ip: Option<String>,
    pub total: u64,
    pub domains: Vec<String>,
}

impl SerializeAs<DomainsDetails> for DomainsDetailsDef {
    fn serialize_as<S>(value: &DomainsDetails, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        DomainsDetailsDef::serialize(value, serializer)
    }
}

#[serde_as]
#[skip_serializing_none]
#[derive(Serialize)]
#[serde(
    crate = "rocket::serde",
    rename_all = "camelCase",
    remote = "IpDetails"
)]
pub struct IpDetailsDef {
    pub ip: String,
    pub hostname: Option<String>,
    pub city: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub org: Option<String>,
    pub postal: Option<String>,
    pub timezone: Option<String>,
    #[serde_as(as = "Option<AsnDetailsDef>")]
    pub asn: Option<AsnDetails>,
    #[serde_as(as = "Option<CompanyDetailsDef>")]
    pub company: Option<CompanyDetails>,
    #[serde_as(as = "Option<CarrierDetailsDef>")]
    pub carrier: Option<CarrierDetails>,
    #[serde_as(as = "Option<PrivacyDetailsDef>")]
    pub privacy: Option<PrivacyDetails>,
    #[serde_as(as = "Option<AbuseDetailsDef>")]
    pub abuse: Option<AbuseDetails>,
    #[serde_as(as = "Option<DomainsDetailsDef>")]
    pub domains: Option<DomainsDetails>,

    #[serde(flatten)]
    pub extra: HashMap<String, json::Value>,
}

impl SerializeAs<IpDetails> for IpDetailsDef {
    fn serialize_as<S>(value: &IpDetails, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        IpDetailsDef::serialize(value, serializer)
    }
}
