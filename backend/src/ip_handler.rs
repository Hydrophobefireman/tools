use std::net::IpAddr;

use crate::err;
use axum::Json;
use maxminddb::geoip2;
use serde::{Deserialize, Serialize};

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
    city: Option<String>,
    country: Option<String>,
    asn: Option<String>,
    ip: String,
}

pub fn fetch_ip_details(addr: IpAddr) -> axum::response::Result<Json<IPInfo>> {
    let city_reader =
        maxminddb::Reader::open_readfile("/data/GeoIP/GeoLite2-City.mmdb").map_err(err::err_handler)?;
    let city: geoip2::City = city_reader.lookup(addr).map_err(err::err_handler)?;
    let asn_reader =
        maxminddb::Reader::open_readfile("/data/GeoIP/GeoLite2-ASN.mmdb").map_err(err::err_handler)?;
    let asn: geoip2::Asn = asn_reader.lookup(addr).map_err(err::err_handler)?;
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
