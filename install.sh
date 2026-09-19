#!/bin/bash
set -e
if [ -t 0 ]; then :; else CLEANED=$(cat | tr -d '\r' | tr -d '\000' | sed 's/[^[:print:]\t]//g'); exec bash <(echo "$CLEANED"); fi
echo -e "\e[32m[1/4]\e[0m Detecting OS and native downloaders..."
dl() {
    if command -v curl >/dev/null 2>&1; then curl -L "$1" -o "$2"
    elif command -v wget >/dev/null 2>&1; then wget -O "$2" "$1"
    elif command -v python3 >/dev/null 2>&1; then python3 -c "import urllib.request; urllib.request.urlretrieve('$1', '$2')"
    else echo -e "\e[31mError: No native downloader found.\e[0m"; exit 1; fi
}
if [ "$(uname)" = "Darwin" ]; then
    echo -e "\e[32m[2/4]\e[0m Downloading SmartTerm for macOS..."
    dl "https://github.com/bbhcoder/smart_term/releases/latest/download/smart_term-macos.pkg" "smart_term.pkg"
    echo -e "\e[32m[3/4]\e[0m Installing..."
    sudo installer -pkg smart_term.pkg -target /
elif command -v dpkg >/dev/null 2>&1; then
    echo -e "\e[32m[2/4]\e[0m Downloading SmartTerm for Debian/Ubuntu..."
    dl "https://github.com/bbhcoder/smart_term/releases/latest/download/smart_term-linux.deb" "smart_term.deb"
    echo -e "\e[32m[3/4]\e[0m Installing..."
    sudo dpkg -i smart_term.deb
else
    echo -e "\e[32m[2/4]\e[0m Downloading SmartTerm for Fedora/RHEL..."
    dl "https://github.com/bbhcoder/smart_term/releases/latest/download/smart_term-linux.rpm" "smart_term.rpm"
    echo -e "\e[32m[3/4]\e[0m Installing..."
    sudo rpm -i smart_term.rpm
fi
echo -e "\e[32m[4/4]\e[0m Configuration"
BIN=""
if command -v smart >/dev/null 2>&1; then BIN="smart"
elif command -v smart_term >/dev/null 2>&1; then BIN="smart_term"
elif [ -x "/usr/bin/smart_term" ]; then BIN="/usr/bin/smart_term"
elif [ -x "/usr/local/bin/smart_term" ]; then BIN="/usr/local/bin/smart_term"
fi
if [ -z "$BIN" ]; then
    if [ -x "/usr/bin/smart_term" ]; then sudo ln -sf /usr/bin/smart_term /usr/local/bin/smart; BIN="smart"
    elif [ -x "/usr/local/bin/smart_term" ]; then sudo ln -sf /usr/local/bin/smart_term /usr/local/bin/smart; BIN="smart"
    fi
fi
if [ -z "$BIN" ]; then echo -e "\e[31mError: SmartTerm binary not found.\e[0m"; exit 1; fi
read -p "Do you want SmartTerm to launch automatically in new terminals? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then "$BIN" org; fi
echo -e "\e[32m[Success]\e[0m Installation complete! Type 'smart' or 'smart_term' to launch."
