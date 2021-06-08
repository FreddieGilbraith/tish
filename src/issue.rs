use clap::Clap;
use uuid::Uuid;

pub struct Issue {
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

#[derive(Clap, Debug)]
pub enum IssueType {
    Epic,
    Story,
    Task,
    Bug,
}

impl std::str::FromStr for IssueType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "epic" => Ok(IssueType::Epic),
            "story" => Ok(IssueType::Story),
            "task" => Ok(IssueType::Task),
            "bug" => Ok(IssueType::Bug),
            _ => Err("not a valid issue type"),
        }
    }
}

pub struct IssueTime {
    estimate: u64,
    spent: u64,
}

pub enum Status {
    Backlog,
    Todo,
    InProgress,
    Done,
    Rejected,
}

pub struct IssueComment {
    text: String,
    author: Person,
}

pub struct Person {
    name: String,
    email: String,
}
