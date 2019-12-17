#!/bin/bash

while read regex; do
    echo -n $regex ": "
    grep -e ^$regex\$ smallnums.txt | wc -l   #This greps smallnums.txt with each regex in eight_regex.txt
    #if [ $? -eq 0 ]; then
    #    echo -n Error
    #fi
    #echo \"\$$regex^\"
    #echo $regex
done < eight_regex.txt
