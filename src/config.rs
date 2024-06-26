use {anyhow::Result, toml};

pub struct Config {
    pub config_path: String,
    pub db_path: String,
    pub cache_path: String,
    pub color: bool,
    pub delete_make_deps: Option<bool>,
    //pub pager_cmd: Option<String>,
}

impl Config {
    pub fn default() -> Self {
        Config {
            config_path: String::from("/etc/rah.toml"),
            db_path: String::from("/var/lib/rah/db/"),
            cache_path: String::from("/var/cache/rah/"),
            color: true,
            delete_make_deps: None,
            //pager_cmd: Some(String::from("less -r")),
        }
    }

    pub fn parse(&mut self, cfg_path: Option<String>) -> Result<()> {
        todo!("{:?}", cfg_path)
    }
}
