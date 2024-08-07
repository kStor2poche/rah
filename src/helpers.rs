use {
    anyhow::{anyhow, Context, Result},
    log::{debug, error, trace},
    std::process::Command,
    users::{get_current_uid, get_user_by_uid},
};

pub fn require_root() -> Result<()> {
    let uid = get_current_uid();

    debug!("Current uid is {uid}...");

    if let Some(user) = get_user_by_uid(uid) {
        debug!("...corresponding to user name {:?}", user.name());
    } else {
        error!("... but this uid doesn't correspond to any user");
        return Err(anyhow!(
            "Cannot identify current user (uid {uid}), aborting..."
        ));
    }

    if uid != 0 {
        error!("Program should be run as root, returning with error...");
        return Err(anyhow!("Program should be run as root, please launch it again with your favourite privilege escalation method !"));
    }

    Ok(())
}

pub fn check_exec_context() -> Result<()> {
    // First check if the chap launching this is even using an arch-based distro
    // TODO: more thourough checks an maybe allow running if pacman/makepkg is present ?
    let data = std::fs::read_to_string("/etc/os-release")
        .context( "Your distro is probably not an Arch-based distro, rah shouldn't be used on it. If untrue, please file an issue here https://github.com/kStor2poche/rah/issues\nAborting...")?;

    if !data.contains("arch") {
        return Err(anyhow!("Your distro is probably not an Arch-based distro, rah shouldn't be used on it. If untrue, please file an issue here https://github.com/kStor2poche/rah/issues\nAborting..."));
    }

    Ok(())
}

pub fn split_pacman_aur(pkgs: Vec<String>) -> Result<(Option<Vec<String>>, Option<Vec<String>>)> {
    // Used to know if package can be installed through pacman
    let mut pacman_sync_check_args = pkgs
        .iter()
        .fold(String::from("(^"), |s1, s2| s1 + &"$|^".to_string() + &s2);
    pacman_sync_check_args.push_str("$)");
    let pacman_sync_check = Command::new("pacman")
        .arg("-Ssq")
        .arg(pacman_sync_check_args)
        .output()?;
    let output = String::from_utf8(pacman_sync_check.stdout)?;
    let pacman_pkgs = match pacman_sync_check.status.code() {
        None => {
            return Err(anyhow!(
                "Pacman command did not exit or was killed by a signal"
            ));
        }
        Some(0) => Some(output.split("/").collect::<Vec<_>>()),
        Some(_) => None,
    };

    todo!("{:?}", pacman_pkgs)
}
