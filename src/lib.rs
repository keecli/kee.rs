// Library module for testable components
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const KEE_ART: &str = r#"
    ██╗  ██╗███████╗███████╗
    ██║ ██╔╝██╔════╝██╔════╝
    █████╔╝ █████╗  █████╗
    ██╔═██╗ ██╔══╝  ██╔══╝
    ██║  ██╗███████╗███████╗
    ╚═╝  ╚═╝╚══════╝╚══════╝

    AWS CLI session manager
    "#;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AccountInfo {
    pub profile_name: String,
    pub sso_start_url: String,
    pub sso_region: String,
    pub sso_account_id: String,
    pub sso_role_name: String,
    pub region: String,
    pub session_name: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct KeeConfig {
    pub accounts: HashMap<String, AccountInfo>,
    pub current_account: Option<String>,
}

impl KeeConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_account(&mut self, name: String, info: AccountInfo) {
        self.accounts.insert(name, info);
    }

    pub fn remove_account(&mut self, name: &str) -> Option<AccountInfo> {
        let removed = self.accounts.remove(name);
        if self.current_account.as_deref() == Some(name) {
            self.current_account = None;
        }
        removed
    }

    pub fn get_account(&self, name: &str) -> Option<&AccountInfo> {
        self.accounts.get(name)
    }

    pub fn list_accounts(&self) -> Vec<(&String, &AccountInfo)> {
        self.accounts.iter().collect()
    }

    pub fn set_current_account(&mut self, name: Option<String>) {
        self.current_account = name;
    }

    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }
}

pub fn format_profile_name(account_name: &str) -> String {
    format!("kee-{}", account_name)
}

pub fn parse_aws_config_section(
    content: &str,
    section_name: &str,
) -> Option<HashMap<String, String>> {
    let mut in_section = false;
    let mut section_data = HashMap::new();
    let target_section = format!("[{}]", section_name);

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
        assert!(KEE_ART.contains("AWS CLI session manager"));
        assert!(!KEE_ART.is_empty());
        assert!(KEE_ART.len() > 50); // Should be a substantial ASCII art
    }

    #[test]
    fn test_account_info_serialization() {
        let account = AccountInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            region: "us-east-1".to_string(),
            session_name: "test-session".to_string(),
        };

        let json = serde_json::to_string(&account).unwrap();
        let deserialized: AccountInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(account, deserialized);
    }

    #[test]
    fn test_kee_config_default() {
        let config = KeeConfig::default();
        assert!(config.accounts.is_empty());
        assert!(config.current_account.is_none());
        assert!(config.is_empty());
    }

    #[test]
    fn test_kee_config_new() {
        let config = KeeConfig::new();
        assert!(config.accounts.is_empty());
        assert!(config.current_account.is_none());
    }

    #[test]
    fn test_kee_config_add_account() {
        let mut config = KeeConfig::new();
        let account = AccountInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            region: "us-east-1".to_string(),
            session_name: "test-session".to_string(),
        };

        config.add_account("test".to_string(), account.clone());

        assert!(!config.is_empty());
        assert_eq!(config.accounts.len(), 1);
        assert_eq!(config.get_account("test"), Some(&account));
    }

    #[test]
    fn test_kee_config_remove_account() {
        let mut config = KeeConfig::new();
        let account = AccountInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            region: "us-east-1".to_string(),
            session_name: "test-session".to_string(),
        };

        config.add_account("test".to_string(), account.clone());
        config.set_current_account(Some("test".to_string()));

        let removed = config.remove_account("test");

        assert_eq!(removed, Some(account));
        assert!(config.is_empty());
        assert!(config.current_account.is_none());
    }

    #[test]
    fn test_kee_config_remove_nonexistent_account() {
        let mut config = KeeConfig::new();
        let removed = config.remove_account("nonexistent");
        assert!(removed.is_none());
    }

    #[test]
    fn test_kee_config_get_account() {
        let mut config = KeeConfig::new();
        let account = AccountInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            region: "us-east-1".to_string(),
            session_name: "test-session".to_string(),
        };

        config.add_account("test".to_string(), account.clone());

        assert_eq!(config.get_account("test"), Some(&account));
        assert_eq!(config.get_account("nonexistent"), None);
    }

    #[test]
    fn test_kee_config_list_accounts() {
        let mut config = KeeConfig::new();
        let account1 = AccountInfo {
            profile_name: "kee-test1".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            region: "us-east-1".to_string(),
            session_name: "test-session1".to_string(),
        };
        let account2 = AccountInfo {
            profile_name: "kee-test2".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-west-2".to_string(),
            sso_account_id: "123456789013".to_string(),
            sso_role_name: "TestRole2".to_string(),
            region: "us-west-2".to_string(),
            session_name: "test-session2".to_string(),
        };

        config.add_account("test1".to_string(), account1);
        config.add_account("test2".to_string(), account2);

        let accounts = config.list_accounts();
        assert_eq!(accounts.len(), 2);

        let account_names: Vec<&String> = accounts.iter().map(|(name, _)| *name).collect();
        assert!(account_names.contains(&&"test1".to_string()));
        assert!(account_names.contains(&&"test2".to_string()));
    }

    #[test]
    fn test_kee_config_set_current_account() {
        let mut config = KeeConfig::new();

        config.set_current_account(Some("test".to_string()));
        assert_eq!(config.current_account, Some("test".to_string()));

        config.set_current_account(None);
        assert!(config.current_account.is_none());
    }

    #[test]
    fn test_kee_config_serialization() {
        let mut config = KeeConfig::new();
        let account = AccountInfo {
            profile_name: "kee-test".to_string(),
            sso_start_url: "https://test.awsapps.com/start".to_string(),
            sso_region: "us-east-1".to_string(),
            sso_account_id: "123456789012".to_string(),
            sso_role_name: "TestRole".to_string(),
            region: "us-east-1".to_string(),
            session_name: "test-session".to_string(),
        };

        config.add_account("test".to_string(), account);
        config.set_current_account(Some("test".to_string()));

        let json = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: KeeConfig = serde_json::from_str(&json).unwrap();

        assert_eq!(config, deserialized);
    }

    #[test]
    fn test_format_profile_name() {
        assert_eq!(format_profile_name("test"), "kee-test");
        assert_eq!(format_profile_name("my-account"), "kee-my-account");
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
region = us-east-1

[profile other-profile]
region = us-west-2
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
        assert_eq!(section.get("region"), Some(&"us-east-1".to_string()));
    }

    #[test]
    fn test_parse_aws_config_section_nonexistent() {
        let config_content = r#"
[profile kee-test]
sso_start_url = https://test.awsapps.com/start
region = us-east-1
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
region = us-east-1
"#;

        let section = parse_aws_config_section(config_content, "profile kee-test").unwrap();

        // Should parse valid lines and ignore malformed ones
        assert_eq!(
            section.get("sso_start_url"),
            Some(&"https://test.awsapps.com/start".to_string())
        );
        assert_eq!(section.get("region"), Some(&"us-east-1".to_string()));
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
        assert_eq!(section.get("region"), Some(&"us-east-1".to_string()));
    }
}
