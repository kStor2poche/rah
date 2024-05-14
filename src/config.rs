pub struct Config {
    config_path: String,
    db_path: String,
    cache_path: String,
    color: bool,
    pager: Option<String>,
}

impl Config {
    fn default() -> Self {
        Config {
            config_path: String::from("/etc/rah/config"),
            db_path: String::from("/var/lib/rah/db/"),
            cache_path: String::from("/var/cache/rah/"),
            color: true,
            pager: Some(String::from("less -r")),
        }
    }

    fn parse(&mut self) {
        todo!()
    }
}
