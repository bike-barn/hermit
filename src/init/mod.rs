extern crate clap;

pub fn new_shell(matches: &clap::ArgMatches) {
    let name = match matches.value_of("SHELL_NAME"){
        Some(shell_name) => shell_name,
        None => "default",
    };
    println!("Creating Hermit Shell {}", name);
}
