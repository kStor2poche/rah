use anyhow::Result;
use raur::Raur;
use chrono::{TimeZone, Utc};

// const escape sequences
const CLEAR: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const BLACK: &str = "\x1b[30m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const PURPLE: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const GREY: &str = "\x1b[37m";
const BLACK_L: &str = "\x1b[38";
const RED_L: &str = "\x1b[39m";
const GREEN_L: &str = "\x1b[40m";
const YELLOW_L: &str = "\x1b[41m";
const BLUE_L: &str = "\x1b[42m";
const PURPLE_L: &str = "\x1b[43m";
const CYAN_L: &str = "\x1b[44m";
const WHITE: &str = "\x1b[45m";

pub async fn search(packages: Vec<&str>) -> Result<()> {
    let raur = raur::Handle::new();

    let hits = raur.search(packages.join(" ")).await?;

    println!("{BOLD}{BLUE}:: {CLEAR}{BOLD}Found {} packages{CLEAR}", hits.len());

    let mut pkg_flags: Vec<_> = vec![String::from("")];
    for pkg in hits {
        // TODO: Query pacman (and/or our database ?) to see if searched packages are already
        // installed with pacman and/or are in the local rah DB

        // out of date
        if let Some(pkg_ood) = pkg.out_of_date {
            let ood_str = Utc.timestamp_opt(pkg_ood, 0).unwrap();
            let last_mod_str = Utc.timestamp_opt(pkg.last_modified, 0).unwrap();
            pkg_flags.push(format!("{RED}[out of date since {}, last update {}]{CLEAR}", ood_str.format("%Y/%m/%d"), last_mod_str.format("%Y/%m/%d")))
        }
        println!("{BOLD}{} {GREEN}{}{}\n{CLEAR}    {}", pkg.name, pkg.version, pkg_flags.join(" "), pkg.description.unwrap_or(format!("{GREY}No description.{CLEAR}")));
    }

    Ok(())
}
pub async fn info(packages: Vec<&str>) -> Result<()> {
    let raur = raur::Handle::new();

    let hits = raur.info(&packages).await?;

    println!("{BOLD}{BLUE}:: {CLEAR}{BOLD}Found info for {} packages{CLEAR}", hits.len());

    for pkg in hits {
        println!("{BOLD}Name : {BLUE}{}{CLEAR}", pkg.name);
        println!("{BOLD}Version : {BLUE}{}{CLEAR}", pkg.version);
        println!("{BOLD}Description : {BLUE}{}{CLEAR}", pkg.description.unwrap_or(format!("{GREY}No description.{CLEAR}")));
    }

    todo!()
}
