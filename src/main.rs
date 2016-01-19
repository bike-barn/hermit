#![warn(missing_docs)]
#[macro_use]
extern crate clap;
extern crate git2;
extern crate uuid;

mod config;
mod hermit;
mod shell;
mod file_operations;

#[macro_use]
mod macros;

use std::env;
use std::error::Error;
use std::path::PathBuf;

use clap::{App, Arg, AppSettings, SubCommand};

use config::FsConfig;
use hermit::Hermit;
use file_operations::FileOperations;

#[cfg(test)]
mod test;

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

    report_errors(file_operations.commit());
}

fn report_errors(results: Vec<file_operations::Result>) {
    let app_name = get_program_name();

    for result in results {
        match result {
            Ok(()) => (),
            Err(e) => println!("{}: error: {}", app_name, e.description()),
        }
    }
}

fn get_program_name() -> String {
    env::args()
        .nth(0)
        .map(PathBuf::from)
        .and_then(|path| path.file_name().map(|name| name.to_owned()))
        .and_then(|file_name| file_name.to_str().map(|name| name.to_owned()))
        .unwrap_or("hermit".to_owned())
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

fn make_app_config<'a, 'b, 'c, 'd, 'e, 'f>() -> App<'a, 'b, 'c, 'd, 'e, 'f> {
    let version = env!("CARGO_PKG_VERSION");

    let app = App::new("hermit")
                  .version(version)
                  .author("Bike Barn <https://github.com/bike-barn/hermit>")
                  .about("A home directory configuration management assistant.")
                  .setting(AppSettings::SubcommandRequiredElseHelp)
                  .setting(AppSettings::VersionlessSubcommands);

    let app = configure_add(app);
    let app = configure_clone(app);
    let app = configure_doctor(app);
    let app = configure_git(app);
    let app = configure_init(app);
    let app = configure_nuke(app);
    let app = configure_status(app);
    let app = configure_use(app);

    app
}

configure_subcommand!(add, configure_add {
   about("Add files to your hermit shell")
});

configure_subcommand!(clone, configure_clone {
    about("Create a local shell from an existing remote shell")
});

configure_subcommand!(doctor, configure_doctor {
    about("Make sure your hermit setup is sane")
});

configure_subcommand!(git, configure_git {
    about("Run git operations on the current shell")
});

configure_subcommand!(init, configure_init {
    about("Create a new hermit shell called SHELL_NAME. If no shell name \
           is given, \"default\" is used.");
    arg(Arg::with_name("SHELL_NAME")
        .help("The name of the shell to be created."))
});

configure_subcommand!(nuke, configure_nuke {
    about("Permanently remove a hermit shell")
});

configure_subcommand!(status, configure_status {
    about("Display the status of your hermit shell")
});

configure_subcommand!(use, configure_use {
    about("Switch to using a different hermit shell")
});
