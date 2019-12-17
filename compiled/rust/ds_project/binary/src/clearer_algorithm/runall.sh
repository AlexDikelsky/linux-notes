#!/bin/bash

#rm bin_nums.txt data.txt octal_nums.txt regex.txt
chmod 755 generator.py grep.sh
./generator.py
./grep.sh > data.txt
