use anyhow::Result;
use raur::Raur;

pub async fn search(packages: Vec<&str>) -> Result<()> {
    let raur = raur::Handle::new();

    let hits = raur.search(packages.join(" ")).await?;

    println!("Found {} packages :", hits.len());

    // TODO: Query pacman to see if searched packages are already installed with pacman and/or are
    // in the local rah DB

    for pkg in hits {
        println!("{} - {}", pkg.name, pkg.version);
    }

    Ok(())
}
pub fn info(packages: Vec<&str>) {
    println!("Xearching for {}", packages.join(", "));
    todo!()
}
