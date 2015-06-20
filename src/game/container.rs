use std::fs::PathExt;
use std::path::Path;

use game::config::*;

pub struct Container {
    pub config: Config
}

impl Container {
    pub fn create() -> Container {
        let cfg_path = Path::new("./bin/settings.toml");
        let mut cfg = Config { ..Default::default() };
        if cfg_path.exists() {
            cfg = Config::from_file(&cfg_path);
        } else {
            cfg.to_file(Path::new("./bin/settings.toml"));
        }

        Container {
            config: cfg
        }
    }
}
