#!/etc/bash

red="\e[31m"
blank="\e[0m"
cyan="\e[36m"

clear
neofetch

echo ""
cowsay Hello $USER

echo -e "$red-------------------- address --------------------$blank"
add=$(ip route get 1.1.1.1 | grep -oP "src \K\S+")
echo "${add}"
echo ""
echo -e "$red-------------------- location --------------------$blank"
echo -e "You are here: \033[32m$PWD\033[0m"
echo ""
echo -e "$red-------------------- time --------------------$blank"
date
echo ""
echo -e "$red-------------------- info --------------------$blank"
echo "to see this screen again run: 'bash cls.sh'"
echo ""
echo ""
echo -e "$cyan#what's on your mind?:$blank"
