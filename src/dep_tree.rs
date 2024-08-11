use {
    crate::colors::*,
    alpm::{Alpm, AlpmList, Db, SigLevel},
    anyhow::{anyhow, Result},
    raur::{self, Raur},
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
    Aur { pkg: raur::Package },
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

impl<'a> From<raur::Package> for Pkg<'a> {
    fn from(pkg: raur::Package) -> Self {
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
    pub async fn build_all(
        packages: &'a Vec<Pkg<'_>>,
        alpm: &'a Alpm,
        raur: &raur::Handle,
    ) -> Result<Vec<DepTree<'a>>> {
        let mut res: Vec<DepTree> = Vec::new();

        // TODO: I think there's a better way to know which repos are used with pacman-conf -l (or
        // with alpm but the config options don't seem to be implemented in the rust interface)
        // TODO: Lookup why I still have to register those dbs despite them being already retrieved
        if alpm.syncdbs().is_empty() {
            match alpm.register_syncdb("core", SigLevel::NONE) {
                Err(err) => eprintln!(
                    "{YELLOW_L}{BOLD}warning :{CLEAR} Could not register db : {}",
                    err
                ),
                Ok(_) => (),
            };
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
        }

        let local_db = alpm.localdb();
        let sync_dbs = alpm.syncdbs();

        for package in packages {
            res.push(DepTree::build(package, &local_db, &sync_dbs, &raur, None).await?);
        }
        Ok(res)
    }

    pub async fn build(
        package: &'a Pkg<'_>,
        local_db: &Db,
        sync_dbs: &AlpmList<'_, &'a Db>,
        raur: &raur::Handle,
        prev: Option<&DepTree<'_>>,
    ) -> Result<Self> {
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
            println!("checking sat for {pkg}");
            if let Some(_) = local_pkgs.find_satisfier(pkg.clone()) {
                println!("sat");
                continue; // package already installed with a correct version
            }

            for db in *sync_dbs {
                match db.pkgs().find_satisfier(pkg.clone()) {
                    Some(pkg) => {
                        // a pacman pkg
                        println!("pacman unsat");
                        let leave = DepTree {
                            pkg: pkg.into(),
                            dep_type: DepType::Dep,
                            leaves: None,
                            parent: Some(&mut branch as *mut DepTree<'a>), // further be dragons !
                        };
                        leaves.push(leave);
                        break;
                    }
                    None => (),
                };
            }

            // an aur pkg, but how can I do dependency lookup not horribly,
            // except by caching raur's results ?
            println!("aur unsat");
            let (pkg_name, pkg_ver_req) = parse_dependency(&pkg);
            let res = raur.search_by(pkg_name, raur::SearchBy::Provides).await?;
            println!("is provided by {res:?}");
            if res.is_empty() {
                return Err(anyhow!(format!("No match found for dependency {}", pkg)));
            }

            let mvp: Pkg;
            let mut viable: Vec<Pkg>;
            let mut maybe_viable: Vec<Pkg>;
            for aur_pkg in res {
                if aur_pkg.name == pkg_name {
                }
                let leave = DepTree {
                    pkg: aur_pkg.into(),
                    dep_type: DepType::Dep,
                    leaves: None,
                    parent: Some(&mut branch as *mut DepTree<'a>), // further be dragons !
                };
                leaves.push(leave);
            }
            //let res_bis = raur.search(pkg_info.0).await?;
            //println!("could be satisfied by {res_bis:?}");
            todo!()
        }

        branch.leaves = Some(leaves);

        todo!()
    }
}

fn parse_dependency<'a>(pkg: &'a String) -> (&'a str, Option<(&'a str, &'a str)>) {
    let sep_index = pkg.find(|c| c == '=' || c == '<' || c == '>');

    if sep_index == None {
        return (pkg, None);
    }

    let sep_index = sep_index.unwrap();

    let pkg_name = &pkg[0..sep_index];

    let pkg_ver_ord;
    let pkg_ver;
    if pkg.chars().nth(sep_index + 1) == Some('=') {
        pkg_ver_ord = &pkg[sep_index..=sep_index + 1];
        pkg_ver = &pkg[sep_index + 2..];
    } else {
        pkg_ver_ord = &pkg[sep_index..=sep_index];
        pkg_ver = &pkg[sep_index + 1..];
    }

    (pkg_name, Some((pkg_ver_ord, pkg_ver)))
}

fn check_version_requirement(req_ord: &str, req_ver: &str, ver: &str) -> Option<bool> {
    todo!()
}
