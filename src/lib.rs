mod issue {
    use uuid::Uuid;
    pub struct Id(u64);

    pub struct RootIssue {
        id: Id,
        uuid: Uuid,
        status: Status,
        reporter: Author,

        title: String,
        description: String,

        comments: Vec<IssueComment>,
    }

    pub struct Status {
        resolved: bool,
        name: String,
    }

    pub struct IssueComment {
        text: String,
        author: Author,
    }

    pub struct Author {
        name: String,
        email: String,
    }
}

mod project {
    use super::issue;

    struct Config {
        project: ProjectConfig,
        statuses: Vec<issue::Status>,
    }

    ProjectConfig {
        name: String,
        key: String,
    }
}

pub mod command {
    use clap::Clap;

    #[derive(Clap, Debug)]
    pub struct Root {
        #[clap(subcommand)]
        command: Command,
    }

    #[derive(Clap, Debug)]
    pub enum Command {
        Create(CreateOpts),
    }

    #[derive(Clap, Debug)]
    pub struct CreateOpts {
        #[clap(short)]
        title: String,
    }

    pub fn cli_parse() -> Root {
        Root::parse()
    }
}
