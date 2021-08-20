mod providers;
use structopt::StructOpt;
use crate::providers::SIMDPostcodeInfo;
use std::error::Error;


#[derive(Debug, StructOpt)]
#[structopt(name= "scot_property_info")]
struct Opts {
    #[structopt(short, long, required_unless = "postcode", conflicts_with = "postcode")]
    address: Option<String>,

    #[structopt(short, long, required_unless = "address", conflicts_with = "address")]
    postcode: Option<String>

}

fn main() -> Result<(), Box<dyn Error>>  {
    let opt = Opts::from_args();

    let postcode_bytes = include_bytes!("../resources/simd_postcodes.csv") as &[u8];
    let mut rdr = csv::Reader::from_reader(postcode_bytes);

    for result in rdr.deserialize() {
        let record: SIMDPostcodeInfo = result?;
        println!("{:?}", record);
        // Try this if you don't like each record smushed on one line:
        // println!("{:#?}", record);
    }

    println!("{:?}", opt);

    Ok(())
}