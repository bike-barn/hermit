#[macro_use]
extern crate clap;

fn main() {

    let version = format!("{}.{}.{}{}",
                          option_env!("CARGO_PKG_VERSION_MAJOR").unwrap_or("X"),
                          option_env!("CARGO_PKG_VERSION_MINOR").unwrap_or("X"),
                          option_env!("CARGO_PKG_VERSION_PATCH").unwrap_or("X"),
                          option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    let app = clap_app!(myapp =>
        (version: &*version)
        (author: "Bike Barn <https://github.com/bike-barn/hermit>")
        (about: "A home directory configuration management assistant.")
        (@subcommand add =>
            (about: "Add files to your hermit shell")
            (usage: "hermit add [<filepattern>…]"))
        (@subcommand clone =>
            (about: "Create a local shell from an existing remote shell")
            (usage: "hermit clone <repository> [shell-name]"))
        (@subcommand doctor =>
            (about: "Make sure your hermit setup is sane")
            (usage: "hermit doctor"))
        (@subcommand git =>
            (about: "Run git operations on the current shell")
            (usage: "hermit git <git arguments>"))
        (@subcommand init =>
            (about: "Create a new hermit shell")
            (usage: "hermit init [shell-name]"))
        (@subcommand nuke =>
            (about: "Permanently remove a hermit shell")
            (usage: "hermit nuke <shell-name>"))
        (@subcommand status =>
            (about: "Display the status of your hermit shell")
            (usage: "hermit status"))
        (@subcommand use =>
            (about: "Switch to using a different hermit shell")
            (usage: "hermit use [shell-name]"))
    );
    let app_matches = app.get_matches();

    match app_matches.subcommand() {
        ("add", Some(matches)) => {
            println!("hermit add is not yet implemented");
        }
        ("clone", Some(matches)) => {
            println!("hermit clone is not yet implemented");
        }
        ("doctor", Some(matches)) => {
            println!("hermit doctor is not yet implemented");
        }
        ("git", Some(matches)) => {
            println!("hermit git is not yet implemented");
        }
        ("init", Some(matches)) => {
            println!("hermit init is not yet implemented");
        }
        ("nuke", Some(matches)) => {
            println!("hermit nuke is not yet implemented");
        }
        ("status", Some(matches)) => {
            println!("hermit status is not yet implemented");
        }
        ("use", Some(matches)) => {
            println!("hermit use is not yet implemented");
        }
        _ => {
            println!("no subcommand was called");
        }
    }
}
