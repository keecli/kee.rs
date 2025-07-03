use kee::{AccountInfo, KeeConfig};
use serde_json;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_config_file_operations() {
    let temp_dir = TempDir::new().unwrap();
    let config_file = temp_dir.path().join("kee.json");

    // Create a test config
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

    // Save to file
    let json = serde_json::to_string_pretty(&config).unwrap();
    fs::write(&config_file, json).unwrap();

    // Load from file
    let loaded_json = fs::read_to_string(&config_file).unwrap();
    let loaded_config: KeeConfig = serde_json::from_str(&loaded_json).unwrap();

    // Verify
    assert_eq!(config, loaded_config);
    assert_eq!(loaded_config.get_account("test"), Some(&account));
    assert_eq!(loaded_config.current_account, Some("test".to_string()));
}

#[test]
fn test_aws_config_parsing() {
    let aws_config_content = r#"
[profile kee-test]
sso_start_url = https://test.awsapps.com/start
sso_region = us-east-1
sso_account_id = 123456789012
sso_role_name = TestRole
region = us-east-1
sso_session = test-session

[sso-session test-session]
sso_start_url = https://test.awsapps.com/start
sso_region = us-east-1

[profile other-profile]
region = us-west-2
"#;

    let section = kee::parse_aws_config_section(aws_config_content, "profile kee-test").unwrap();

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
    assert_eq!(
        section.get("sso_session"),
        Some(&"test-session".to_string())
    );
}

#[test]
fn test_multiple_accounts_management() {
    let mut config = KeeConfig::new();

    let account1 = AccountInfo {
        profile_name: "kee-prod".to_string(),
        sso_start_url: "https://prod.awsapps.com/start".to_string(),
        sso_region: "us-east-1".to_string(),
        sso_account_id: "111111111111".to_string(),
        sso_role_name: "ProdRole".to_string(),
        region: "us-east-1".to_string(),
        session_name: "prod-session".to_string(),
    };

    let account2 = AccountInfo {
        profile_name: "kee-dev".to_string(),
        sso_start_url: "https://dev.awsapps.com/start".to_string(),
        sso_region: "us-west-2".to_string(),
        sso_account_id: "222222222222".to_string(),
        sso_role_name: "DevRole".to_string(),
        region: "us-west-2".to_string(),
        session_name: "dev-session".to_string(),
    };

    // Add accounts
    config.add_account("prod".to_string(), account1.clone());
    config.add_account("dev".to_string(), account2.clone());

    // Test listing
    let accounts = config.list_accounts();
    assert_eq!(accounts.len(), 2);

    // Test getting specific accounts
    assert_eq!(config.get_account("prod"), Some(&account1));
    assert_eq!(config.get_account("dev"), Some(&account2));

    // Test setting current account
    config.set_current_account(Some("prod".to_string()));
    assert_eq!(config.current_account, Some("prod".to_string()));

    // Test removing account
    let removed = config.remove_account("dev");
    assert_eq!(removed, Some(account2));
    assert_eq!(config.accounts.len(), 1);
    assert!(config.get_account("dev").is_none());

    // Test removing current account clears current_account
    config.set_current_account(Some("prod".to_string()));
    let removed_current = config.remove_account("prod");
    assert_eq!(removed_current, Some(account1));
    assert!(config.current_account.is_none());
    assert!(config.is_empty());
}

#[test]
fn test_profile_name_formatting() {
    assert_eq!(kee::format_profile_name("production"), "kee-production");
    assert_eq!(
        kee::format_profile_name("dev-environment"),
        "kee-dev-environment"
    );
    assert_eq!(kee::format_profile_name("test123"), "kee-test123");
}

#[test]
fn test_config_serialization_roundtrip() {
    let mut original_config = KeeConfig::new();

    let account = AccountInfo {
        profile_name: "kee-test".to_string(),
        sso_start_url: "https://test.awsapps.com/start".to_string(),
        sso_region: "us-east-1".to_string(),
        sso_account_id: "123456789012".to_string(),
        sso_role_name: "TestRole".to_string(),
        region: "us-east-1".to_string(),
        session_name: "test-session".to_string(),
    };

    original_config.add_account("test".to_string(), account);
    original_config.set_current_account(Some("test".to_string()));

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&original_config).unwrap();

    // Deserialize back
    let deserialized_config: KeeConfig = serde_json::from_str(&json).unwrap();

    // Should be identical
    assert_eq!(original_config, deserialized_config);
}

#[test]
fn test_empty_config_operations() {
    let mut config = KeeConfig::new();

    assert!(config.is_empty());
    assert!(config.list_accounts().is_empty());
    assert!(config.get_account("nonexistent").is_none());
    assert!(config.remove_account("nonexistent").is_none());
    assert!(config.current_account.is_none());

    // Setting current account on empty config should work
    config.set_current_account(Some("test".to_string()));
    assert_eq!(config.current_account, Some("test".to_string()));

    // But config should still be considered empty (no accounts)
    assert!(config.is_empty());
}

#[cfg(feature = "integration-tests")]
mod integration_tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_binary_help_command() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--help"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("AWS CLI session manager"));
        assert!(stdout.contains("add"));
        assert!(stdout.contains("use"));
        assert!(stdout.contains("list"));
        assert!(stdout.contains("current"));
        assert!(stdout.contains("remove"));
    }

    #[test]
    fn test_binary_version_info() {
        let output = Command::new("cargo")
            .args(&["run", "--", "--version"])
            .output()
            .expect("Failed to execute command");

        assert!(output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("kee"));
    }
}
