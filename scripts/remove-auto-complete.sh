#!/bin/bash
# Kee — Auto-completion uninstallation
# This script removes shell completions.

set -e

# ANSI color codes
BOLD_WHITE="\033[1;37m"
RESET="\033[0m"

# Function to remove bash completion
remove_bash_completion() {
    local completion_file="$HOME/.kee/.kee_completion.bash"

    # Remove completion file
    if [[ -f "$completion_file" ]]; then
        rm "$completion_file" 2>/dev/null
    fi

    # Remove source lines from shell configs
    local config_files=("$HOME/.bashrc" "$HOME/.bash_profile")
    for config_file in "${config_files[@]}"; do
        if [[ -f "$config_file" ]] && grep -q "\.kee_completion\.bash" "$config_file"; then
            grep -v "\.kee_completion\.bash" "$config_file" > "${config_file}.tmp" && mv "${config_file}.tmp" "$config_file"
        fi
    done
}

# Function to remove zsh completion
remove_zsh_completion() {
    local completion_file="$HOME/.kee/completions/_kee"
    local completion_dir="$HOME/.kee/completions"
    local config_file="$HOME/.zshrc"

    # Remove completion file
    if [[ -f "$completion_file" ]]; then
        rm "$completion_file" 2>/dev/null

        # Remove directory if empty
        if [[ -d "$completion_dir" ]] && [[ -z "$(ls -A "$completion_dir")" ]]; then
            rmdir "$completion_dir" 2>/dev/null
        fi
    fi

    # Remove configuration from .zshrc
    if [[ -f "$config_file" ]] && grep -q "\.kee/completions" "$config_file"; then
        # Create a temporary file without the kee completion lines
        awk '
            /# Kee completion/ { skip = 3; next }
            skip > 0 { skip--; next }
            { print }
        ' "$config_file" > "${config_file}.tmp" && mv "${config_file}.tmp" "$config_file"
    fi
}

# Function to remove fish completion
remove_fish_completion() {
    local completion_file="$HOME/.config/fish/completions/kee.fish"

    if [[ -f "$completion_file" ]]; then
        rm "$completion_file" 2>/dev/null
    fi
}

# Since we don't know which was installed...
remove_bash_completion
remove_zsh_completion
remove_fish_completion

echo -e "\n [✓] Shell auto-completions have been uninstalled!"
