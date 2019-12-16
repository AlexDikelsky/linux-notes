#!/usr/bin/env python3
import subprocess

octal_nums_filename = "octal_nums.txt"
bin_nums_filename = "bin_nums.txt"
regex_filename = "regex.txt"

def gen_octal():
    power = 4
    octal_nums = open(octal_nums_filename, "w")
    for i in range((8**power)):
        octal_nums.write("{:04o}\n".format(i))
    octal_nums.close()

def gen_bin():
    power = 4
    bin_nums = open(bin_nums_filename, "w")
    for i in range((2**power)):
        bin_nums.write("{:04b}\n".format(i))
    bin_nums.close()

def octal_map(char):  #{{{
    if char == '0':
        return '0'
    elif char == '1':
        return '1'
    elif char == '2':
        return '+'
    elif char == '3':
        return '*'
    elif char == '4':
        return '.'
    elif char == '5':
        return '?'
    elif char == '6':
        return 'x'
    elif char == '7':
        return '|'   #}}}

def octal_to_regex():
    octal_file = open(octal_nums_filename, "r")
    regex_file = open(regex_filename, "w")

    lines = octal_file.read()
    lines = lines.split('\n')

    for line in lines:
        for num in line:
            regex_file.write(octal_map(num))
        regex_file.write("\n")

gen_octal()
gen_bin()
octal_to_regex()
