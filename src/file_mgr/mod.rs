use std::io::{Error, ErrorKind};

pub fn read_file(path: &str) -> Result<String, Error> {
  std::fs::read_to_string(path)
}

pub fn read_yaml(path: &str) -> Result<Vec<yaml_rust::Yaml>, Error> {
  let content = match read_file(path) {
    Ok(content) => content,
    Err(err) => {
      return Err(Error::new(
        ErrorKind::InvalidData,
        format!("Error getting file contant: {err}"),
      ));
    }
  };
  match yaml_rust::YamlLoader::load_from_str(&content) {
    Ok(res) => Ok(res),
    Err(err) => Err(Error::new(
      ErrorKind::InvalidInput,
      format!("Error parsing file: {err}"),
    )),
  }
}

pub fn write_yaml_file(path: &str, value: &yaml_rust::Yaml) -> Result<(), Error> {
  let mut out_str = String::new();
  let mut emitter = yaml_rust::YamlEmitter::new(&mut out_str);
  if let Err(err) = emitter.dump(value) {
    return Err(Error::new(
      ErrorKind::InvalidInput,
      format!("Failed to write yaml: {err}"),
    ));
  }
  if let Err(err) = std::fs::write(path, out_str) {
    return Err(Error::new(
      ErrorKind::InvalidInput,
      format!("failed to sync with file, err: {err}"),
    ));
  }
  Ok(())
}
