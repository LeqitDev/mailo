use std::{
    fs::File,
    io::prelude::*,
    path::{self, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SettingsWrapper {
    path: PathBuf,
    pub settings: Settings,
}

impl SettingsWrapper {
    pub fn new(path: PathBuf, settings: Settings) -> Self {
        Self { path, settings }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Settings {
    pub master_password: bool,
}

pub trait SettingsTrait {
    fn save(&self);
    fn load(path: PathBuf) -> Self;
}

impl SettingsTrait for SettingsWrapper {
    fn save(&self) {
        log::info!("Saving settings to: {:#?}", self.path);
        if let Ok(mut file) = File::create(self.path.clone()) {
            file.write_all(serde_json::to_string(&self.settings).unwrap().as_bytes())
                .unwrap();
            drop(file);
        } else {
            log::error!("Failed to save settings");
        }
    }

    fn load(path: PathBuf) -> Self {
        if let Ok(file) = File::open(path.clone()) {
            let mut buf_reader = std::io::BufReader::new(file);
            let mut contents = String::new();
            buf_reader.read_to_string(&mut contents).unwrap();
            drop(buf_reader);
            if let Ok(ret) = serde_json::from_str::<Settings>(&contents) {
                Self::new(path, ret)
            } else {
                log::warn!("Failed to parse settings, using default settings");
                Self::new(path, Settings::default())
            }
        } else {
            log::warn!("Failed to load settings, using default settings");
            Self::new(path, Settings::default())
        }
    }
}
