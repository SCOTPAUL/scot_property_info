mod providers;
use structopt::StructOpt;
use crate::providers::{SIMDPostcodeInfo, fetch_simd_postcode_info, fetch_address_info, fetch_council_tax_info, TaxBand};
use std::error::Error;
use std::collections::HashMap;
use prettytable::{Table, row, cell};



#[derive(Debug, StructOpt)]
#[structopt(name= "scot_property_info")]
struct Opts {
    #[structopt(short, long, required_unless = "postcode", conflicts_with = "postcode")]
    address: Option<String>,

    #[structopt(short, long, parse(from_str = to_upper), required_unless = "address", conflicts_with = "address")]
    postcode: Option<String>

}

fn remove_postcode_whitespace(postcode: &str) -> String {
    postcode.chars().filter(|c| !c.is_whitespace()).collect()
}

fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

fn create_council_tax_table(tax_bands: &Vec<TaxBand>) -> Table {
    let mut table = Table::new();

    table.add_row(row!["Address", "Tax Band"]);
    for tax_band in tax_bands {
        table.add_row(row![tax_band.address, tax_band.band]);
    }

    table

}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    let opt: Opts = Opts::from_args();

    let postcodes = fetch_simd_postcode_info()?;

    let location_query = opt.postcode.or(opt.address).unwrap();

    let address_info = fetch_address_info(&location_query).await?;

    let postcode_no_whitespace = remove_postcode_whitespace(&address_info.address.postcode);

    println!("Address info {:?}", address_info);

    let simd_info = postcodes.get(&postcode_no_whitespace);

    if let Some(simd) = simd_info {
        println!("SIMD decile for this postcode is {:?}", simd.decile.to_string())
    }
    else {
        println!("Couldn't find postcode {:?}, please check it's correct", &address_info.address.postcode);
    }

    let council_tax_info = fetch_council_tax_info(&address_info).await?;

    let council_tax_table = create_council_tax_table(&council_tax_info);

    council_tax_table.printstd();

    Ok(())
}