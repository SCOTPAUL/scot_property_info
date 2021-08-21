mod providers;
use structopt::StructOpt;
use crate::providers::{SIMDPostcodeInfo, fetch_simd_postcode_info, fetch_address_info};
use std::error::Error;
use std::collections::HashMap;


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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>  {
    let opt: Opts = Opts::from_args();

    let postcodes = fetch_simd_postcode_info()?;

    if let Some(postcode) = &opt.postcode {
        let postcode_no_whitespace = remove_postcode_whitespace(&postcode);

        let address_info = fetch_address_info(&postcode).await?;

        let simd_info = postcodes.get(&postcode_no_whitespace);

        if let Some(simd) = simd_info {
            println!("SIMD decile for this postcode is {:?}", simd.decile)
        }
        else {
            println!("Couldn't find postcode {:?}, please check it's correct", postcode);
        }
    }

    Ok(())
}