use chrono::prelude::*;
use chrono::Datelike;
use serde::Deserialize;
use std::collections::HashMap;

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
    pub postcode: String,
    dz: String,
    pub rank: u32,
    pub vigintile: u8,
    pub decile: u8,
    quintile: u8
}

pub fn fetch_simd_postcode_info() -> Result<HashMap<String, SIMDPostcodeInfo>, csv::Error> {
    let postcode_bytes = include_bytes!("../resources/simd_postcodes.csv") as &[u8];
    let mut rdr = csv::Reader::from_reader(postcode_bytes);
    let mut postcodes = HashMap::new();

    for result in rdr.deserialize() {
        let record: SIMDPostcodeInfo = result?;
        postcodes.insert(record.postcode.clone(), record);
    }

    Ok(postcodes)

}