#!/bin/bash
set -e

echo -e "\e[32m[1/4]\e[0m Detecting OS and native downloaders..."
dl() {
    if command -v curl >/dev/null 2>&1; then curl -L "$1" -o "$2"
    elif command -v wget >/dev/null 2>&1; then wget -O "$2" "$1"
    elif command -v python3 >/dev/null 2>&1; then python3 -c "import urllib.request; urllib.request.urlretrieve('$1', '$2')"
    else echo -e "\e[31mError: No native downloader found.\e[0m"; exit 1; fi
}

if [ "$(uname)" == "Darwin" ]; then
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
read -p "Do you want SmartTerm to launch automatically in new terminals? (y/n) " -n 1 -r
echo

if [[ $REPLY =~ ^[Yy]$ ]]; then
    if command -v smart >/dev/null 2>&1; then
        smart org
    elif [ -x "/usr/local/bin/smart" ]; then
        /usr/local/bin/smart org
    fi
fi

echo -e "\e[32m[Success]\e[0m Installation complete! Open a new tab or type 'smart' to launch."