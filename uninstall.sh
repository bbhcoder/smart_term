#!/usr/bin/env bash
set -e
echo -e "\033[1;31mUninstalling SmartTerm...\033[0m"
pkill -f smartd || true
sudo rm -f /usr/local/bin/smart
sudo rm -f /usr/local/bin/smartd
if [ -f "$HOME/.bashrc" ]; then
    sed -i '/smart init/d' "$HOME/.bashrc"
fi
if [ -f "$HOME/.zshrc" ]; then
    sed -i '/smart init/d' "$HOME/.zshrc"
fi
echo -e "\033[1;32mSmartTerm removed. Database kept safely at ~/.smart_dev/smart_term.db\033[0m"
