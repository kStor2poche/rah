use anyhow::{anyhow, Result};
use chrono::{TimeZone, Utc};
use raur::Raur;

// const escape sequences
pub const CLEAR: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const BLACK: &str = "\x1b[30m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const PURPLE: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const GREY: &str = "\x1b[37m";
pub const BLACK_L: &str = "\x1b[38";
pub const RED_L: &str = "\x1b[39m";
pub const GREEN_L: &str = "\x1b[40m";
pub const YELLOW_L: &str = "\x1b[41m";
pub const BLUE_L: &str = "\x1b[42m";
pub const PURPLE_L: &str = "\x1b[43m";
pub const CYAN_L: &str = "\x1b[44m";
pub const WHITE: &str = "\x1b[45m";

pub async fn sync(packages: Vec<&str>) -> Result<()> {
    let raur = raur::Handle::new();

    let hits = raur.info(&packages).await?;

    if hits.len() != packages.len() {
        eprint!("{BOLD}{RED}error :{CLEAR} Package(s) missing : ");
        let hit_names = hits.iter().map(|pkg| pkg.name.clone()).collect::<Vec<_>>();
        for package in packages.as_slice() {
            if !(hit_names.contains(&package.to_string())) {
                eprint!("{} ", package);
            }
        }
        eprintln!("");
        todo!();
        return Err(anyhow!("TODO"))
    }

    for hit in hits {
        println!("haiii");
        todo!()
    }

    Ok(())
}

pub async fn search(packages: Vec<&str>) -> Result<()> {
    let raur = raur::Handle::new();

    let hits = raur.search(packages.join(" ")).await?;

    println!(
        "{BOLD}{BLUE}:: {CLEAR}{BOLD}Found {} package{}{CLEAR}",
        hits.len(),
        if hits.len() != 1 { "s" } else { "" }
    );

    let mut pkg_flags: Vec<_> = vec![String::from("")];
    for pkg in hits {
        // TODO: Fetch the pacman db (and/or our database ?) to see if searched packages are already
        // installed with pacman and/or are in the local rah DB

        pkg_flags.clear();
        // out of date
        if let Some(pkg_ood) = pkg.out_of_date {
            let ood_str = Utc.timestamp_opt(pkg_ood, 0).unwrap();
            let last_mod_str = Utc.timestamp_opt(pkg.last_modified, 0).unwrap();
            pkg_flags.push(format!(
                "{RED} [out of date since {}, last update {}]{CLEAR}",
                ood_str.format("%Y/%m/%d"),
                last_mod_str.format("%Y/%m/%d")
            ))
        }
        println!(
            "{BOLD}{} {GREEN}{}{}\n{CLEAR}    {}",
            pkg.name,
            pkg.version,
            pkg_flags.join(""),
            pkg.description
                .unwrap_or(format!("{GREY}No description.{CLEAR}"))
        );
    }

    Ok(())
}

pub async fn info(packages: Vec<&str>) -> Result<()> {
    let raur = raur::Handle::new();

    let hits = raur.info(&packages).await?;

    println!(
        "{BOLD}{BLUE}:: {CLEAR}{BOLD}Found info for {} package{}{CLEAR}",
        hits.len(),
        if hits.len() != 1 { "s" } else { "" }
    );

    for pkg in hits {
        println!("{BOLD}Name : {CLEAR}{}", pkg.name);
        println!("{BOLD}Version : {CLEAR}{}", pkg.version);
        println!("{BOLD}Package base : {CLEAR}{}", pkg.package_base);
        println!("{BOLD}Votes : {CLEAR}{}", pkg.num_votes);
        println!("{BOLD}Popularity : {CLEAR}{}", pkg.popularity);
        println!(
            "{BOLD}Description : {CLEAR}{}",
            pkg.description
                .unwrap_or(format!("{GREY}No description.{CLEAR}"))
        );
        println!(
            "{BOLD}Submitter : {CLEAR}{}",
            pkg.submitter
                .unwrap_or(format!("{GREY}No submitter.{CLEAR}"))
        );
        println!(
            "{BOLD}Maintainer : {CLEAR}{}",
            pkg.maintainer
                .unwrap_or(format!("{GREY}No maintainer.{CLEAR}"))
        );
        let co_maintainers = pkg.co_maintainers;
        println!(
            "{BOLD}Co-maintainers : {CLEAR}{}",
            if co_maintainers.is_empty() {
                format!("{GREY}No co-maintainers.{CLEAR}")
            } else {
                co_maintainers.join(", ")
            }
        );
        if let Some(pkg_ood) = pkg.out_of_date {
            let ood_ts = Utc.timestamp_opt(pkg_ood, 0).unwrap();
            println!(
                "{BOLD}Out of date : {RED}Flagged out of date since {}{CLEAR}",
                ood_ts.format("%Y-%m-%d %H:%M (UTC)")
            )
        } else {
            println!("{BOLD}Out of date : {CLEAR}{GREY}Not flagged out of date{CLEAR}");
        }
        let first_sub_ts = Utc.timestamp_opt(pkg.first_submitted, 0).unwrap();
        println!(
            "{BOLD}First submitted : {CLEAR}{}",
            first_sub_ts.format("%Y-%m-%d %H:%M (UTC)")
        );
        let last_mod_ts = Utc.timestamp_opt(pkg.last_modified, 0).unwrap();
        println!(
            "{BOLD}Last updated : {CLEAR}{}",
            last_mod_ts.format("%Y-%m-%d %H:%M (UTC)")
        );

        println!(
            "{BOLD}Git clone URL : {CLEAR}https://aur.archlinux.org/{}.git",
            pkg.package_base
        );
        println!(
            "{BOLD}Upstream URL : {CLEAR}{}",
            pkg.url.unwrap_or(format!("{GREY}No upstream URL."))
        );
        println!(
            "{BOLD}Tarball URL : {CLEAR}https://aur.archlinux.org{}",
            pkg.url_path
        );
        println!("{BOLD}Licenses : {CLEAR}{}", pkg.license.join(", "));
        let groups = pkg.groups;
        println!(
            "{BOLD}Groups : {CLEAR}{}",
            if groups.is_empty() {
                format!("{GREY}No groups.{CLEAR}")
            } else {
                groups.join(", ")
            }
        );
        let provides = pkg.provides;
        println!(
            "{BOLD}Provides : {CLEAR}{}",
            if provides.is_empty() {
                format!("{GREY}No provides.{CLEAR}")
            } else {
                provides.join(", ")
            }
        );
        let depends = pkg.depends;
        println!(
            "{BOLD}Depends : {CLEAR}{}",
            if depends.is_empty() {
                format!("{GREY}No dependencies.{CLEAR}")
            } else {
                depends.join(", ")
            }
        );
        let opt_depends = pkg.opt_depends;
        println!(
            "{BOLD}Opt. dependencies : {CLEAR}{}",
            if opt_depends.is_empty() {
                format!("{GREY}No optionnal dependencies.{CLEAR}")
            } else {
                opt_depends.join(", ")
            }
        );
        let make_depends = pkg.make_depends;
        println!(
            "{BOLD}Make dependencies : {CLEAR}{}",
            if make_depends.is_empty() {
                format!("{GREY}No make dependencies.{CLEAR}")
            } else {
                make_depends.join(", ")
            }
        );
        let check_depends = pkg.check_depends;
        println!(
            "{BOLD}Check dependencies : {CLEAR}{}",
            if check_depends.is_empty() {
                format!("{GREY}No check dependencies.{CLEAR}")
            } else {
                check_depends.join(", ")
            }
        );
        let conflicts = pkg.conflicts;
        println!(
            "{BOLD}Conflicts : {CLEAR}{}",
            if conflicts.is_empty() {
                format!("{GREY}No conflicts.{CLEAR}")
            } else {
                conflicts.join(", ")
            }
        );
        let replaces = pkg.replaces;
        println!(
            "{BOLD}Replaces : {CLEAR}{}",
            if replaces.is_empty() {
                format!("{GREY}No replaces.{CLEAR}")
            } else {
                replaces.join(", ")
            }
        );
        let keywords = pkg.keywords;
        println!(
            "{BOLD}Keywords : {CLEAR}{}",
            if keywords.is_empty() {
                format!("{GREY}No keywords.{CLEAR}")
            } else {
                keywords.join(", ")
            }
        );
        println!("")
    }
    Ok(())
}
