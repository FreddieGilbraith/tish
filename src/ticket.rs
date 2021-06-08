use clap::Clap;
use uuid::Uuid;

pub struct Ticket {
    id: Uuid,
    title: String,
    description: String,
    ticket_type: TicketType,
    status: Status,

    test_cmd: Option<String>,
    feature: Option<String>,

    priority: Option<u64>,

    reporter: Person,
    assignee: Option<Person>,

    time: Option<TicketTime>,
    links: Option<TicketLinks>,

    comments: Vec<TicketComment>,
}

pub struct TicketLinks {
    parent: Option<TicketLink>,
    children: Option<Vec<TicketLink>>,
    blocks: Option<Vec<TicketLink>>,
    depends: Option<Vec<TicketLink>>,
    duplicate: Option<Vec<TicketLink>>,
}

pub struct TicketLink {
    id: Uuid,
    ticket_type: TicketType,
    title: String,
}

#[derive(Clap, Debug, Clone)]
pub enum TicketType {
    Epic,
    Story,
    Task,
    Bug,
}

use std::fmt;

impl fmt::Display for TicketType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TicketType::Epic => "Epic",
                TicketType::Story => "Story",
                TicketType::Task => "Task",
                TicketType::Bug => "Bug",
            }
        )
    }
}

impl std::str::FromStr for TicketType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "epic" => Ok(TicketType::Epic),
            "story" => Ok(TicketType::Story),
            "task" => Ok(TicketType::Task),
            "bug" => Ok(TicketType::Bug),
            _ => Err("not a valid ticket type"),
        }
    }
}

pub struct TicketTime {
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

pub struct TicketComment {
    text: String,
    author: Person,
}

pub struct Person {
    pub name: String,
    pub email: String,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}
