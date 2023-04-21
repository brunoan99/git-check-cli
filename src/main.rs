use mylib::{cli, file_mgr, file_tracker, process};

fn setup_config() -> (String, file_tracker::Tracker) {
  let path = "$HOME/.config/git-check-cli/git-check-cli.yaml";
  let config_path = process::eval_to_absolute_path(path);

  let yaml_config = match file_mgr::read_yaml(&config_path) {
    Ok(content) => content,
    Err(err) => {
      eprintln!("{err}");
      std::process::exit(1);
    }
  };

  let tracker = match file_tracker::Tracker::try_from(&yaml_config[0]) {
    Ok(tracker) => tracker,
    Err(err) => {
      eprintln!("{err}");
      std::process::exit(1);
    }
  };

  (config_path.into(), tracker)
}

fn main() {
  let (configs_path, mut tracker) = setup_config();
  cli::run(&mut tracker, configs_path);
}
