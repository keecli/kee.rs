# Fish completion for kee

# Commands
complete -c kee -n "__fish_use_subcommand" -a "add" -d "Add a new AWS account"
complete -c kee -n "__fish_use_subcommand" -a "use" -d "Use an account"
complete -c kee -n "__fish_use_subcommand" -a "ls" -d "List all configured accounts"
complete -c kee -n "__fish_use_subcommand" -a "current" -d "Show current active account"
complete -c kee -n "__fish_use_subcommand" -a "rm" -d "Remove an account"

# Account names for use and rm commands
complete -c kee -n "__fish_seen_subcommand_from use rm" -a "(kee ls --names 2>/dev/null)"

# Flags for ls command
complete -c kee -n "__fish_seen_subcommand_from ls" -l names -d "Only show account names"
complete -c kee -n "__fish_seen_subcommand_from ls" -l help -d "Show help information"
