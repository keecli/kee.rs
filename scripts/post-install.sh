#!/bin/bash
# Kee — Post-installation
# This script runs after Kee is installed to set up shell completions

set -e

# Detect the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "a"
# Run the completion installation script
if [[ -f "$SCRIPT_DIR/install-auto-complete.sh" ]]; then
    echo "a"
    "$SCRIPT_DIR/install-auto-complete.sh"
    echo "b"
else
    echo " [!] The auto-completion installation script is not here."
    echo "     You can manually install auto-completions later with:"
    echo "     ./scripts/install-auto-complete.sh"
fi
