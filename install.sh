#!/bin/bash
# Kee — Installation script

set -e

# ANSI color codes
BOLD_WHITE="\033[1;37m"
RESET="\033[0m"

echo ""
echo -e " Installing ${BOLD_WHITE}Kee${RESET}..."

# Install the binary
cargo install --path . --quiet

# Detect shell and add to PATH if not already present
CARGO_BIN="$HOME/.cargo/bin"
PATH_ALREADY_CONFIGURED=false

# Function to add to PATH in shell config
add_to_path() {
    local shell_config="$1"
    local shell_name="$2"

    if [[ -f "$shell_config" ]]; then
        if ! grep -q "\.cargo/bin" "$shell_config"; then
            echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$shell_config"
            echo ""
            echo " [✓] Added ~/.cargo/bin to PATH ($shell_name)"
        else
            echo ""
            echo " [✓] ~/.cargo/bin already in PATH ($shell_name)"
            PATH_ALREADY_CONFIGURED=true
        fi
    fi
}

# Check current shell and add to appropriate config
case "$SHELL" in
    */zsh)
        add_to_path "$HOME/.zshrc" "zsh"
        ;;
    */bash)
        add_to_path "$HOME/.bashrc" "bash"
        ;;
    */fish)
        if [[ -d "$HOME/.config/fish" ]]; then
            FISH_CONFIG="$HOME/.config/fish/config.fish"
            if ! grep -q "\.cargo/bin" "$FISH_CONFIG" 2>/dev/null; then
                echo 'set -gx PATH $HOME/.cargo/bin $PATH' >> "$FISH_CONFIG"
                echo " [✓] Added to fish config"
            else
                echo " [✓] ~/.cargo/bin already in PATH (fish)"
                PATH_ALREADY_CONFIGURED=true
            fi
        fi
        ;;
    *)
        echo " Sorry. I am not familiar with your shell: $SHELL"
        echo " Please manually add ~/.cargo/bin to your PATH"
        ;;
esac


# Install shell auto-completion
if [[ -f "./scripts/install-auto-complete.sh" ]]; then
    ./scripts/install-auto-complete.sh
else
    echo " [!] Auto-completion installation script not found."
    echo "     Manually install with: ./scripts/install-auto-complete.sh"
fi

echo ""
echo -e " [✓] ${BOLD_WHITE}Kee${RESET} was installed successfully!"

# Check if .cargo/bin is currently in PATH or if kee command is available
if command -v kee >/dev/null 2>&1 || [[ ":$PATH:" == *":$HOME/.cargo/bin:"* ]] || [[ ":$PATH:" == *":.cargo/bin:"* ]]; then
    echo -e "     ${BOLD_WHITE}Kee${RESET} is ready for you. Type: ${BOLD_WHITE}kee add <account>${RESET} to add your first account."
elif [[ "$PATH_ALREADY_CONFIGURED" == true ]]; then
    SOURCE=""
    case "$SHELL" in
        */zsh)
            SOURCE="source ~/.zshrc"
            ;;
        */bash)
            SOURCE="source ~/.bashrc"
            ;;
        */fish)
            SOURCE="source ~/.config/fish/config.fish"
            ;;
        *)
            SOURCE="source your shell's config file."
            ;;
    esac
    echo -e "     Restart your terminal or run: ${SOURCE}"
    echo -e "     Then type: ${BOLD_WHITE}kee add <account>${RESET} to add your first account."
else
    echo -e "     Restart your terminal or run: ${BOLD_WHITE}source ~/.zshrc${RESET}"
    echo -e "     Then type: ${BOLD_WHITE}kee add <account>${RESET} to add your first account."
fi
