#![allow(dead_code)]

use configparser::ini::Ini;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProfileInfo {
    pub profile_name: String,
    pub sso_start_url: String,
    pub sso_region: String,
    pub sso_account_id: String,
    pub sso_role_name: String,
    pub session_name: String,
}

pub struct AwsManager {
    aws_config_file: PathBuf,
}

impl AwsManager {
    pub fn new() -> io::Result<Self> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::NotFound,
                "\n [X] Could not find the AWS home directory\n",
            )
        })?;

        let aws_config_file = home_dir.join(".aws").join("config");

        Ok(Self { aws_config_file })
    }

    pub fn load_config(&self) -> io::Result<Ini> {
        if !self.aws_config_file.exists() {
            return Ok(Ini::new());
        }

        let content = fs::read_to_string(&self.aws_config_file)?;
        let mut config = Ini::new();
        config
            .read(content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }

    pub fn save_config(&self, config: &Ini) -> io::Result<()> {
        let mut output = String::new();
        for (section_name, section_map) in config.get_map_ref() {
            output.push_str(&format!("[{section_name}]\n"));
            for (key, value_opt) in section_map {
                if let Some(value) = value_opt {
                    output.push_str(&format!("{key} = {value}\n"));
                }
            }
            output.push('\n');
        }
        fs::write(&self.aws_config_file, output)
    }

    pub fn format_config(&self) -> io::Result<()> {
        let config = self.load_config()?;
        self.save_config(&config)
    }

    pub fn remove_profile(&self, profile_name: &str) -> io::Result<()> {
        let mut config = self.load_config()?;
        let section_name = format!("profile {profile_name}");
        config.remove_section(&section_name);
        self.save_config(&config)
    }

    pub fn read_profile(&self, profile_name: &str) -> Option<ProfileInfo> {
        if !self.aws_config_file.exists() {
            return None;
        }

        let content = fs::read_to_string(&self.aws_config_file).ok()?;
        let mut config = Ini::new();
        config.read(content).ok()?;

        let section_name = format!("profile {profile_name}");
        let section = config.get_map_ref().get(&section_name)?;

        let sso_account_id = section.get("sso_account_id")?.as_ref()?.clone();
        let sso_role_name = section.get("sso_role_name")?.as_ref()?.clone();

        let session_name = section
            .get("sso_session")
            .and_then(|s| s.as_ref())
            .unwrap_or(&String::new())
            .clone();

        // Helper function to get string value from section
        let get_value = |section: &HashMap<String, Option<String>>, key: &str| {
            section
                .get(key)
                .and_then(|s| s.as_ref())
                .cloned()
                .unwrap_or_default()
        };

        // Handle SSO session format - get sso_start_url and sso_region from sso-session section
        let (sso_start_url, sso_region) = if !session_name.is_empty() {
            let sso_section_name = format!("sso-session {session_name}");
            if let Some(sso_section) = config.get_map_ref().get(&sso_section_name) {
                (
                    get_value(sso_section, "sso_start_url"),
                    get_value(sso_section, "sso_region"),
                )
            } else {
                (String::new(), String::new())
            }
        } else {
            // Legacy format - try to get from profile section
            (
                get_value(section, "sso_start_url"),
                get_value(section, "sso_region"),
            )
        };

        Some(ProfileInfo {
            profile_name: profile_name.to_string(),
            sso_start_url,
            sso_region,
            sso_account_id,
            sso_role_name,
            session_name,
        })
    }
}
