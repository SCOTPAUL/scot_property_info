mod providers;
use structopt::StructOpt;


#[derive(Debug, StructOpt)]
#[structopt(name= "scot_property_info")]
struct Opts {
    #[structopt(short, long, required_unless = "postcode", conflicts_with = "postcode")]
    address: Option<String>,

    #[structopt(short, long, required_unless = "address", conflicts_with = "address")]
    postcode: Option<String>

}

fn main() {
    let opt = Opts::from_args();
    println!("{:?}", opt);
}