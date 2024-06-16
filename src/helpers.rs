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

pub fn check_deps(depends: Vec<String>) -> Result<Vec<Box<str>>> {
    let pacman_deps_check = Command::new("pacman").arg("-T").args(depends).output()?;
    match pacman_deps_check.status.code() {
        None => {
            return Err(anyhow!(
                "Pacman command did not exit or was killed by a signal"
            ))
        }
        Some(127) => {
            let deps_output = String::from_utf8_lossy(&pacman_deps_check.stdout);
            let deps_vec = deps_output
                .split('\n')
                .map(|s| s.into())
                .collect::<Vec<Box<str>>>();
            trace!("    found : {:?}", deps_vec);
            return Ok(deps_vec);
        }
        Some(0) => {
            trace!("    deps ok !");
            return Ok(vec![]);
        }
        Some(_) => {
            return Err(anyhow!(
                "Pacman returned a fatal error : {}",
                String::from_utf8_lossy(&pacman_deps_check.stderr)
            ))
        }
    }
}
