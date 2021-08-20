use chrono::prelude::*;
use chrono::Datelike;
use serde::Deserialize;

struct AddressInfo {
    lat: f32,
    lon: f32,
    query: Option<String>,
    road: Option<String>,
    postcode: String
}

struct TaxBand {
    address: String,
    band: char
}

struct PurchasePrice<D> where D: Datelike {
    address: String,
    purchase_price_pounds: u32,
    purchase_date: D
}

struct SIMDInfo {
    postcode: String,
    overall_rank: u32,
    income_domain_rank: u32,
    employment_domain_rank: u32,
    education_domain_rank: u32,
    health_domain_rank: u32,
    access_domain_rank: u32,
    crime_domain_rank: u32,
    housing_domain_rank: u32,
    population: u32
}

#[derive(Debug, Deserialize)]
pub struct SIMDPostcodeInfo {
    postcode: String,
    dz: String,
    rank: u32,
    vigintile: u8,
    decile: u8,
    quintile: u8
}