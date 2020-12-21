#[macro_use]
extern crate failure_derive;

mod config;
mod env;
mod file_operations;
mod hermit;
mod message;
mod shell;

#[macro_use]
mod macros;

use std::process;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

use crate::{
    config::{Config, FsConfig},
    file_operations::FileOperations,
    hermit::{Error, Hermit, Result},
};

#[cfg(test)]
mod test_helpers;

const SHELL_NAME_ARG: &str = "SHELL_NAME";

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}: {}", env::get_program_name(), err);
            process::exit(1)
        }
    }
}

#[rustfmt::skip]
fn run() -> Result<()>{
    let app = make_app_config();
    let app_matches = app.get_matches();

    let hermit_root = env::get_hermit_dir().expect("Could not determine hermit root location.");
    let fs_config = FsConfig::new(hermit_root);
    let mut hermit = Hermit::new(fs_config);

    let home_dir = env::home_dir().expect("Could not determine home directory.");
    let mut file_operations = FileOperations::rooted_at(home_dir);

    match app_matches.subcommand() {
        ("add",     Some(matches)) => handle_add     (matches, &mut hermit, &mut file_operations),
        ("clone",   Some(matches)) => handle_clone   (matches, &mut hermit, &mut file_operations),
        ("doctor",  Some(matches)) => handle_doctor  (matches, &mut hermit, &mut file_operations),
        ("git",     Some(matches)) => handle_git     (matches, &mut hermit, &mut file_operations),
        ("init",    Some(matches)) => handle_init    (matches, &mut hermit, &mut file_operations),
        ("nuke",    Some(matches)) => handle_nuke    (matches, &mut hermit, &mut file_operations),
        ("shell",   Some(matches)) => handle_shell   (matches, &mut hermit, &mut file_operations),
        ("status",  Some(matches)) => handle_status  (matches, &mut hermit, &mut file_operations),
        ("inhabit", Some(matches)) => handle_inhabit (matches, &mut hermit, &mut file_operations),
        _ => unreachable!(message::error_str("unknown subcommand passed"))
    }?;

    report_errors(file_operations.commit());

    Ok(())
}

fn report_errors(results: Vec<file_operations::Result>) {
    for result in results {
        match result {
            Ok(()) => (),
            Err(e) => println!("{}", message::error(e)),
        }
    }
}

#[allow(clippy::let_and_return)]
fn make_app_config<'a, 'b>() -> App<'a, 'b> {
    let app = App::new("hermit")
        .version(env!("CARGO_PKG_VERSION"))
        .author("A product of the Bike Barn <https://github.com/bike-barn/hermit>")
        .about("A home directory configuration management assistant.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands);

    let app = add_add_subcommand(app);
    let app = add_clone_subcommand(app);
    let app = add_doctor_subcommand(app);
    let app = add_git_subcommand(app);
    let app = add_init_subcommand(app);
    let app = add_nuke_subcommand(app);
    let app = add_shell_subcommand(app);
    let app = add_status_subcommand(app);
    let app = add_inhabit_subcommand(app);

    app
}

// **************************************************
// Subcommand configuration and implementation
// **************************************************

subcommand! {
  add_add_subcommand("add") {
    about("Add files to your hermit shell")
  }
}

fn handle_add<C: Config>(
    _matches: &ArgMatches<'_>,
    _hermit: &mut Hermit<C>,
    _file_operations: &mut FileOperations,
) -> Result<()> {
    not_implemented("add")
}

subcommand! {
  add_clone_subcommand("clone") {
    about("Create a local shell from an existing remote shell")
  }
}

fn handle_clone<C: Config>(
    _matches: &ArgMatches<'_>,
    _hermit: &mut Hermit<C>,
    _file_operations: &mut FileOperations,
) -> Result<()> {
    not_implemented("clone")
}

subcommand! {
  add_doctor_subcommand("doctor") {
    about("Make sure your hermit setup is sane")
  }
}

fn handle_doctor<C: Config>(
    _matches: &ArgMatches<'_>,
    _hermit: &mut Hermit<C>,
    _file_operations: &mut FileOperations,
) -> Result<()> {
    not_implemented("doctor")
}

subcommand! {
  add_git_subcommand("git") {
    about("Run git operations on the current shell")
  }
}

fn handle_git<C: Config>(
    _matches: &ArgMatches<'_>,
    _hermit: &mut Hermit<C>,
    _file_operations: &mut FileOperations,
) -> Result<()> {
    not_implemented("git")
}

subcommand! {
  add_init_subcommand("init") {
    about("Create a new hermit shell called SHELL_NAME. If no shell name \
           is given, \"default\" is used.")
    arg(shell_name_arg("The name of the shell to be created."))
  }
}

fn handle_init<C: Config>(
    matches: &ArgMatches<'_>,
    hermit: &mut Hermit<C>,
    file_operations: &mut FileOperations,
) -> Result<()> {
    let shell_name = matches.value_of(SHELL_NAME_ARG).unwrap();
    hermit.init_shell(file_operations, shell_name)?;
    Ok(())
}

subcommand! {
  add_nuke_subcommand("nuke") {
    about("Permanently remove a hermit shell")
  }
}

fn handle_nuke<C: Config>(
    _matches: &ArgMatches<'_>,
    _hermit: &mut Hermit<C>,
    _file_operations: &mut FileOperations,
) -> Result<()> {
    not_implemented("nuke")
}

subcommand! {
  add_shell_subcommand("shell") {
    about("Display the shell you are currently inhabiting")
  }
}

fn handle_shell<C: Config>(
    _matches: &ArgMatches<'_>,
    hermit: &mut Hermit<C>,
    _file_operations: &mut FileOperations,
) -> Result<()> {
    let shell = hermit.current_shell()?;
    println!("{}", shell.name);
    Ok(())
}

subcommand! {
    add_status_subcommand("status") {
        about("Display the status of your hermit shell")
    }
}

fn handle_status<C: Config>(
    _matches: &ArgMatches<'_>,
    _hermit: &mut Hermit<C>,
    _file_operations: &mut FileOperations,
) -> Result<()> {
    not_implemented("status")
}

subcommand! {
  add_inhabit_subcommand("inhabit") {
    about("Switch to using a different hermit shell")
  }
}

fn handle_inhabit<C: Config>(
    matches: &ArgMatches<'_>,
    hermit: &mut Hermit<C>,
    file_operations: &mut FileOperations,
) -> Result<()> {
    let shell_name = matches.value_of(SHELL_NAME_ARG).unwrap();
    hermit.inhabit(file_operations, shell_name)?;
    Ok(())
}

// **************************************************
// Utility functions
// **************************************************

fn shell_name_arg<'a, 'b>(message: &'static str) -> Arg<'a, 'b> {
    Arg::with_name(SHELL_NAME_ARG)
        .default_value("default")
        .help(message)
}

fn not_implemented(name: &'static str) -> Result<()> {
    Err(Error::SubcommandNotImplemented(name))
}
