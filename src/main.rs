mod cli;
mod filters;
mod search;
mod videos;
use cli::SubCommand;
use structopt::StructOpt;

fn main() {
    let opts = SubCommand::from_args();
    println!("{:?}", opts);
}
