use {
    crate::colors::*,
    alpm::{Alpm, AlpmList, Db, SigLevel},
    anyhow::{anyhow, Result},
    raur::Handle,
};

#[derive(Clone)]
enum DepType {
    Base,
    Dep,
    MakeDep,
    OptDep,
    CheckDep,
}

#[derive(Clone)]
pub enum Pkg<'a> {
    Pacman { pkg: &'a alpm::Package },
    Aur { pkg: &'a raur::Package },
}

impl Pkg<'_> {
    pub fn name(&self) -> String {
        match self {
            Pkg::Pacman { pkg } => pkg.name().into(),
            Pkg::Aur { pkg } => pkg.name.clone().into(),
        }
    }

    pub fn depends(&self) -> Vec<String> {
        match self {
            Pkg::Pacman { pkg } => pkg
                .depends()
                .into_iter()
                .map(|dep| dep.to_string())
                .collect::<Vec<_>>(),
            Pkg::Aur { pkg } => pkg.depends.clone(),
        }
    }

    pub fn make_depends(&self) -> Result<Vec<String>> {
        match self {
            Pkg::Pacman { .. } => Err(anyhow!("Alpm packages don't need make dependencies")),
            Pkg::Aur { pkg } => Ok(pkg.make_depends.clone()),
        }
    }
}

impl<'a> From<&'a raur::Package> for Pkg<'a> {
    fn from(pkg: &'a raur::Package) -> Self {
        Pkg::Aur { pkg }
    }
}

impl<'a> From<&'a alpm::Package> for Pkg<'a> {
    fn from(pkg: &'a alpm::Package) -> Self {
        Pkg::Pacman { pkg }
    }
}

#[derive(Clone)]
pub struct DepTree<'a> {
    pkg: Pkg<'a>,
    dep_type: DepType,
    leaves: Option<Vec<DepTree<'a>>>,
    parent: Option<*mut DepTree<'a>>,
}

impl<'a> DepTree<'a> {
    pub fn build(package: &'a Pkg, local_db: &Db, sync_dbs: &AlpmList<&'a Db>, raur: &Handle, prev: Option<&DepTree>) -> Result<Self> {
        let mut branch = Self {
            pkg: package.clone(),
            dep_type: DepType::Base,
            leaves: Some(Vec::new()),
            parent: None,
        };

        let local_pkgs = local_db.pkgs();

        let mut leaves: Vec<DepTree> = Vec::new();
        let mut not_found: Vec<String> = Vec::new(); // TODO: fill the Vec. Now. Do it. Just
                                                     // reminding you. No pressure.

        //let mut cache = HashSet::new(); // because caching could be a nice thing, I just don't
        //know how much I'd actually benefit from it

        for pkg in package.depends() {
            if let Some(_) = local_pkgs.find_satisfier(pkg.clone()) {
                continue; // package already installed with a correct version
            }

            for db in *sync_dbs {
                match db.pkgs().find_satisfier(pkg.clone()) {
                    Some(pkg) => { // a pacman pkg
                        let leave = DepTree {
                            pkg: pkg.into(),
                            dep_type: DepType::Dep,
                            leaves: None,
                            parent: Some(&mut branch as *mut DepTree<'a>), // here be dragons !
                        };
                        leaves.push(leave);
                    }
                    None => { // an aur pkg, but how can I do dependency lookup not horribly,
                              // except by caching raur's results ?
                        todo!()
                    }
                     
                };
            }
        }

        branch.leaves = Some(leaves); // TODO: mfw unsafe code

        todo!()
    }

    pub fn build_all(packages: &'a Vec<Pkg>) -> Result<Vec<DepTree<'a>>> { // TODO: change this
                                                                           // function so that it
                                                                           // returns just a list
                                                                           // of alpm and pacman
                                                                           // packages to install ?
                                                                           // OR take alpm as an
                                                                           // argument
        let mut res: Vec<DepTree> = Vec::new();

        let alpm = Alpm::new("/", "/var/lib/pacman/")?; // change this at some point to get it from
        let raur = Handle::new();

        // TODO: I think there's a better way to know which repos are used with pacman-conf -l (or
        // with alpm but the config options don't seem to be implemented in the rust interface)
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

        let local_db = alpm.localdb();
        let sync_dbs = alpm.syncdbs(); // faire un slice::from_raw_parts ??

        for package in packages {
            res.push(DepTree::build(package, &local_db, &sync_dbs, &raur, None)?);
        }
        Ok(res)
    }
}
