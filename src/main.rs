use structopt::StructOpt;

mod explain;
mod format;
mod shell;
mod utils;

use utils::Result;

#[derive(StructOpt)]
pub enum Action
{
    Explain, Format, Shell
}

#[derive(StructOpt)]
#[structopt(name = "petfall", about = "Something about handling JSON and talking petunias falling")]
pub struct Opts
{
    filename: String,
    #[structopt(subcommand)]
    action: Action
}

fn main() -> Result
{
    let opts = Opts::from_args();

    let contents = std::fs::read_to_string(&opts.filename)?;
    let json = json::parse(&contents)?;

    match opts.action {
        Action::Explain => explain::explain(json),
        Action::Format => format::format(json),
        Action::Shell => shell::shell(json)
    }
}
