use async_std::fs;
use async_std::prelude::*;
use git2::Repository;
use std::error::Error;
use std::path::{Path, PathBuf};

use tish;

fn recursive_get_repo(maybe_dir: Option<&Path>) -> Repository {
    let dir = maybe_dir.expect("could not find git repo in this directory or parents");

    match Repository::open(dir) {
        Ok(repo) => repo,
        Err(_) => recursive_get_repo(dir.parent()),
    }
}

fn generate_possible_config_paths(root_dir: &Path) -> Vec<PathBuf> {
    vec![
        root_dir.join("tish.toml"),
        root_dir.join(".tish.toml"),
        root_dir.join("config").join("tish.toml"),
        root_dir.join(".config").join("tish.toml"),
        root_dir.join("tish").join("config.toml"),
        root_dir.join(".tish").join("config.toml"),
    ]
}

async fn try_to_open_config(root_dir: &Path) -> Result<tish::project::Config, Box<dyn Error>> {
    let config_file_paths = generate_possible_config_paths(&root_dir);

    for path in config_file_paths {
        match fs::read_to_string(path).await {
            Ok(file_as_str) => {
                let mut config: tish::project::Config = toml::from_str(&file_as_str)?;
                config.reify_paths(root_dir).await?;
                return Ok(config);
            }
            Err(_) => {
                continue;
            }
        }
    }

    Err("Could not find a config file in any of the valid paths".into())
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let repo = recursive_get_repo(Some(&std::env::current_dir().unwrap()));

    let root_dir = repo
        .workdir()
        .expect("could not open repo working directory");

    let config: tish::project::Config = try_to_open_config(&root_dir).await?;

    dbg!(&config);

    let tish_command = tish::command::cli_parse();

    Ok(())
}
