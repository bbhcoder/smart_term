#!/bin/bash
set -e

echo -e "\e[33m[1/3]\e[0m Removing SmartTerm packages and binaries..."
if [ "$(uname)" == "Darwin" ]; then
    sudo pkgutil --forget com.bbhcoder.smartterm >/dev/null 2>&1 || true
    sudo rm -f /usr/local/bin/smart
elif command -v dpkg >/dev/null 2>&1; then
    sudo dpkg -r smart_term >/dev/null 2>&1 || true
    sudo rm -f /usr/local/bin/smart
elif command -v rpm >/dev/null 2>&1; then
    sudo rpm -e smart_term >/dev/null 2>&1 || true
    sudo rm -f /usr/local/bin/smart
fi

echo -e "\e[33m[2/3]\e[0m Cleaning up shell configuration files..."
for rc in ~/.bashrc ~/.zshrc ~/.bash_profile ~/.zprofile; do
    if [ -f "$rc" ]; then
        if [[ "$OSTYPE" == "darwin"* ]]; then
            sed -i '' '/# Launch SmartTerm/d' "$rc"
            sed -i '' '/# SmartTerm/d' "$rc"
            sed -i '' '/SMART_TERM_ACTIVE/d' "$rc"
        else
            sed -i '/# Launch SmartTerm/d' "$rc"
            sed -i '/# SmartTerm/d' "$rc"
            sed -i '/SMART_TERM_ACTIVE/d' "$rc"
        fi
    fi
done

echo -e "\e[33m[3/3]\e[0m (Optional) Do you want to delete your SmartTerm history database? (y/n)"
read -p "> " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    rm -f ~/.smart_term_history.sqlite
    echo -e "\e[32m[System]\e[0m History database removed."
fi

echo -e "\e[32m[Success]\e[0m SmartTerm has been completely uninstalled. Please close and reopen your terminal!"