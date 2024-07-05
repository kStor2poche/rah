use {
    crate::colors::*,
    alpm::{Alpm, SigLevel},
    anyhow::{anyhow, Result},
};

pub struct DepTree {
    pkg: Box<str>,
    dep_type: DepType,
    parent: Option<Box<DepTree>>,
    leaves: Option<Vec<DepTree>>,
}

enum DepType {
    Base,
    Dep,
    MakeDep,
    OptDep,
    CheckDep,
}

pub enum Pkg {
    Alpm { pkg: alpm::Package },
    Aur { pkg: raur::Package },
}

impl Pkg {
    pub fn name(&self) -> String {
        match self {
            Pkg::Alpm { pkg } => pkg.name().into(),
            Pkg::Aur { pkg } => pkg.name.clone().into(),
        }
    }

    pub fn depends(&self) -> Vec<String> {
        match self {
            Pkg::Alpm { pkg } => pkg
                .depends()
                .into_iter()
                .map(|dep| dep.to_string())
                .collect::<Vec<_>>(),
            Pkg::Aur { pkg } => pkg.depends.clone(),
        }
    }

    pub fn make_depends(&self) -> Result<Vec<String>> {
        match self {
            Pkg::Alpm { .. } => Err(anyhow!("Alpm packages don't need make dependencies")),
            Pkg::Aur { pkg } => Ok(pkg.make_depends.clone()),
        }
    }
}

impl From<raur::Package> for Pkg {
    fn from(pkg: raur::Package) -> Self {
        Pkg::Aur { pkg }
    }
}

impl From<alpm::Package> for Pkg {
    fn from(pkg: alpm::Package) -> Self {
        Pkg::Alpm { pkg }
    }
}

impl DepTree {
    pub fn build(package: &Pkg, alpm: Option<Alpm>) -> Result<Self> {
        let mut branch = Self {
            pkg: package.name().into(),
            dep_type: DepType::Base,
            parent: None,
            leaves: Some(Vec::new()),
        };
        let alpm = Alpm::new("/", "/var/lib/pacman/")?; // change this at some point to get it from
                                                        // the pacman-conf command

        // TODO: I think there's a better way to know which repos are used with pacman-conf (or with
        // alpm but the config options don't seem to be implemented in the rust interface)
        alpm.register_syncdb("core", SigLevel::NONE)?;
        match alpm.register_syncdb("extra", SigLevel::NONE) {
            Err(err) => eprintln!(
                "{YELLOW_L}{BOLD}warning :{CLEAR} Could not register db : {}",
                err
            ),
            Ok(_) => (),
        };
        alpm.register_syncdb("multilib", SigLevel::NONE)?;
        match alpm.register_syncdb("extra", SigLevel::NONE) {
            Err(err) => eprintln!(
                "{YELLOW_L}{BOLD}warning :{CLEAR} Could not register db : {}",
                err
            ),
            Ok(_) => (),
        };

        let db = alpm.localdb();
        let pkgs = db.pkgs();

        //let mut cache = HashSet::new(); // because caching could be a nice thing, I just don't
        //know how much I'd actually benefit from it

        let pkg = pkgs.find_satisfier(package.depends().first().unwrap().to_string());
        todo!()
    }

    pub fn build_all(packages: &Vec<Pkg>) -> Result<Vec<DepTree>> {
        let mut res: Vec<DepTree> = Vec::new();
        for package in packages {
            res.push(DepTree::build(package, None)?);
        }
        Ok(res)
    }
}
