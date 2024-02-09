# OS: RaspOS 64 Lite

# config.txt
# Don't hesite to remove stuff from config.txt (not everything though)
# Modify /boot/config.txt to add:
# dtoverlay=pwm-2chan,pin=12,func=4,pin2=13,func2=4

# Update / Upgrade
sudo apt update && sudo apt upgrade

# Rust
sudo apt install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# inputbot
sudo apt install -y libx11-dev libxtst-dev libudev-dev libinput-dev

