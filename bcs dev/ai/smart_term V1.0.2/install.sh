#!/bin/bash
echo -e "\e[32m[1/3]\e[0m Building Smart Term (Release mode)..."
cargo build --release

echo -e "\e[32m[2/3]\e[0m Installing to /usr/local/bin/smart..."
# کپی کردن فایل اجرایی به مسیر سیستمی لینوکس/مک
sudo cp target/release/smart_term /usr/local/bin/smart
sudo chmod +x /usr/local/bin/smart

echo -e "\e[32m[3/3]\e[0m \e[1mInstallation Complete!\e[0m"
echo -e "You can now launch the terminal from anywhere by typing: \e[36msmart\e[0m"