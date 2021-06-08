use super::issue;
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
    command: Command,
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
    #[clap(long, short, about = "The title of the new ticket")]
    title: String,

    #[clap(long, short, about = "The type of the new ticket")]
    issue_type: issue::IssueType,
}

#[derive(Clap, Debug)]
pub struct CheckOpts {
    #[clap(
        long,
        short,
        about = "Perform a deep clean, checking all tickets for inconsistencies"
    )]
    deep: bool,
}

#[derive(Clap, Debug)]
pub struct ExtractOpts {
    #[clap(subcommand)]
    extract_what: ExtractWhat,
}

#[derive(Clap, Debug)]
pub enum ExtractWhat {
    Tests,
    Features,
}

pub fn cli_parse() -> Root {
    Root::parse()
}
