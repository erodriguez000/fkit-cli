use clap::ArgMatches;
use crate::prelude::*;
use crate::exec;

pub fn run_dev(_sub_cmd: &ArgMatches) -> Result<()> {
    exec::spawn_and_wait(
		None,
		"npm",
		&["run", "fkit"],
		true,
	)?;
    Ok(())
}