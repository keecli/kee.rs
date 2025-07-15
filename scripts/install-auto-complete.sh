#!/bin/bash
# Kee — Auto-completion installation
# This script installs shell completions for the user's current shell only.

set -e

# ANSI color codes
BOLD_WHITE="\033[1;37m"
RESET="\033[0m"

# Detect the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPLETIONS_DIR="$(dirname "$SCRIPT_DIR")/completions"


# Function to detect current shell
detect_shell() {
    if [[ -n "$SHELL" ]]; then
        case "$SHELL" in
            */bash)
                echo "bash"
                return 0
                ;;
            */zsh)
                echo "zsh"
                return 0
                ;;
            */fish)
                echo "fish"
                return 0
                ;;
        esac
    fi

    # Fallback: check for shell config files
    if [[ -f "$HOME/.zshrc" ]]; then
        echo "zsh"
        return 0
    elif [[ -f "$HOME/.bashrc" || -f "$HOME/.bash_profile" ]]; then
        echo "bash"
        return 0
    elif [[ -f "$HOME/.config/fish/config.fish" ]]; then
        echo "fish"
        return 0
    fi

    echo "unknown"
    return 1
}

# Function to install bash completion
install_bash_completion() {
    local completion_file="$HOME/.kee/.kee_completion.bash"
    local source_line="source ~/.kee/.kee_completion.bash"

    mkdir -p "$HOME/.kee"

    # Copy completion file
    if cp "$COMPLETIONS_DIR/kee.bash" "$completion_file" 2>/dev/null; then
        echo -e " [✓] Bash auto-completion installed to ${BOLD_WHITE}~/.kee/.kee_completion.bash${RESET}"

        # Add source line to shell config
        local config_files=("$HOME/.bashrc" "$HOME/.bash_profile")
        local added=false

        for config_file in "${config_files[@]}"; do
            if [[ -f "$config_file" ]]; then
                added=true
                if ! grep -q "\.kee_completion\.bash" "$config_file"; then
                    echo "$source_line" >> "$config_file"
                    break
                else
                    break
                fi
            fi
        done

        # If no config file exists, create .bash_profile
        if [[ "$added" == false ]]; then
            echo "$source_line" >> "$HOME/.bash_profile"
        fi

        return 0
    else
        echo -e " [X] Failed to install bash auto-completion."
        return 1
    fi
}

# Function to install zsh completion (Docker approach)
install_zsh_completion() {
    local completion_dir="$HOME/.kee/completions"
    local completion_file="$completion_dir/_kee"
    local config_file="$HOME/.zshrc"

    mkdir -p "$completion_dir"

    # Copy completion file
    if cp "$COMPLETIONS_DIR/_kee" "$completion_file" 2>/dev/null; then
        # Add fpath configuration to .zshrc
        local fpath_line="fpath=(~/.kee/completions \$fpath)"
        local compinit_line="autoload -Uz compinit && compinit"

        if [[ -f "$config_file" ]]; then
            if ! grep -q "\.kee/completions" "$config_file"; then
                echo "" >> "$config_file"
                echo "# Kee completion" >> "$config_file"
                echo "$fpath_line" >> "$config_file"
                echo "$compinit_line" >> "$config_file"
            fi
        else
            # Create .zshrc if it doesn't exist
            echo "# Kee completion" > "$config_file"
            echo "$fpath_line" >> "$config_file"
            echo "$compinit_line" >> "$config_file"
        fi

        return 0
    else
        echo -e " [X] Failed to install zsh auto-completion."
        return 1
    fi
}

# Function to install fish completion
install_fish_completion() {
    local completion_dir="$HOME/.config/fish/completions"
    local completion_file="$completion_dir/kee.fish"

    mkdir -p "$completion_dir"

    # Copy completion file
    if cp "$COMPLETIONS_DIR/kee.fish" "$completion_file" 2>/dev/null; then
        return 0
    else
        echo -e " [X] Failed to install fish auto-completion."
        return 1
    fi
}

# Check if completion files exist
if [[ ! -d "$COMPLETIONS_DIR" ]]; then
    echo -e " [X] Auto-completion files not found at $COMPLETIONS_DIR"
else
    # Detect shell
    CURRENT_SHELL=$(detect_shell)
    case "$CURRENT_SHELL" in
        bash)
            if install_bash_completion; then
                echo -e " [✓] Bash completion installed successfully!"
                echo -e "     Restart your terminal or run: ${BOLD_WHITE}source ~/.bashrc${RESET}"
            else
                exit 1
            fi
            ;;
        zsh)
            if install_zsh_completion; then
                echo -e " [✓] Zsh completion installed successfully!"
                echo -e "     Restart your terminal or run: ${BOLD_WHITE}source ~/.zshrc${RESET}"
            else
                exit 1
            fi
            ;;
        fish)
            if install_fish_completion; then
                echo -e " [✓] Fish completion installed successfully!"
                echo -e "     Restart your terminal for completions to take effect"
            else
                exit 1
            fi
            ;;
        unknown)
            echo -e " [X] Could not detect your shell type"
            echo -e "     Supported shells: ${BOLD_WHITE}bash${RESET}, ${BOLD_WHITE}zsh${RESET}, ${BOLD_WHITE}fish${RESET}"
            exit 1
            ;;
esac
fi
