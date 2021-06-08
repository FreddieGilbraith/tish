use super::project::Project;
use uuid::Uuid;

pub struct Effect<'project> {
    task_id: Uuid,
    project: &'project Project,
    kind: EffectKind,
}

impl<'project> std::fmt::Display for Effect<'project> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.task_id)
    }
}

pub enum EffectKind {
    CreateTask { title: String, description: String },
}
