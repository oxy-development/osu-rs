use std::path::Path;

use crate::stated_game_app::{GameApp, StateMachine, GameState};
use crate::config::Config;

pub struct Container {
    pub config: Config,
    pub app: GameApp
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

        let mut app = GameApp::new();
        app.add_state(GameState::Loading, vec![GameState::Initial].as_ref());
        app.add_state(GameState::Initial, vec![GameState::Settings, GameState::SongChoose].as_ref());

        Container {
            config: cfg,
            app: app.clone()
        }
    }
}
