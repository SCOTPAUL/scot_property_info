use chrono::prelude::*;
use chrono::Datelike;
use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;
use std::error::Error;
use serde_with::{serde_as, DisplayFromStr};
use table_extract::Table;

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct LocationInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub lat: f32,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub lon: f32,
    query: Option<String>,
    pub road: Option<String>,
    pub address: AddressInfo
}

#[derive(Debug, Deserialize)]
pub struct AddressInfo {
    pub postcode: String
}

#[derive(Debug)]
pub struct TaxBand {
    pub address: String,
    pub band: String
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

pub async fn fetch_address_info(query: &str) -> Result<LocationInfo, Box<dyn Error>> {
    let request_url = format!("https://nominatim.openstreetmap.org/search?q={q}&format=json&addressdetails=1&limit=1",
                              q = urlencoding::encode(query));

    let resp = reqwest::Client::builder()
        .user_agent("scot_property_info")
        .build()?
        .get(request_url)
        .send()
        .await?;

    let mut address_info: Vec<LocationInfo> = resp.json().await?;

    let mut address_info_val = address_info.remove(0);

    address_info_val.query = Some(query.to_string());

    Ok(address_info_val)
}

pub async fn fetch_council_tax_info(location: &LocationInfo) -> Result<Vec<TaxBand>, Box<dyn Error>>{
    let request_url = format!(
        "https://www.saa.gov.uk/search/?SEARCHED=1&ST=&SEARCH_TERM={postcode}&DISPLAY_COUNT=100&SEARCH_TABLE=council_tax&searchtype=#results",
        postcode = urlencoding::encode(location.address.postcode.as_str())
    );

    let html = reqwest::get(request_url)
        .await?
        .text()
        .await?;

    let table = Table::find_first(html.as_str()).unwrap();

    let mut tax_bands = vec![];

    for row in &table {
        let address: Vec<&str> = row.get("Property Address")
            .unwrap_or("<ADDRESS_ERROR>")
            .split("<br>")
            .collect();
        let address_without_postcode_or_city = &address[0..address.len() - 2];
        let address_str = address_without_postcode_or_city.join("\n");

        let tax_band = TaxBand {
            address: address_str,
            band: row.get("Band").unwrap_or("<BAND_ERROR>").to_string()
        };

        tax_bands.push(tax_band);
    }

    Ok(tax_bands)
}