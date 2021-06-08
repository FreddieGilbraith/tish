use super::project::Project;
use uuid::Uuid;

pub struct Effect<'project> {
    pub task_id: Uuid,
    pub project: &'project Project,
    pub kind: EffectKind,
}

pub enum EffectKind {
    CreateTask { title: String, description: String },
    MakeCommit { message: String },
}

impl<'project> Effect<'project> {
    pub async fn create_stringified_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let stringified_action = match &self.kind {
            EffectKind::MakeCommit { message } => {
                format!("make a commit with the message '{}'", message)
            }

            EffectKind::CreateTask { title, description } => format!(
                "create a task with the title '{}' and description '{}'",
                title, description
            ),
        };

        Ok(format!("[{}]: {}", self.task_id, stringified_action))
    }
}
