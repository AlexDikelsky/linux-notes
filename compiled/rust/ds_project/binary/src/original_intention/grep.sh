#!/bin/bash

while read regex; do
    echo -n $regex", "
    #grep "\'"'^'$regex'$'"\'" nums.txt | wc -l
    grep '^'$regex'$' nums.txt | wc -l
    
done < regex.txt
