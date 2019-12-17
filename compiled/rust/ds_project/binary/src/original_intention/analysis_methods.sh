#!/bin/bash

i=0
max=256
while [[ $i -le $max ]]; do
    echo -n "$i, " >> times_seen.txt
    grep "$i"'$' data.txt | wc -l >> times_seen.txt
    (( i++ ))
done
