use aho_corasick::AhoCorasick;
use clap::ArgMatches;
use colored::Colorize;
use std::{fs, path::Path, str::from_utf8};

use crate::exec::{prompt, spawn_and_wait};
use crate::prelude::*;
use crate::utils;

const DEFAULT_APP_NAME: &str = "my-tauri-app";

const FILES: &[&str; 2] = &["package.json", "src-tauri/tauri.conf.json"];

const GIT_TMPL_BASE: &str = "https://github.com/erodriguez000/my-tauri-app.git";

struct Conf<'a> {
    app_name: &'a str,
}

pub fn new(sub_cmd: &ArgMatches) -> Result<()> {
    // --- Get the name
    let app_name = sub_cmd.get_one::<String>("name");

    let app_name = match app_name {
        Some(name) => name.to_string(),
        None => prompt(
            &format!("What is your app name? ({DEFAULT_APP_NAME}): "),
            Some(DEFAULT_APP_NAME),
        )?,
    };

    let app_dir_name = (&app_name).to_string().to_lowercase();

    let app_dir = Path::new(&app_dir_name);

    // check if the dir already exist
    if app_dir.exists() {
        println!(
            "{}",
            "Directory already exists! Try a new name".red().bold()
        );
        return Ok(());
    }
    // --- Do the git clone
    // git clone --depth 1 --branch <tag_name> <repo_url>
    let prompt = "Creating your Tauri app...".cyan();
    println!("{}", prompt);
    println!("{}", "...".cyan());
    println!("{}", "...".cyan());
    println!("{}", "...".cyan());

    spawn_and_wait(
        None,
        "git",
        &[
            "clone",
            "--depth",
            "1",
            "--branch",
            "master",
            GIT_TMPL_BASE,
            &app_dir_name,
        ],
        false,
    )?;

    // --- Replace the parts

    let prompt = "Configuring your Tauri app...".magenta();
    println!("{}", prompt);
    println!("{}", "...".magenta());
    println!("{}", "...".magenta());
    println!("{}", "...".magenta());

    replace_parts(
        app_dir,
        Conf {
            app_name: &app_name,
        },
    )?;

    // Open in VS Code

    let prompt = "Tauri app initialized! Opening in VS Code".blue();
    println!("{}", prompt);
    println!("{}", "...".blue());
    println!("{}", "...".blue());
    println!("{}", "...".blue());

    spawn_and_wait(None, "code", &[&app_dir_name], false)?;

    let prompt = "Done! Next steps...".bright_magenta();
    println!("{}", prompt);
    println!("{}", "> Open terminal in VScode...".bright_magenta());
    println!("{}", "> npm i".bright_magenta());
    println!("{}", "> fkit dev".bright_magenta());

    Ok(())
}

fn replace_parts(dir: &Path, conf: Conf) -> Result<()> {
    let files = FILES
        .iter()
        .map(|f| utils::path_joins(dir, f))
        .collect::<Vec<_>>();
    let patterns = &[DEFAULT_APP_NAME];
    let ac = AhoCorasick::new(patterns);
    let replace_by = &[conf.app_name];

    for file in files {
        let content = fs::read_to_string(&file)?;
        let res = ac.replace_all_bytes(content.as_bytes(), replace_by);
        let new_content = from_utf8(&res).unwrap();

        if content != new_content {
            println!("File updated: '{}'", file.to_string_lossy());
            fs::write(file, new_content)?;
        } else {
            println!(
                "File skipped (nothing changed): '{}'",
                file.to_string_lossy()
            );
        }
    }
    Ok(())
}
