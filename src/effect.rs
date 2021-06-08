use super::project::Project;
use super::ticket;
use uuid::Uuid;

pub struct Effect<'project> {
    pub task_id: Uuid,
    pub project: &'project Project,
    pub kind: EffectKind,
}

pub enum EffectKind {
    CreateTicket {
        title: String,
        ticket_type: ticket::TicketType,
        reporter: ticket::Person,
    },
    StageTicket,
    MakeCommit {
        message: String,
    },
}

impl<'project> Effect<'project> {
    pub async fn create_stringified_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let stringified_action = match &self.kind {
            EffectKind::StageTicket => {
                format!("stage the ticket '{}' to be committed", self.task_id)
            }

            EffectKind::MakeCommit { message } => {
                format!("make a commit with the message '{}'", message)
            }

            EffectKind::CreateTicket {
                title,
                ticket_type,
                reporter,
            } => format!(
                "create a {} task with the title '{}' reported by '{}'",
                ticket_type, title, reporter
            ),
        };

        Ok(format!("[{}]: {}", self.task_id, stringified_action))
    }
}
