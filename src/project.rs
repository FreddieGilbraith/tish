use super::command;
use super::effect;
use super::issue;

use async_std::fs;
use git2::Repository;
use normpath::PathExt;
use serde::Deserialize;
use std::error::Error;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Config {
    issues_path: PathBuf,

    project: ProjectConfig,

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
            "Created new {:?} \"{}\" [{}]",
            create_opts.issue_type, create_opts.title, &task_id
        );

        Ok(vec![effect::Effect {
            task_id,
            project: &self,
            kind: effect::EffectKind::MakeCommit { message },
        }])
    }

    pub async fn apply_effects<'a>(
        &'a self,
        effects: &[effect::Effect<'a>],
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
