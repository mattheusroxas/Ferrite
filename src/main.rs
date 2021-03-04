mod cli;
mod search;
use crate::cli::General;
// use cli::SubCommand;
use std::error::Error;
// use structopt::StructOpt;

fn main() -> Result<(), Box<dyn Error + 'static>> {
    // let opts = SubCommand::from_args();
    // println!("{:?}", opts);
    let config_file = dirs::config_dir()
        .unwrap()
        .join("ferrite")
        .join("config.toml");
    let _general: General = toml::from_str(&std::fs::read_to_string(config_file)?)?;
    search::search();

    Ok(())
}
