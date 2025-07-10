// Library module for testable components
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod aws;
pub use aws::ProfileInfo;

pub const KEE_ART: &str = r#"

    ██╗  ██╗███████╗███████╗
    ██║ ██╔╝██╔════╝██╔════╝
    █████╔╝ █████╗  █████╗
    ██╔═██╗ ██╔══╝  ██╔══╝
    ██║  ██╗███████╗███████╗
    ╚═╝  ╚═╝╚══════╝╚══════╝

    AWS CLI profile manager"#;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct KeeConfig {
    pub profiles: HashMap<String, ProfileInfo>,
    pub current_profile: Option<String>,
}

impl KeeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_profile(&mut self, name: String, info: ProfileInfo) {
        self.profiles.insert(name, info);
    }

    pub fn remove_profile(&mut self, name: &str) -> Option<ProfileInfo> {
        let removed = self.profiles.remove(name);
        if self.current_profile.as_deref() == Some(name) {
            self.current_profile = None;
        }
        removed
    }

    pub fn get_profile(&self, name: &str) -> Option<&ProfileInfo> {
        self.profiles.get(name)
    }

    pub fn list_profiles(&self) -> Vec<(&String, &ProfileInfo)> {
        self.profiles.iter().collect()
    }

    pub fn set_current_profile(&mut self, name: Option<String>) {
        self.current_profile = name;
    }

    pub fn is_empty(&self) -> bool {
        self.profiles.is_empty()
    }
}

pub fn format_profile_name(profile_name: &str) -> String {
    format!("kee-{profile_name}")
}

pub fn parse_aws_config_section(
    content: &str,
    section_name: &str,
) -> Option<HashMap<String, String>> {
    let mut in_section = false;
    let mut section_data = HashMap::new();
    let target_section = format!("[{section_name}]");

    for line in content.lines() {
        let line = line.trim();

        if line.starts_with('[') {
            in_section = line == target_section;
            continue;
        }

        if in_section && !line.is_empty() && line.contains('=') {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                section_data.insert(key, value);
            }
        }
    }

    if section_data.is_empty() {
        None
    } else {
        Some(section_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kee_art_contains_expected_content() {
        assert!(KEE_ART.contains("AWS CLI profile manager"));
        assert!(!KEE_ART.is_empty());
        assert!(KEE_ART.len() > 50); // Should be a substantial ASCII art
    }

    #[test]
    fn test_profile_info_serialization() {
        let profile = ProfileInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            session_name: "test-session".to_string(),
        };

        let json = serde_json::to_string(&profile).unwrap();
        let deserialized: ProfileInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(profile, deserialized);
    }

    #[test]
    fn test_kee_config_default() {
        let config = KeeConfig::default();
        assert!(config.profiles.is_empty());
        assert!(config.current_profile.is_none());
        assert!(config.is_empty());
    }

    #[test]
    fn test_kee_config_new() {
        let config = KeeConfig::new();
        assert!(config.profiles.is_empty());
        assert!(config.current_profile.is_none());
    }

    #[test]
    fn test_kee_config_add_profile() {
        let mut config = KeeConfig::new();
        let profile = ProfileInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            session_name: "test-session".to_string(),
        };

        config.add_profile("test".to_string(), profile.clone());

        assert!(!config.is_empty());
        assert_eq!(config.profiles.len(), 1);
        assert_eq!(config.get_profile("test"), Some(&profile));
    }

    #[test]
    fn test_kee_config_remove_profile() {
        let mut config = KeeConfig::new();
        let profile = ProfileInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            session_name: "test-session".to_string(),
        };

        config.add_profile("test".to_string(), profile.clone());
        config.set_current_profile(Some("test".to_string()));

        let removed = config.remove_profile("test");

        assert_eq!(removed, Some(profile));
        assert!(config.is_empty());
        assert!(config.current_profile.is_none());
    }

    #[test]
    fn test_kee_config_remove_nonexistent_profile() {
        let mut config = KeeConfig::new();
        let removed = config.remove_profile("nonexistent");
        assert!(removed.is_none());
    }

    #[test]
    fn test_kee_config_get_profile() {
        let mut config = KeeConfig::new();
        let profile = ProfileInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            session_name: "test-session".to_string(),
        };

        config.add_profile("test".to_string(), profile.clone());

        assert_eq!(config.get_profile("test"), Some(&profile));
        assert_eq!(config.get_profile("nonexistent"), None);
    }

    #[test]
    fn test_kee_config_list_profiles() {
        let mut config = KeeConfig::new();
        let profile1 = ProfileInfo {
            profile_name: "kee-test1".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            session_name: "test-session1".to_string(),
        };
        let profile2 = ProfileInfo {
            profile_name: "kee-test2".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-west-2".to_string(),
            sso_account_id: "123456789013".to_string(),
            sso_role_name: "TestRole2".to_string(),
            session_name: "test-session2".to_string(),
        };

        config.add_profile("test1".to_string(), profile1);
        config.add_profile("test2".to_string(), profile2);

        let profiles = config.list_profiles();
        assert_eq!(profiles.len(), 2);

        let profile_names: Vec<&String> = profiles.iter().map(|(name, _)| *name).collect();
        assert!(profile_names.contains(&&"test1".to_string()));
        assert!(profile_names.contains(&&"test2".to_string()));
    }

    #[test]
    fn test_kee_config_set_current_profile() {
        let mut config = KeeConfig::new();

        config.set_current_profile(Some("test".to_string()));
        assert_eq!(config.current_profile, Some("test".to_string()));

        config.set_current_profile(None);
        assert!(config.current_profile.is_none());
    }

    #[test]
    fn test_kee_config_serialization() {
        let mut config = KeeConfig::new();
        let profile = ProfileInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            session_name: "test-session".to_string(),
        };

        config.add_profile("test".to_string(), profile);
        config.set_current_profile(Some("test".to_string()));

        let json = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: KeeConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_format_profile_name() {
        assert_eq!(format_profile_name("test"), "kee-test");
        assert_eq!(format_profile_name("my-profile"), "kee-my-profile");
        assert_eq!(format_profile_name(""), "kee-");
    }

    #[test]
    fn test_parse_aws_config_section_valid() {
        let config_content = r#"
[profile kee-test]
sso_start_url = https://test.awsapps.com/start
sso_region = us-east-1
sso_account_id = 123456789012
sso_role_name = TestRole

[profile other-profile]
"#;

        let section = parse_aws_config_section(config_content, "profile kee-test").unwrap();

        assert_eq!(
            section.get("sso_start_url"),
            Some(&"https://test.awsapps.com/start".to_string())
        );
        assert_eq!(section.get("sso_region"), Some(&"us-east-1".to_string()));
        assert_eq!(
            section.get("sso_account_id"),
            Some(&"123456789012".to_string())
        );
        assert_eq!(section.get("sso_role_name"), Some(&"TestRole".to_string()));
    }

    #[test]
    fn test_parse_aws_config_section_nonexistent() {
        let config_content = r#"
[profile kee-test]
sso_start_url = https://test.awsapps.com/start
"#;

        let section = parse_aws_config_section(config_content, "profile nonexistent");
        assert!(section.is_none());
    }

    #[test]
    fn test_parse_aws_config_section_empty_content() {
        let section = parse_aws_config_section("", "profile test");
        assert!(section.is_none());
    }

    #[test]
    fn test_parse_aws_config_section_malformed() {
        let config_content = r#"
[profile kee-test]
invalid_line_without_equals
sso_start_url = https://test.awsapps.com/start
= value_without_key
key_without_value =
"#;

        let section = parse_aws_config_section(config_content, "profile kee-test").unwrap();

        // Should parse valid lines and ignore malformed ones
        assert_eq!(
            section.get("sso_start_url"),
            Some(&"https://test.awsapps.com/start".to_string())
        );
        assert_eq!(section.get("key_without_value"), Some(&"".to_string()));
        assert!(!section.contains_key("invalid_line_without_equals"));
    }

    #[test]
    fn test_parse_aws_config_section_with_spaces() {
        let config_content = r#"
[profile kee-test]
  sso_start_url   =   https://test.awsapps.com/start
  region=us-east-1
"#;

        let section = parse_aws_config_section(config_content, "profile kee-test").unwrap();

        assert_eq!(
            section.get("sso_start_url"),
            Some(&"https://test.awsapps.com/start".to_string())
        );
    }
}
