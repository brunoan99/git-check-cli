use std::fs;
use std::path::PathBuf;
use std::process;
use yaml_rust::YamlLoader;

use clap::Parser;
use git_check_cli::{Cli, Commands};

fn path_of_project_list_file(home_path: PathBuf) -> String {
  let mut project_file_paths = home_path;
  project_file_paths.push(".config/git-check-cli/projects");
  project_file_paths.set_extension("yaml");
  project_file_paths.into_os_string().into_string().unwrap()
}

fn main() {
  let cli = Cli::parse();

  let home_path = home::home_dir().unwrap_or_else(|| {
    eprintln!("Problem finding home directory!");
    process::exit(1);
  });

  let projects_file = path_of_project_list_file(home_path);

  let contents = fs::read_to_string(projects_file).unwrap_or_else(|err| {
    eprintln!(
      "Problem reading projects list file, the op gives error: {}",
      err
    );
    process::exit(1);
  });

  let projects = YamlLoader::load_from_str(&contents).unwrap_or_else(|err| {
    eprintln!("Problem parsing file, error: {}", err);

    process::exit(1);
  });
  let project = &projects[0];

  println!("{:#?}", &project["project-list"][0]);

  match &cli.command {
    Commands::Check => {
      //projects_file
    }
    Commands::CheckPath { name: _ } => {
      //
    }
    Commands::AddPath { path: _ } => {
      //
    }
    Commands::RemovePath { path: _ } => {
      //
    }
  }
}
