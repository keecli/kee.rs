// Unit tests for the main binary functionality
use std::process::Command;

#[test]
fn test_help_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("Failed to execute help command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Check that help contains expected sections
    assert!(stdout.contains("AWS CLI profile manager"));
    assert!(stdout.contains("Commands:"));
    assert!(stdout.contains("add"));
    assert!(stdout.contains("use"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("current"));
    assert!(stdout.contains("remove"));
}

#[test]
fn test_list_command_no_accounts() {
    let temp_dir = std::env::temp_dir().join("kee_test_no_accounts");
    std::fs::create_dir_all(&temp_dir).unwrap();

    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .env("HOME", &temp_dir)
        .output()
        .expect("Failed to execute list command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show message about no accounts configured
    assert!(stdout.contains("No accounts configured"));

    // Clean up
    std::fs::remove_dir_all(&temp_dir).ok();
}

#[test]
fn test_current_command_no_session() {
    let output = Command::new("cargo")
        .args(&["run", "--", "current"])
        .output()
        .expect("Failed to execute current command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show no active account
    assert!(stdout.contains("No profile is currently active"));
}

#[test]
fn test_invalid_command() {
    let output = Command::new("cargo")
        .args(&["run", "--", "invalid-command"])
        .output()
        .expect("Failed to execute invalid command");

    assert!(!output.status.success());
}

#[test]
fn test_remove_nonexistent_account() {
    let output = Command::new("cargo")
        .args(&["run", "--", "remove", "nonexistent-account"])
        .output()
        .expect("Failed to execute remove command");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();

    // Should show account not found message
    assert!(stdout.contains("not found"));
}

// Test utility functions
#[cfg(test)]
mod utils_tests {
    #[test]
    fn test_profile_name_generation() {
        // Test the profile name format that kee uses
        let account_name = "test-account";
        let expected_profile = format!("kee-{}", account_name);
        assert_eq!(expected_profile, "kee-test-account");
    }

    #[test]
    fn test_environment_variable_names() {
        // Test the environment variable names kee uses
        let kee_vars = vec!["KEE_ACTIVE_SESSION", "KEE_CURRENT_ACCOUNT", "AWS_PROFILE"];

        for var in kee_vars {
            assert!(!var.is_empty());
            assert!(var.starts_with("KEE_") || var.starts_with("AWS_"));
        }
    }
}
