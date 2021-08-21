mod providers;
use structopt::StructOpt;
use crate::providers::{SIMDPostcodeInfo, fetch_simd_postcode_info};
use std::error::Error;
use std::collections::HashMap;


#[derive(Debug, StructOpt)]
#[structopt(name= "scot_property_info")]
struct Opts {
    #[structopt(short, long, required_unless = "postcode", conflicts_with = "postcode")]
    address: Option<String>,

    #[structopt(short, long, parse(from_str = parse_postcode), required_unless = "address", conflicts_with = "address")]
    postcode: Option<String>

}

fn parse_postcode(postcode: &str) -> String {
    postcode.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() -> Result<(), Box<dyn Error>>  {
    let opt: Opts = Opts::from_args();

    let postcodes = fetch_simd_postcode_info()?;

    if let Some(postcode) = &opt.postcode {
        let simd_info = postcodes.get(postcode);

        if let Some(simd) = simd_info {
            println!("SIMD decile for this postcode is {:?}", simd.decile)
        }
        else {
            println!("Couldn't find postcode {:?}, please check it's correct", postcode);
        }
    }

    Ok(())
}