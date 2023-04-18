use colored::Colorize;
use mylib::file_tracker::Tracker;
use mylib::{cli, file_tracker};
use std::path::PathBuf;
use std::process;
use std::{ffi::OsString, fs};
use yaml_rust::YamlLoader;

// proceess

fn path_of_projects_list_file(home_path: PathBuf) -> Result<String, OsString> {
  let mut project_file_paths = home_path;
  project_file_paths.push(".config/git-check-cli/git-check-cli");
  project_file_paths.set_extension("yaml");
  project_file_paths.into_os_string().into_string()
}

// main

fn setup_config() -> (String, file_tracker::Tracker) {
  let home_path = home::home_dir().unwrap_or_else(|| {
    eprintln!("Error finding home directory!");
    process::exit(1);
  });

  let config_path = path_of_projects_list_file(home_path).unwrap_or_else(|_| {
    eprintln!("Error getting projects file path");
    process::exit(1)
  });

  let contents = fs::read_to_string(config_path.clone()).unwrap_or_else(|err| {
    eprintln!("Error reading projects list file, error: {err}");
    process::exit(1);
  });

  let configs_yaml = YamlLoader::load_from_str(&contents).unwrap_or_else(|err| {
    eprintln!("Error parsing file, error: {err}");
    process::exit(1);
  });

  let tracker = Tracker::try_from(configs_yaml).unwrap_or_else(|err| {
    eprintln!("{err}");
    process::exit(1);
  });

  (config_path, tracker)
}

fn main() {
  println!("{}", "Starting git-check-cli".bold());

  let (configs_path, mut tracker) = setup_config();

  cli::run_cli(&mut tracker, configs_path);
}
