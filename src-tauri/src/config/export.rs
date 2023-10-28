use std::path::PathBuf;

use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Export {
    pub export_folder: PathBuf,
    pub command: String,
    pub args: Vec<String>,
    pub locale: String,
}

impl Default for Export {
    fn default() -> Self {
        let dirs = UserDirs::new().unwrap();

        Self {
            export_folder: dirs
                .document_dir()
                .expect("Should be able to retrieve current users document directory but couldn't")
                .to_path_buf(),
            command: "/opt/homebrew/bin/weasyprint".to_owned(),
            args: [
                "{{ input_file }}",
                "{{ output_file }}",
                "--presentational-hints",
            ]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
            locale: "lv_LV".to_string(),
        }
    }
}
