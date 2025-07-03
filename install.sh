#!/bin/bash
# Kee — Rust Installation Script

set -e

echo " Installing Kee (Rust)..."

# Install the binary
cargo install --path .

# Detect shell and add to PATH if not already present
CARGO_BIN="$HOME/.cargo/bin"

# Function to add to PATH in shell config
add_to_path() {
    local shell_config="$1"
    local shell_name="$2"

    if [[ -f "$shell_config" ]]; then
        if ! grep -q "\.cargo/bin" "$shell_config"; then
            echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> "$shell_config"
            echo " [✓] Added ~/.cargo/bin to PATH ($shell_name)"
        else
            echo " [✓] ~/.cargo/bin already in PATH ($shell_name)"
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
            fi
        fi
        ;;
    *)
        echo " Sorry. I am not familiar with your shell: $SHELL"
        echo " Please manually add ~/.cargo/bin to your PATH"
        ;;
esac

echo ""
echo " 🦀 Kee was installed successfully!"
echo " Restart your terminal or run:"
echo "   source ~/.zshrc  # (or your shell's config file)"
echo ""
