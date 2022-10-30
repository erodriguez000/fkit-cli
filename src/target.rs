use std::process::{Stdio, Command};
use clap::ArgMatches;
use colored::Colorize;
use crate::prelude::*;

pub fn run(sub_cmd: &ArgMatches) -> Result<()> {
    match sub_cmd.subcommand() {
        Some(("get", _sub_cmd)) => {
            println!("{}", "Searching for target directories!".blue().bold());
            get();
        },
        Some(("delete", _sub_cmd)) => {
            println!("{}", "Searching and deleting target directories!".red().bold());
            _delete();
        },
        _ => println!("Hmm, I don't recognize that command...")
    }
    Ok(())
}

fn get() {
    let child = Command::new("find")
        .args([".", "-name", "target", "-type", "dir", "-prune"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    Command::new("xargs")
        .args(["du", "-chs"])
        .stdin(child.stdout.unwrap())
        .spawn()
        .unwrap()
        .wait_with_output()
        .unwrap();
}
fn _delete() {
    Command::new("find")
        .args([".", "-name", "node_modules", "-type", "dir", "-prune", "-exec", "rm", "rf"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
}
