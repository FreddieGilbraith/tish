use git2::Repository;
use std::path::{Path, PathBuf};
use tish::command;

fn recursive_get_repo(maybe_dir: Option<&Path>) -> Repository {
    let dir = maybe_dir.expect("could not find git repo in this directory or parents");

    match Repository::open(dir) {
        Ok(repo) => repo,
        Err(e) => recursive_get_repo(dir.parent()),
    }
}

fn main() {
    let repo = recursive_get_repo(Some(&std::env::current_dir().unwrap()));

    let root_dir = repo
        .workdir()
        .expect("could not open repo working directory");

    let config_file_path = root_dir.join("tish.toml");

    dbg!(&config_file_path);

    let tish_command = command::cli_parse();

    dbg!(tish_command);
}
