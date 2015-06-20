use toml;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use rustc_serialize;

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct GeneralConfig {
    pub beatmap_dir: String,
    pub skin_dir: String,
    pub replays_dir: String,
    pub screenshots_dir: String,
    pub temp_dir: String,
}

impl Default for GeneralConfig {
    fn default() -> GeneralConfig {
        GeneralConfig {
            beatmap_dir: "Songs".to_string(),
            skin_dir: "Skins".to_string(),
            replays_dir: "Replays".to_string(),
            screenshots_dir: "Screenshots".to_string(),
            temp_dir: "Temp".to_string(),
        }
    }
}

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct AudioConfig {
    pub volume_global: i32,
    pub volume_music: i32,
    pub volume_effects: i32,
}

impl Default for AudioConfig {
    fn default() -> AudioConfig {
        AudioConfig {
            volume_global: 100,
            volume_music: 100,
            volume_effects: 75,
        }
    }
}

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct VideoConfig {
    pub width: u32,
    pub height: u32,
    pub fps_limit: u64,
    pub ups_limit: u64,
    pub enable_vsync: bool,
    pub fullscreen: bool,
    pub show_fps_counter: bool,
}

impl Default for VideoConfig {
    fn default() -> VideoConfig {
        VideoConfig {
            width: 800,
            height: 600,
            fps_limit: 60,
            ups_limit: 120,
            enable_vsync: false,
            fullscreen: false,
            show_fps_counter: false,
        }
    }
}

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct MiscConfig {
    pub confirm_exit: bool,
    pub debug_title: bool,
}

impl Default for MiscConfig {
    fn default() -> MiscConfig {
        MiscConfig {
            confirm_exit: false,
            debug_title: false,
        }
    }
}

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct KeyConfig {
    pub key_left: String,
    pub key_right: String,
    pub key_pause: String,
    pub key_screenshot: String,
}

impl Default for KeyConfig {
    fn default() -> KeyConfig {
        KeyConfig {
            key_left: "V".to_string(),
            key_right: "B".to_string(),
            key_pause: "Esc".to_string(),
            key_screenshot: "F12".to_string()
        }
    }
}

#[derive(Debug,RustcEncodable,RustcDecodable)]
pub struct Config {
    //general settings
    pub general: GeneralConfig,
    pub audio: AudioConfig,
    pub video: VideoConfig,
    pub misc: MiscConfig,
    pub key_config: KeyConfig
}

impl Config {
    pub fn from_file(path: &Path) -> Config {
        let mut f = File::open(path).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s);
        let dec:Config = toml::decode_str(&s).unwrap();
        dec
    }

    pub fn to_file(&self, path: &Path) {
        let mut f = File::create(path).unwrap();
        let st = toml::encode_str(self);
        f.write_all(st.as_ref());
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            general: GeneralConfig { ..Default::default() },
            audio: AudioConfig { ..Default::default() },
            video: VideoConfig { ..Default::default() },
            misc: MiscConfig { ..Default::default() },
            key_config: KeyConfig { ..Default::default() }
        }
    }
}
