use super::command;
use super::effect;
use super::ticket;

use async_std::fs;
use git2::Repository;
use normpath::PathExt;
use serde::Deserialize;
use std::error::Error;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Config {
    tickets_path: PathBuf,

    project: ProjectConfig,

    features_path: Option<PathBuf>,
}

impl Config {
    pub async fn reify_paths(&mut self, root_dir: &Path) -> Result<(), Box<dyn Error>> {
        self.tickets_path = root_dir.join(&self.tickets_path);
        fs::create_dir_all(&self.tickets_path).await?;
        self.tickets_path = self.tickets_path.canonicalize()?;

        Ok(())
    }
}

#[derive(Deserialize, Debug)]
pub struct ProjectConfig {
    name: String,
}

pub struct Project {
    config: Config,
    repo: Repository,
}

impl Project {
    pub fn new(config: Config, repo: Repository) -> Self {
        Self { config, repo }
    }

    pub async fn generate_effects<'a>(
        &'a self,
        command: &command::Root,
    ) -> Result<Vec<effect::Effect<'a>>, Box<dyn Error>> {
        match &command.command {
            command::Command::Create(create_opts) => {
                self.generate_effects_create(create_opts).await
            }
            _ => Err("not yet implemented".into()),
        }
    }

    pub async fn generate_effects_create<'a>(
        &'a self,
        create_opts: &command::CreateOpts,
    ) -> Result<Vec<effect::Effect<'a>>, Box<dyn Error>> {
        let task_id = Uuid::new_v4();

        let message = format!(
            "[tish | {}] Created new {} \"{}\"",
            &task_id, create_opts.ticket_type, create_opts.title
        );

        let name = self.repo.config()?.get_string("user.name")?;
        let email = self.repo.config()?.get_string("user.email")?;
        let reporter = ticket::Person { name, email };

        let mut effect_chain = vec![effect::Effect {
            task_id,
            project: &self,
            kind: effect::EffectKind::CreateTicket {
                reporter,
                title: create_opts.title.clone(),
                ticket_type: create_opts.ticket_type.clone(),
            },
        }];

        effect_chain.push(effect::Effect {
            task_id,
            project: &self,
            kind: effect::EffectKind::StageTicket,
        });

        effect_chain.push(effect::Effect {
            task_id,
            project: &self,
            kind: effect::EffectKind::MakeCommit { message },
        });

        Ok(effect_chain)
    }

    pub async fn apply_effects<'a>(
        &'a self,
        effects: &[effect::Effect<'a>],
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
