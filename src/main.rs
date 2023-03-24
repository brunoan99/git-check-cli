use inquire::Select;
use std::path::PathBuf;
use std::process;
use std::{ffi::OsString, fs};
use yaml_rust::YamlLoader;

use clap::Parser;
use git_check_cli::{Cli, Commands};

fn path_of_project_list_file(home_path: PathBuf) -> Result<String, OsString> {
  let mut project_file_paths = home_path;
  project_file_paths.push(".config/git-check-cli/git-check-cli");
  project_file_paths.set_extension("yaml");
  project_file_paths.into_os_string().into_string()
}

fn main() {
  println!("Starting git-check-cli");

  let cli = Cli::parse();

  let home_path = home::home_dir().unwrap_or_else(|| {
    eprintln!("Problem finding home directory!");
    process::exit(1);
  });

  let projects_file = path_of_project_list_file(home_path).unwrap_or_else(|_| {
    eprintln!("Problem getting projects file path");
    process::exit(1)
  });

  let contents = fs::read_to_string(projects_file).unwrap_or_else(|err| {
    eprintln!("Problem reading projects list file, error: {}", err);
    process::exit(1);
  });

  let _configs = &(YamlLoader::load_from_str(&contents).unwrap_or_else(|err| {
    eprintln!("Problem parsing file, error: {}", err);
    process::exit(1);
  }))[0];

  // println!("{:#?}", configs["config"]);
  // println!("{:#?}", configs["project-list"]);

  match &cli.command {
    Commands::Check => {
      // projects_file
    }
    Commands::CheckPath { name: _ } => {
      //
    }
    Commands::AddPath { path: _, name: _ } => {
      //
    }
    Commands::RemovePath {} => {
      let options: Vec<&str> = vec!["name", "path"];

      let ans = Select::new("remove by: ", options)
        .with_help_message("text message")
        .prompt();

      match ans {
        Ok("name") => println!("name! was chosen"), // TODO: another Select prompt to select which project by name to remove
        Ok("path") => println!("path! was chosen"), // TODO: another Select prompt to select which project by path to remove
        Ok(_) => println!("There was an error, please try again"),
        Err(_) => println!("There was an error, please try again"),
      }
      //
    }
  }
}
