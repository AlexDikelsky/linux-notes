#!/usr/bin/env python3

def print_list(list):
    for item in list:
        print(item, sep="", end="")
    print()

def print_regex(list):
    for item in list:
        print(base_4_map(str(item)), sep="", end="")
    print()

def base_b(base, size, map_to_regex=False):
    num = [0 for i in range(size)]
    print_list(num)
    for i in range(base**size-1):
        #Incrament the last one until it becomes 4, then set it to 0
        #After you set the num to 0, add one to the next, unless you need to carry again.
        
        num[-1] += 1   #inc last number
        for j in range(size -1, -1, -1):  #Loop through list backwards
            if num[j] == base:  #If the number is too big
                num[j] = 0      #set it to 0
                num[j-1] += 1   #inc the 'next' number
        if not map_to_regex:
            print_list(num)
        else:
            print_regex(num)
            
def base_4_map(char):
    if char == '0':
        return '0'
    elif char == '1':
        return '1'
    elif char == '2':
        return '.'
    elif char == '3':
        return '*'

#print_regex(['1', '2', '3', '0'])
#base_b(4, 8, True)
#base_b(2, 8, False)
