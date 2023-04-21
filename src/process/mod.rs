use std::process::{Command, Output};

fn remove_break_line(output: &mut Output) {
  if output.stdout.ends_with(&[10]) && !output.stdout.is_empty() {
    output.stdout.remove(output.stdout.len() - 1);
  }
}

fn get_stdout_as_string(output: &Output) -> String {
  let str_stdout = std::str::from_utf8(&output.stdout).unwrap();
  str_stdout.into()
}

fn _get_stderr_as_string(output: &Output) -> String {
  let str_stderr = std::str::from_utf8(&output.stderr).unwrap();
  str_stderr.into()
}

pub fn eval_to_absolute_path(exp: &str) -> String {
  if exp.contains('$') {
    let exp_to_eval = format!("/bin/echo {exp}");
    let mut output = Command::new("sh")
      .args(["-c", &exp_to_eval])
      .output()
      .unwrap();
    remove_break_line(&mut output);
    let path = get_stdout_as_string(&output);
    path.into()
  } else {
    exp.into()
  }
}
