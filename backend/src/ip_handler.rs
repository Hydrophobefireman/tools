extern crate lazy_static;
use std::net::IpAddr;

use crate::err;
use axum::Json;
use maxminddb::{geoip2, MaxMindDBError};
use serde::{Deserialize, Serialize};

type MaxMindDBReadResult = Result<maxminddb::Reader<Vec<u8>>, MaxMindDBError>;

lazy_static::lazy_static! {
    static ref CITY_READER: MaxMindDBReadResult = maxminddb::Reader::open_readfile("/data/GeoIP/GeoLite2-City.mmdb");
    static ref ASN_READER: MaxMindDBReadResult = maxminddb::Reader::open_readfile("/data/GeoIP/GeoLite2-ASN.mmdb") ;
}

const DEFAULT_CITY: geoip2::city::City = geoip2::city::City {
    geoname_id: None,
    names: None,
};
const DEFAULT_COUNTRY: geoip2::country::Country = geoip2::country::Country {
    geoname_id: None,
    names: None,
    is_in_european_union: None,
    iso_code: None,
};

#[derive(Deserialize)]
pub struct IPPayload {
    pub ip: String,
}

#[derive(Serialize)]
pub struct IPInfo {
    pub city: Option<String>,
    pub country: Option<String>,
    pub asn: Option<String>,
    pub ip: String,
}

pub fn fetch_ip_details(addr: IpAddr) -> axum::response::Result<Json<IPInfo>> {
    let city: geoip2::City = match CITY_READER.is_err() {
        true => {
            return Err(CITY_READER
                .as_ref()
                .err()
                .map(err::err_handler)
                .unwrap()
                .into())
        }
        false => CITY_READER.as_ref().ok().unwrap(),
    }
    .lookup(addr)
    .map_err(err::err_handler)?;
    let asn: geoip2::Asn = ASN_READER
        .as_ref()
        .unwrap()
        .lookup(addr)
        .map_err(err::err_handler)?;
    Ok(Json(IPInfo {
        city: city
            .city
            .unwrap_or(DEFAULT_CITY)
            .names
            .unwrap_or_default()
            .get("en")
            .map(ToString::to_string),
        country: city
            .country
            .unwrap_or(DEFAULT_COUNTRY)
            .names
            .unwrap_or_default()
            .get("en")
            .map(ToString::to_string),
        asn: asn.autonomous_system_organization.map(ToString::to_string),
        ip: addr.to_string(),
    }))
}
