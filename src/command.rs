use super::ticket;
use clap::{AppSettings, Clap};
use std::str::FromStr;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(
    version = "0.0",
    author = "Freddie Gilbraith <freddie.gilbraith@littlebonsai.co.uk>"
)]
pub struct Root {
    #[clap(long, about = "Don't apply any changes, only display them and quit")]
    pub dry_run: bool,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap, Debug)]
pub enum Command {
    Create(CreateOpts),
    Check(CheckOpts),
    Fix(CheckOpts),
    Extract(ExtractOpts),
    Test,
}

#[derive(Clap, Debug)]
pub struct CreateOpts {
    #[clap(long, about = "The title of the new ticket")]
    pub title: String,

    #[clap(long, about = "The type of the new ticket")]
    pub ticket_type: ticket::TicketType,
}

#[derive(Clap, Debug)]
pub struct CheckOpts {
    #[clap(
        long,
        short,
        about = "Perform a deep clean, checking all tickets for inconsistencies"
    )]
    pub deep: bool,
}

#[derive(Clap, Debug)]
pub struct ExtractOpts {
    #[clap(subcommand)]
    pub extract_what: ExtractWhat,
}

#[derive(Clap, Debug)]
pub enum ExtractWhat {
    Tests,
    Features,
}

pub fn cli_parse() -> Root {
    Root::parse()
}
