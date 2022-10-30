use app_cmd::app_cmd;

mod app_cmd;
mod exec;
pub mod error;
pub mod utils;
pub mod prelude;
mod react;
mod react_native;
mod tauri;

mod target;
mod node_modules;

mod run_dev;

use crate::prelude::*;
use run_dev::run_dev;

pub use app_cmd::VERSION;

fn main() {
	match cmd_run() {
		Ok(_) => (),
		Err(err) => println!("FAIL - {err}"),
	}
}

fn cmd_run() -> Result<()> {
	let app = app_cmd().get_matches();

	match app.subcommand() {
        Some(("react", sub_cmd)) => {
            match react::new(sub_cmd) {
                Ok(()) => (),
                Err(_) => println!("Hmm, looks like something went wrong...")
            }
        },
        Some(("rn", sub_cmd)) => {
            match react_native::new(sub_cmd) {
                Ok(()) => (),
                Err(_) => println!("Hmm, looks like something went wrong...")
            }
        },
        Some(("tauri", sub_cmd)) => {
            match tauri::new(sub_cmd) {
                Ok(()) => (),
                Err(_) => println!("Hmm, looks like something went wrong...")
            }
        },
        Some(("dev", sub_cmd)) => {
            println!("FKIT DEV entered!");
            match run_dev(sub_cmd) {
                Ok(()) => (),
                Err(_) => println!("Hmm, looks like something went wrong...")
            }
        },
        Some(("target", sub_cmd)) => {
            match target::run(sub_cmd) {
                Ok(()) => (),
                Err(_) => println!("Hmm, looks like something went wrong...")
            }
        },
        Some(("node", sub_cmd)) => {
            match node_modules::run(sub_cmd) {
                Ok(()) => (),
                Err(_) => println!("Hmm, looks like something went wrong...")
            }
        },
		_ => {
			// needs cmd_app version as the orginal got consumed by get_matches
			app_cmd().print_long_help()?;
			println!("\n");
		}
	}

	Ok(())
}