use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli::Cli;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub input_file: PathBuf,
    pub video_file_dir: PathBuf,
    pub output_file_dir: PathBuf,
}

impl Config {
    pub fn new(video_file_dir: PathBuf, output_file_dir: PathBuf) -> Self {
        Self {
            input_file: PathBuf::new(),
            video_file_dir,
            output_file_dir,
        }
    }

    pub fn overwrite(&mut self, video_file_dir: Option<PathBuf>, output_file_dir: Option<PathBuf>) {
        if let Some(video_file_dir) = video_file_dir {
            self.video_file_dir = video_file_dir;
        }
        if let Some(output_file_dir) = output_file_dir {
            self.output_file_dir = output_file_dir;
        }

        let app_dir = dirs::config_dir().unwrap();
        let config_file_path = app_dir.join("ascii").join("config.json");
        let config_str = serde_json::to_string_pretty(self).unwrap();
        std::fs::write(config_file_path, config_str).unwrap();
    }

    pub fn init() -> Self {
        let args = Cli::get_args();

        let mut config = Config::default();

        if args.overwrite {
            config.overwrite(args.video_dir, args.output_dir);
        }

        if !args.input.is_absolute() {
            let input_file = config.video_file_dir.join(&args.input);
            config.input_file = input_file;
        } else {
            config.input_file = args.input;
        }
        config
    }
}

impl Default for Config {
    fn default() -> Self {
        let app_dir = dirs::config_dir().unwrap();
        let config_file_path = app_dir.join("ascii").join("config.json");
        if !config_file_path.exists() {
            let config = Self::new(PathBuf::from("./video"), PathBuf::from("./output"));
            fs::create_dir_all(app_dir.join("ascii")).unwrap();
            let config_str = serde_json::to_string_pretty(&config).unwrap();
            std::fs::write(&config_file_path, config_str).unwrap();
        }
        let config_str = std::fs::read_to_string(config_file_path).unwrap();
        let mut config: Self = serde_json::from_str(&config_str).unwrap();
        config.input_file = PathBuf::new();
        config
    }
}
