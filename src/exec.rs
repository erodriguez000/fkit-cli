use std::{io::Write, path::Path};
use crate::prelude::Result;
use std::process::Command;

pub fn prompt(message: &str, default: Option<&str>) -> Result<String> {
    print!("{message}");
    let _ = std::io::stdout().flush();

    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf)?;

    let val = buf.trim();

    let val = match (val.is_empty(), default) {
        (true, Some(default)) => default,
        (false, _) => val,
        (true, None) => val, // return the empty string (TODO: might want to return error)
    };

    Ok(val.to_string())
}

pub fn spawn_and_wait(cwd: Option<&Path>, cmd_str: &str, args: &[&str], print_exec: bool) -> Result<()> {
	let mut cmd = build_cmd(cwd, cmd_str, args);

	if print_exec {
		println!("> executing: {} {}", cmd_str, args.join(" "));
	}

	match cmd.spawn()?.wait() {
		Ok(status) => {
			if !status.success() {
				// Err((cmd_str, args, status).into())
                Ok(())
			} else {
				Ok(())
			}
		}
		Err(ex) => Err(ex.into()),
	}
}

pub fn build_cmd(cwd: Option<&Path>, cmd: &str, args: &[&str]) -> Command {
	let mut cmd = Command::new(cmd);
	if let Some(cwd) = cwd {
		cmd.current_dir(cwd);
	}
	cmd.args(args);
	cmd
}
