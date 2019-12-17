#!/bin/bash

while read regex; do
    echo -n $regex ": "
    grep -e ^$regex\$ bin_nums.txt | wc -l
done < regex.txt
