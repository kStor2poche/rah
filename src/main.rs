mod query;
mod database;
mod config;

use clap::{
    Arg,
    ArgAction,
    Command,
};

fn main() {
    // TODO: first check if the chap launching this is even using an arch-based distro

    let command_matches = Command::new("rah")
        .about("rah - the Rusty AUR Helper !")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("query")
                .short_flag('Q')
                .long_flag("query")
                .about("Query the local package database")
                .arg(
                    Arg::new("search")
                        .short('s')
                        .long("search")
                        .help("Search for matching packages in the local package database")
                        .conflicts_with("info")
                        .action(ArgAction::Set)
                        .num_args(1..),
                )
                .arg(
                    Arg::new("info")
                        .short('i')
                        .long("info")
                        .help("Get package info from the local package database")
                        .conflicts_with("search")
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .get_matches();

    match command_matches.subcommand() {
        Some(("query", query_matches)) => {
            if let Some(packages) = query_matches.get_many::<String>("search") {
                let packages = packages.map(|s| s.as_str()).collect::<Vec<_>>();
                query::search(packages);
            }
            if let Some(packages) = query_matches.get_many::<String>("info") {
                let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Retrieving info for {comma_sep}...");        
            } else if let Some(packages) = query_matches.get_many::<String>("") {
                let comma_sep = packages.map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
                println!("Seeking db for {comma_sep}...");        
            }
        },
        Some((command, _)) => {
            println!("Command \"{}\" not found.", command);
        }
        None => {
        }
    }
}
