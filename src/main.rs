use structopt::StructOpt;

mod lib;

#[derive(StructOpt)]
struct Cli {
    country: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    lib::init(args.country);
    Ok(())
}
