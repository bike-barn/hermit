#![warn(missing_docs)]
#[macro_use]
extern crate clap;
extern crate git2;
extern crate uuid;

mod config;
mod hermit;
mod shell;
mod file_operations;

use std::env;
use std::path::PathBuf;

use clap::App;

use config::FsConfig;
use hermit::Hermit;
use file_operations::FileOperations;

#[cfg(test)]
mod test;

fn make_app_config<'a, 'b, 'c, 'd, 'e, 'f>() -> App<'a, 'b, 'c, 'd, 'e, 'f> {
    let version = env!("CARGO_PKG_VERSION");

    clap_app!(myapp =>
        (version: version)
        (author: "Bike Barn <https://github.com/bike-barn/hermit>")
        (about: "A home directory configuration management assistant.")
        (@setting SubcommandRequiredElseHelp)
        (@setting VersionlessSubcommands)
        (@subcommand add =>
            (usage: "hermit add [<filepattern>…]")
            (about: "Add files to your hermit shell"))
        (@subcommand clone =>
            (usage: "hermit clone <repository> [shell-name]")
            (about: "Create a local shell from an existing remote shell"))
        (@subcommand doctor =>
            (usage: "hermit doctor")
            (about: "Make sure your hermit setup is sane"))
        (@subcommand git =>
            (usage: "hermit git <git arguments>")
            (about: "Run git operations on the current shell"))
        (@subcommand init =>
            (usage: "hermit init [SHELL_NAME]")
            (about: "Create a new hermit shell called SHELL_NAME. If no shell name is given, \"default\" is used.")
            (@arg SHELL_NAME: "The name of the shell to be created."))
        (@subcommand nuke =>
            (usage: "hermit nuke <shell-name>")
            (about: "Permanently remove a hermit shell"))
        (@subcommand status =>
            (usage: "hermit status")
            (about: "Display the status of your hermit shell"))
        (@subcommand use =>
            (usage: "hermit use [shell-name]")
            (about: "Switch to using a different hermit shell"))
    )
}

fn main() {
    let app = make_app_config();
    let app_matches = app.get_matches();

    let hermit_root = get_hermit_dir().expect("Could not determine hermit root location.");
    let fs_config = FsConfig::new(hermit_root);
    let hermit = Hermit::new(fs_config);

    let home_dir = env::home_dir().expect("Could not determine home directory.");
    let mut file_operations = FileOperations::rooted_at(home_dir);

    match app_matches.subcommand() {
        ("add", Some(_matches)) => {
            println!("hermit add is not yet implemented");
        }
        ("clone", Some(_matches)) => {
            println!("hermit clone is not yet implemented");
        }
        ("doctor", Some(_matches)) => {
            println!("hermit doctor is not yet implemented");
        }
        ("git", Some(_matches)) => {
            println!("hermit git is not yet implemented");
        }
        ("init", Some(matches)) => {
            let shell_name = matches.value_of("SHELL_NAME").unwrap_or("default");
            hermit.init_shell(&mut file_operations, shell_name);
        }
        ("nuke", Some(_matches)) => {
            println!("hermit nuke is not yet implemented");
        }
        ("status", Some(_matches)) => {
            println!("hermit status is not yet implemented");
        }
        ("use", Some(_matches)) => {
            println!("hermit use is not yet implemented");
        }
        _ => {}
    };

    file_operations.commit();
}

fn get_hermit_dir() -> Option<PathBuf> {
    env::var("HERMIT_ROOT")
        .map(PathBuf::from)
        .ok()
        .or_else(default_hermit_dir)
}

fn default_hermit_dir() -> Option<PathBuf> {
    env::home_dir().map(|home| home.join(".config/hermit"))
}
