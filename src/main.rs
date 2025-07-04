use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

const BOLD_WHITE: &str = "\x1b[1;37m";
const RESET: &str = "\x1b[0m";
const KEE_ART: &str = r#"
    ██╗  ██╗███████╗███████╗
    ██║ ██╔╝██╔════╝██╔════╝
    █████╔╝ █████╗  █████╗
    ██╔═██╗ ██╔══╝  ██╔══╝
    ██║  ██╗███████╗███████╗
    ╚═╝  ╚═╝╚══════╝╚══════╝

    AWS CLI session manager"#;

#[derive(Parser)]
#[command(name = "kee")]
#[command(about = KEE_ART)]
#[command(long_about = format!("{KEE_ART}\n\nExamples:\n  kee add myaccount          Add a new AWS account\n  kee use myaccount          Use an account (starts sub-shell)\n  kee list                   List all configured accounts\n  kee current                Show current active account\n  kee remove myaccount       Remove an account configuration"))]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new AWS account
    Add { account_name: String },
    /// Use an account
    Use { account_name: String },
    /// List all configured accounts
    List,
    /// Show current active account
    Current,
    /// Remove an account
    Remove { account_name: String },
}

#[derive(Serialize, Deserialize, Clone)]
struct AccountInfo {
    profile_name: String,
    sso_start_url: String,
    sso_region: String,
    sso_account_id: String,
    sso_role_name: String,
    session_name: String,
}

#[derive(Serialize, Deserialize, Default)]
struct KeeConfig {
    accounts: HashMap<String, AccountInfo>,
    current_account: Option<String>,
}

struct KeeManager {
    config_file: PathBuf,
    aws_config_file: PathBuf,
}

fn hlt(text: &str) -> String {
    format!("{BOLD_WHITE}{text}{RESET}")
}

impl KeeManager {
    fn new() -> io::Result<Self> {
        let home_dir = dirs::home_dir().ok_or_else(|| {
            io::Error::new(io::ErrorKind::NotFound, "Could not find home directory")
        })?;

        let config_dir = home_dir.join(".aws");
        let config_file = config_dir.join("kee.json");
        let aws_config_file = config_dir.join("config");

        // Create .aws directory if it doesn't exist
        fs::create_dir_all(&config_dir)?;

        Ok(Self {
            config_file,
            aws_config_file,
        })
    }

    fn load_config(&self) -> KeeConfig {
        if !self.config_file.exists() {
            return KeeConfig::default();
        }

        match fs::read_to_string(&self.config_file) {
            Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
            Err(_) => KeeConfig::default(),
        }
    }

    fn save_config(&self, config: &KeeConfig) -> io::Result<()> {
        let content = serde_json::to_string_pretty(config)?;
        fs::write(&self.config_file, content)
    }

    fn add_account(&self, account_name: &str) -> io::Result<bool> {
        let profile_name = account_name;

        println!("\n Starting SSO configuration...");
        println!(" (This will open your browser to complete authentication.)");
        println!("\n Follow the prompts:");
        println!("  {} Enter your SSO start URL", hlt("1."));
        println!("  {} Enter your SSO region", hlt("2."));
        println!("  {} Authenticate in your browser", hlt("3."));
        println!("  {} Select your AWS account", hlt("4."));
        println!("  {} Select your role", hlt("5."));
        println!(
            "  {} Choose your output format (recommend: json)",
            hlt("6.")
        );
        println!(
            "  {} Choose your output format (recommend: json)",
            hlt("7.")
        );
        println!(
            "\n {} When prompted for 'session name', use: {}\n",
            hlt("Tip:"),
            hlt(account_name)
        );

        // Run aws configure sso
        let status = Command::new("aws")
            .args(["configure", "sso", "--profile", profile_name])
            .status()?;

        if !status.success() {
            println!(" [X] SSO configuration failed.");
            return Ok(false);
        }

        println!(
            " {} You can ignore the AWS CLI example above. {} will handle profiles for you.",
            hlt("Note:"),
            hlt("Kee")
        );

        // Reformat the AWS config file
        self.reformat_config_file()?;

        // Read profile info
        let profile_info = match self.read_profile_info(profile_name) {
            Some(info) => info,
            None => {
                println!(" [X] Could not read profile information.");
                return Ok(false);
            }
        };

        // Save to kee config
        let mut config = self.load_config();
        config
            .accounts
            .insert(account_name.to_string(), profile_info);
        self.save_config(&config)?;

        // Test the profile
        if self.check_credentials(profile_name) {
            println!("\n [✓] The account was added and is working correctly! — I just tested it.");
        } else {
            println!("\n [X] I created the profile but credentials may need a refresh...");
            println!(" {} aws sso login --profile {}", hlt("Try:"), profile_name);
        }

        Ok(true)
    }

    fn read_profile_info(&self, profile_name: &str) -> Option<AccountInfo> {
        if !self.aws_config_file.exists() {
            return None;
        }

        let content = fs::read_to_string(&self.aws_config_file).ok()?;
        let mut config = configparser::ini::Ini::new();
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

        // Handle SSO session format - get sso_start_url and sso_region from sso-session section
        let (sso_start_url, sso_region) = if !session_name.is_empty() {
            let sso_section_name = format!("sso-session {session_name}");
            if let Some(sso_section) = config.get_map_ref().get(&sso_section_name) {
                let start_url = sso_section
                    .get("sso_start_url")
                    .and_then(|s| s.as_ref())
                    .cloned()
                    .unwrap_or_default();
                let region = sso_section
                    .get("sso_region")
                    .and_then(|s| s.as_ref())
                    .cloned()
                    .unwrap_or_default();
                (start_url, region)
            } else {
                (String::new(), String::new())
            }
        } else {
            // Legacy format - try to get from profile section
            let start_url = section
                .get("sso_start_url")
                .and_then(|s| s.as_ref())
                .cloned()
                .unwrap_or_default();
            let sso_region = section
                .get("sso_region")
                .and_then(|s| s.as_ref())
                .cloned()
                .unwrap_or_default();
            (start_url, sso_region)
        };

        Some(AccountInfo {
            profile_name: profile_name.to_string(),
            sso_start_url,
            sso_region,
            sso_account_id,
            sso_role_name,
            session_name,
        })
    }

    fn list_accounts(&self) {
        let config = self.load_config();

        if config.accounts.is_empty() {
            println!(
                "\n No accounts configured. Use '{}' to add an account.",
                hlt("kee add <account_name>")
            );
            return;
        }

        println!();
        for (account_name, account_info) in &config.accounts {
            let status = if Some(account_name.as_str()) == config.current_account.as_deref() {
                " (Current session)"
            } else {
                ""
            };

            println!(" {}{}", hlt(account_name), status);
            println!(" • {} {}", hlt("Account:"), account_info.sso_account_id);
            println!(" • {} {}", hlt("Role:"), account_info.sso_role_name);
            println!();
        }
    }

    fn remove_account(&self, account_name: &str) -> io::Result<bool> {
        let mut config = self.load_config();

        if !config.accounts.contains_key(account_name) {
            println!("\n Account '{}' not found.", hlt(account_name));
            return Ok(false);
        }

        // Confirm removal
        print!(
            "\n Are you sure you want to remove account '{}'? (y/N): ",
            hlt(account_name)
        );
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() != "y" {
            return Ok(false);
        }

        // Get account info before removal
        let account_info = config.accounts.get(account_name).unwrap().clone();

        // Remove from config
        config.accounts.remove(account_name);

        // Clear current account if it's the one being removed
        if config.current_account.as_deref() == Some(account_name) {
            config.current_account = None;
        }

        self.save_config(&config)?;

        // Remove AWS profile and session
        let hlt_account = hlt(account_name);
        match self.remove_aws_profile(&account_info.profile_name) {
            Ok(_) => {
                if !account_info.session_name.is_empty() {
                    let _ = self.remove_sso_session(&account_info.session_name);
                    println!(
                        " [✓] Account '{hlt_account}', AWS profile '{hlt_account}', and SSO session '{hlt_account}' removed."
                    );
                } else {
                    println!(
                        " [✓] Account '{hlt_account}' and AWS profile '{hlt_account}' removed."
                    );
                }
            }
            Err(e) => {
                println!(" [✓] Account '{hlt_account}' removed from Kee.");
                println!(
                    " [!] {} Could not remove AWS profile '{}': {}",
                    hlt("Warning:"),
                    hlt(&account_info.profile_name),
                    e
                );
                println!(" You may want to remove it manually from ~/.aws/config");
            }
        }

        Ok(true)
    }

    fn use_account(&self, account_name: &str) -> io::Result<bool> {
        // Check if already in a kee session
        if env::var("KEE_ACTIVE_SESSION").is_ok() {
            let current_session =
                env::var("KEE_CURRENT_ACCOUNT").unwrap_or_else(|_| "unknown".to_string());
            println!(
                "\n You already are in a {} session for: {}",
                hlt("Kee"),
                hlt(&current_session)
            );
            println!(
                " Exit the current session first by typing '{}'",
                hlt("exit")
            );
            return Ok(false);
        }

        let mut config = self.load_config();
        let hlt_account = hlt(account_name);

        if !config.accounts.contains_key(account_name) {
            println!("\n Account '{hlt_account}' not found.");

            if !config.accounts.is_empty() {
                println!(" Available accounts:");
                for name in config.accounts.keys() {
                    println!(" • {}\n", hlt(name));
                }
            }

            // Offer to add the account
            print!(" Would you like to add account '{hlt_account}' now? (y/N): ");
            io::stdout().flush()?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;

            if input.trim().to_lowercase() == "y" {
                if self.add_account(account_name)? {
                    print!(" Would you like to use account '{hlt_account}' now? (y/N): ");
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;

                    if input.trim().to_lowercase() == "y" {
                        // Reload config
                        config = self.load_config();
                    } else {
                        println!(
                            "\n Account '{}' is ready to use. Run '{}' when needed.",
                            hlt_account,
                            hlt(&format!("kee use {account_name}"))
                        );
                        return Ok(true);
                    }
                } else {
                    println!(" Failed to add account '{hlt_account}'.");
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }

        let account_info = config.accounts.get(account_name).unwrap();
        let profile_name = &account_info.profile_name;

        // Check credentials
        if !self.check_credentials(profile_name) {
            println!(" Credentials expired or not available. Attempting SSO login...");
            if !self.sso_login(profile_name)? {
                println!(
                    " Failed to authenticate. Please run '{}' manually.",
                    hlt("aws sso login")
                );
                return Ok(false);
            }
        }

        // Update current account
        config.current_account = Some(account_name.to_string());
        self.save_config(&config)?;

        // Start subshell
        self.start_subshell(account_name, profile_name)?;

        // Clear current account when subshell exits
        config.current_account = None;
        self.save_config(&config)?;

        Ok(true)
    }

    fn current_account(&self) {
        // Check if in active session
        if let Ok(current) = env::var("KEE_CURRENT_ACCOUNT") {
            println!("\n Current session: {}", hlt(&current));
            println!(" Type '{}' to return to your main shell.", hlt("exit"));
        } else {
            let config = self.load_config();
            match config.current_account {
                Some(current) => println!("\n Current account: {}", hlt(&current)),
                None => println!("\n No account is currently active."),
            }
        }
    }

    fn check_credentials(&self, profile_name: &str) -> bool {
        match Command::new("aws")
            .args(["sts", "get-caller-identity", "--profile", profile_name])
            .env("AWS_CLI_AUTO_PROMPT", "off")
            .env("AWS_PAGER", "")
            .output()
        {
            Ok(output) => output.status.success(),
            Err(_) => false,
        }
    }

    fn sso_login(&self, profile_name: &str) -> io::Result<bool> {
        let status = Command::new("aws")
            .args(["sso", "login", "--profile", profile_name])
            .status()?;

        Ok(status.success())
    }

    fn start_subshell(&self, account_name: &str, profile_name: &str) -> io::Result<()> {
        // Get current shell
        let shell = if cfg!(windows) {
            env::var("COMSPEC").unwrap_or_else(|_| "cmd.exe".to_string())
        } else {
            env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string())
        };

        // Show banner
        println!("{KEE_ART}");
        println!("    Session: {}", hlt(account_name));
        println!("\n    Starting a sub-shell for this session...");
        println!("    Type '{}' to return to your main shell.", hlt("exit"));

        // Start subshell with environment
        let mut cmd = Command::new(&shell);
        cmd.env("AWS_PROFILE", profile_name);
        cmd.env("KEE_CURRENT_ACCOUNT", account_name);
        cmd.env("KEE_ACTIVE_SESSION", "1");

        // Update PS1 for Unix-like systems
        if !cfg!(windows) {
            if let Ok(ps1) = env::var("PS1") {
                cmd.env("PS1", format!("(kee:{account_name}) {ps1}"));
            } else {
                cmd.env("PS1", format!("(kee:{account_name}) $ "));
            }
        }

        let _ = cmd.status();

        println!("\n {} — Session ended.", hlt(account_name));
        Ok(())
    }

    fn reformat_config_file(&self) -> io::Result<()> {
        if !self.aws_config_file.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.aws_config_file)?;
        let mut config = configparser::ini::Ini::new();
        config
            .read(content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

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

    fn remove_aws_profile(&self, profile_name: &str) -> io::Result<()> {
        if !self.aws_config_file.exists() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.aws_config_file)?;
        let mut config = configparser::ini::Ini::new();
        config
            .read(content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let section_name = format!("profile {profile_name}");
        config.remove_section(&section_name);

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

    fn remove_sso_session(&self, session_name: &str) -> io::Result<()> {
        if !self.aws_config_file.exists() || session_name.is_empty() {
            return Ok(());
        }

        let content = fs::read_to_string(&self.aws_config_file)?;
        let mut config = configparser::ini::Ini::new();
        config
            .read(content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        let section_name = format!("sso-session {session_name}");
        config.remove_section(&section_name);

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
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let kee = KeeManager::new()?;

    match cli.command {
        Commands::Add { account_name } => {
            kee.add_account(&account_name)?;
        }
        Commands::Use { account_name } => {
            kee.use_account(&account_name)?;
        }
        Commands::List => {
            kee.list_accounts();
        }
        Commands::Current => {
            kee.current_account();
        }
        Commands::Remove { account_name } => {
            kee.remove_account(&account_name)?;
        }
    }

    Ok(())
}
