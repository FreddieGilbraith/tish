use serde::{Deserialize, Serialize};

mod issue {
    use uuid::Uuid;

    pub struct RootIssue {
        id: Uuid,
        title: String,
        description: String,
        issue_type: IssueType,
        status: Status,

        test_cmd: Option<String>,
        feature: Option<String>,

        priority: Option<u64>,

        reporter: Person,
        assignee: Option<Person>,

        time: Option<IssueTime>,
        links: Option<IssueLinks>,

        comments: Vec<IssueComment>,
    }

    pub struct IssueLinks {
        parent: Option<IssueLink>,
        children: Option<Vec<IssueLink>>,
        blocks: Option<Vec<IssueLink>>,
        depends: Option<Vec<IssueLink>>,
        duplicate: Option<Vec<IssueLink>>,
    }

    pub struct IssueLink {
        id: Uuid,
        issue_type: IssueType,
        title: String,
    }

    pub enum IssueType {
        Epic,
        Story,
        Task,
        Bug,
    }

    pub struct IssueTime {
        estimate: u64,
        spent: u64,
    }

    pub struct Status {
        name: String,
    }

    pub struct IssueComment {
        text: String,
        author: Person,
    }

    pub struct Person {
        name: String,
        email: String,
    }
}

pub mod project {
    use super::issue;
    use async_std::fs;
    use normpath::PathExt;
    use serde::Deserialize;
    use std::error::Error;
    use std::path::{Path, PathBuf};

    #[derive(Deserialize, Debug)]
    pub struct Config {
        issues_path: PathBuf,

        project: ProjectConfig,
        statuses: Vec<StatusConfig>,

        features_path: Option<PathBuf>,
    }

    impl Config {
        pub async fn reify_paths(&mut self, root_dir: &Path) -> Result<(), Box<dyn Error>> {
            self.issues_path = root_dir.join(&self.issues_path);
            fs::create_dir_all(&self.issues_path).await?;
            self.issues_path = self.issues_path.canonicalize()?;

            Ok(())
        }
    }

    #[derive(Deserialize, Debug)]
    pub struct StatusConfig {
        name: String,
        resolved: bool,
    }

    #[derive(Deserialize, Debug)]
    pub struct ProjectConfig {
        name: String,
        key: String,
    }
}

pub mod command {
    use clap::{AppSettings, Clap};

    #[derive(Clap, Debug)]
    #[clap(setting = AppSettings::ColoredHelp)]
    #[clap(
        version = "0.0",
        author = "Freddie Gilbraith <freddie.gilbraith@littlebonsai.co.uk>"
    )]
    pub struct Root {
        #[clap(
            short,
            long,
            about = "Manually provide a path to an override config file"
        )]
        config: Option<std::path::PathBuf>,

        #[clap(long, about = "Don't apply any changes, only display them and quit")]
        dry_run: bool,

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
}
