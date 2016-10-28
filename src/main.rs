#![warn(missing_docs)]
#[macro_use]
extern crate clap;
extern crate git2;
extern crate uuid;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

mod config;
mod env;
mod hermit;
mod message;
mod shell;
mod file_operations;

#[macro_use]
mod macros;

use std::error::Error;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use config::{Config, FsConfig};
use hermit::Hermit;
use file_operations::FileOperations;

#[cfg(test)]
mod test_helpers;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let app = make_app_config();
    let app_matches = app.get_matches();

    let hermit_root = env::get_hermit_dir().expect("Could not determine hermit root location.");
    let fs_config = FsConfig::new(hermit_root);
    let mut hermit = Hermit::new(fs_config);

    let home_dir = env::home_dir().expect("Could not determine home directory.");
    let mut file_operations = FileOperations::rooted_at(home_dir);

    match app_matches.subcommand() {
        ("add",    Some(matches)) => handle_add    (matches, &mut hermit, &mut file_operations),
        ("clone",  Some(matches)) => handle_clone  (matches, &mut hermit, &mut file_operations),
        ("doctor", Some(matches)) => handle_doctor (matches, &mut hermit, &mut file_operations),
        ("git",    Some(matches)) => handle_git    (matches, &mut hermit, &mut file_operations),
        ("init",   Some(matches)) => handle_init   (matches, &mut hermit, &mut file_operations),
        ("nuke",   Some(matches)) => handle_nuke   (matches, &mut hermit, &mut file_operations),
        ("status", Some(matches)) => handle_status (matches, &mut hermit, &mut file_operations),
        ("use",    Some(matches)) => handle_use    (matches, &mut hermit, &mut file_operations),
        _ => unreachable!(message::error("unknown subcommand passed"))
    };

    report_errors(file_operations.commit());
}

fn report_errors(results: Vec<file_operations::Result>) {
    for result in results {
        match result {
            Ok(()) => (),
            Err(e) => println!("{}", message::error(e.description())),
        }
    }
}

fn make_app_config<'a, 'b, 'c, 'd, 'e, 'f>() -> App<'a, 'b, 'c, 'd, 'e, 'f> {
    let version = env!("CARGO_PKG_VERSION");

    let app = App::new("hermit")
        .version(version)
        .author("Bike Barn <https://github.com/bike-barn/hermit>")
        .about("A home directory configuration management assistant.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands);

    let app = add_add_subcommand(app);
    let app = add_clone_subcommand(app);
    let app = add_doctor_subcommand(app);
    let app = add_git_subcommand(app);
    let app = add_init_subcommand(app);
    let app = add_nuke_subcommand(app);
    let app = add_status_subcommand(app);
    let app = add_use_subcommand(app);

    app
}


// **************************************************
// Subcommand configuration and implementation
// **************************************************

subcommand!{
  add_add_subcommand("add") {
    about("Add files to your hermit shell")
  }
}

fn handle_add<C: Config>(_matches: &ArgMatches,
                         _hermit: &mut Hermit<C>,
                         _file_operations: &mut FileOperations) {
    println!("hermit add is not yet implemented");
}


subcommand!{
  add_clone_subcommand("clone") {
    about("Create a local shell from an existing remote shell")
  }
}

fn handle_clone<C: Config>(_matches: &ArgMatches,
                           _hermit: &mut Hermit<C>,
                           _file_operations: &mut FileOperations) {
    println!("hermit clone is not implemented yet.")
}


subcommand!{
  add_doctor_subcommand("doctor") {
    about("Make sure your hermit setup is sane")
  }
}

fn handle_doctor<C: Config>(_matches: &ArgMatches,
                            _hermit: &mut Hermit<C>,
                            _file_operations: &mut FileOperations) {
    println!("hermit doctor is not implemented yet.")
}



subcommand!{
  add_git_subcommand("git") {
    about("Run git operations on the current shell")
  }
}

fn handle_git<C: Config>(_matches: &ArgMatches,
                         _hermit: &mut Hermit<C>,
                         _file_operations: &mut FileOperations) {
    println!("hermit git is not implemented yet.")
}


subcommand!{
  add_init_subcommand("init") {
    about("Create a new hermit shell called SHELL_NAME. If no shell name \
           is given, \"default\" is used.")
    arg(Arg::with_name("SHELL_NAME").help("The name of the shell to be created."))
  }
}

fn handle_init<C: Config>(matches: &ArgMatches,
                          hermit: &mut Hermit<C>,
                          file_operations: &mut FileOperations) {
    let shell_name = matches.value_of("SHELL_NAME").unwrap_or("default");
    hermit.init_shell(file_operations, shell_name);
}


subcommand!{
  add_nuke_subcommand("nuke") {
    about("Permanently remove a hermit shell")
  }
}

fn handle_nuke<C: Config>(_matches: &ArgMatches,
                          _hermit: &mut Hermit<C>,
                          _file_operations: &mut FileOperations) {
    println!("hermit nuke is not implemented yet.")
}


subcommand!{
  add_status_subcommand("status") {
    about("Display the status of your hermit shell")
  }
}

fn handle_status<C: Config>(_matches: &ArgMatches,
                            _hermit: &mut Hermit<C>,
                            _file_operations: &mut FileOperations) {
    println!("hermit status is not implemented yet.")
}


subcommand!{
  add_use_subcommand("use") {
    about("Switch to using a different hermit shell")
  }
}

fn handle_use<C: Config>(_matches: &ArgMatches,
                         _hermit: &mut Hermit<C>,
                         _file_operations: &mut FileOperations) {
    println!("hermit use is not implemented yet.")
}
