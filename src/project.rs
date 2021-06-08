use super::command;
use super::effect;
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
}

impl Project {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn generate_effects<'a>(
        &'a self,
        commands: &command::Root,
    ) -> Result<Vec<effect::Effect<'a>>, Box<dyn Error>> {
        Ok(vec![])
    }

    pub async fn apply_effects<'a>(
        &'a self,
        effects: &[effect::Effect<'a>],
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
