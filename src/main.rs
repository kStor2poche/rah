mod config;
mod database;
mod query;

use {
    anyhow::{anyhow, Context, Result}, clap::{arg, Arg, ArgAction, Command}, log::{debug, error}, std::env, users::{get_current_uid, get_user_by_uid}
};

const VERSION: &str = "0.0.1";

fn check_exec_context() -> Result<()> {
    let uid = get_current_uid();

    debug!("Current uid is {uid}...");

    if let Some(user) = get_user_by_uid(uid) {
        debug!("...corresponding to user name {:?}", user.name());
    } else {
        error!("... but this uid doesn't correspond to any user");
        return Err(anyhow!("Cannot identify current user (uid {uid}), aborting..."))
    }

    if uid != 0 {
        error!("Program should be run as root, returning with error...");
        return Err(anyhow!("Program should be run as root, please launch it again with your favourite privilege escalation method !"))
    }

    // First check if the chap launching this is even using an arch-based distro
    // TODO: more thourough checks an maybe allow running if pacman/makepkg is present ?
    let data = std::fs::read_to_string("/etc/os-release").context("Your distro is probably not an Arch-based distro, rah shouldn't be used on it. If untrue, please file an issue here https://github.com/kStor2poche/rah/issues\nAborting...")?;

    if !data.contains("arch") {
        return Err(anyhow!("Your distro is probably not an Arch-based distro, rah shouldn't be used on it. If untrue, please file an issue here https://github.com/kStor2poche/rah/issues\nAborting..."))
    }

    Ok(())
}

fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env_logger::builder()
            .filter_level(log::LevelFilter::Off)
            .init();
    } else {
        env_logger::init();
    }

    check_exec_context()?;

    let command_matches = Command::new("rah")
        .about("rah - the Rusty AUR Helper !")
        .version(VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg(arg!(-c --config <FILE> "Choose a specific config file."))
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
        }
        Some((command, _)) => {
            println!("Command \"{}\" not found.", command);
        }
        None => {}
    }

    Ok(())
}
