#!/etc/bash

#global shit like array and variables
ip_array=()
count=0
#this one look scary, but it's just cutting the result of running "ifcinfig eno1" to only display the ip
network=$(ifconfig eno1 | grep "inet " | cut -d " " -f 10)

#cool welcome message in a nice cyan color
echo -e "\e[36mWelcome to IP scanner!\e[0m"

#this gives the user sometihng to look at while the script runs its magic.
search() {
  echo -e "\e[32mYour address is:\e[0m"
  echo -e "\e[31m$network\e[0m"
  echo -e "\e[0mNow searching for addresses on the network: \e[35m${network%.*}.x\e[0m" #"\e[35${network%.*}\e[0m")
  #sleep 2.5
}

#This is where the magic actually happens.
scan() {
  for ip in {1..254}; do          #this is to go trough every address from 1 to 254 and put it in a variable named "ip"
    address="${network%.*}.${ip}" # this is to add the "ip" variable to the "network" varable so we get something like 192.168.20.IP

    ping -c 1 "${address}" >/dev/null 2>&1 #This is the thingy that pings an address and looks for a response

    if [ $? -ne 0 ]; then # this looks for adresses that did not respond, and adds those to an array
      ip_array+=("$address")
      echo ${address}
    fi

    ((count += 1)) #this is just to show how many times the code ran. other words, how many adresses was pinged.
  done
}

# This is where we call the functions and in order
search
wait
scan
wait

#and, can't forget the line that tells the user how many addresses was checked
echo -e "\e[33mscanned: $count addresses\e[0m"
