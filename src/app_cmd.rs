use clap::{crate_version, Arg, Command};

pub const VERSION: &str = crate_version!();

pub fn app_cmd() -> Command {
    Command::new("fkit")
        .version(VERSION)
        .about("Testing")
        .subcommand(react())
        .subcommand(react_native())
        .subcommand(tauri())
        .subcommand(target())
        .subcommand(node())
		.subcommand(sub_dev())
}

// region:      --- React

fn react() -> Command {
	Command::new("react")
		.about("Build new react app from template base")
		.arg(Arg::new("name").help("React project name (no spaces!)"))
}
// endregion:   --- React

// region:      --- React Native
fn react_native() -> Command {
	Command::new("rn")
		.about("Build new react-native app from template base")
		.arg(Arg::new("name").help("React Native project name (no spaces!)"))
}
// endregion:   --- React Native

// region:      --- Tauri
fn tauri() -> Command {
	Command::new("tauri")
		.about("Build new tauri app from template base")
		.arg(Arg::new("name").help("Tauri project name (no spaces!)"))
}
// endregion:   --- Tauri

// region:      --- Target Cleanup

fn target() -> Command {
	Command::new("target")
		.about("Manage your Cargo target directories")
		.subcommand(get_target())
		.subcommand(delete_target())
}

fn get_target() -> Command {
    Command::new("get")
        .about("Get your Cargo target directories")
}

fn delete_target() -> Command {
    Command::new("delete")
        .about("Get your Cargo target directories")
}

// endregion:   --- Target Cleanup

// region:      --- node_modules Cleanup

fn node() -> Command {
	Command::new("node")
		.about("Manage your node_module directories")
		.subcommand(get_node())
		.subcommand(delete_node())
}

fn get_node() -> Command {
    Command::new("get")
        .about("Get your node_module directories")
}

fn delete_node() -> Command {
    Command::new("delete")
        .about("Delete your node_module directories")
}


// endregion:   --- node_modules Cleanup

fn sub_dev() -> Command {
    Command::new("dev").about("Starts hot-reload developement")
}
