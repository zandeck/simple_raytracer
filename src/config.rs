use crate::camera::{Camera, CameraBuilder};
use failure::format_err;
use failure::Error;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

pub struct ConfigLoader<'a> {
    path: &'a Path,
}

impl<'a> ConfigLoader<'a> {
    pub fn new(path: &'static str) -> ConfigLoader {
        ConfigLoader {
            path: Path::new(path),
        }
    }

    pub fn load(&self) -> Result<Config, Error> {
        let mut file = File::open(self.path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        match toml::from_str(&content) {
            Ok(config) => Ok(config),
            Err(e) => Err(format_err!("Error Reading config: {}", e)),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Config {
    pub params: SimulationParameters,
    camera: CameraBuilder,
}

impl Config {
    pub fn camera(&self) -> Camera {
        self.camera.build()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct SimulationParameters {
    pub x_dim: u32,
    pub y_dim: u32,
    pub n: u32,
}
