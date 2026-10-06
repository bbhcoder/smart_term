#!/usr/bin/env bash
set -e
REPO="bbhcoder/smart_term2"
OS="$(uname -s)"
ARCH="$(uname -m)"
if [ "$OS" = "Linux" ]; then
    TARGET="x86_64-unknown-linux-gnu"
elif [ "$OS" = "Darwin" ]; then
    if [ "$ARCH" = "arm64" ] || [ "$ARCH" = "aarch64" ]; then
        TARGET="aarch64-apple-darwin"
    else
        TARGET="x86_64-apple-darwin"
    fi
else
    echo "Unsupported OS: $OS"
    exit 1
fi
URL="https://github.com/$REPO/releases/latest/download/smart_term-${TARGET}.tar.gz"
TMP_DIR=$(mktemp -d)
echo -e "\033[1;34mDownloading SmartTerm ($TARGET)...\033[0m"
curl -sSL "$URL" | tar -xz -C "$TMP_DIR"
echo -e "\033[1;33mInstalling binaries to /usr/local/bin (requires sudo)\033[0m"
sudo mv "$TMP_DIR/smart" /usr/local/bin/
sudo mv "$TMP_DIR/smartd" /usr/local/bin/
sudo chmod +x /usr/local/bin/smart /usr/local/bin/smartd
SHELL_NAME=$(basename "$SHELL")
RC_FILE=""
if [ "$SHELL_NAME" = "zsh" ]; then
    RC_FILE="$HOME/.zshrc"
elif [ "$SHELL_NAME" = "bash" ]; then
    RC_FILE="$HOME/.bashrc"
fi
if [ -n "$RC_FILE" ]; then
    if ! grep -q 'smart init' "$RC_FILE"; then
        echo -e "\neval \"\$(smart init $SHELL_NAME)\"" >> "$RC_FILE"
        echo -e "\033[1;32mHook added to $RC_FILE\033[0m"
    fi
fi
echo -e "\033[1;32mSmartTerm installed successfully! Please restart your terminal.\033[0m"
