#!/usr/bin/env python3
import math

a = [17, 78, 19, 23, 29, 77, 95, 77,  1, 11, 13, 15, 1, 55]
b = [91, 85, 51, 38, 33, 29, 23, 19, 17, 13, 11,  2, 7,  1]

#a = [17, 78, 19, 23, 29, 77, 95, 77,  1, 11, 13, 15, 15, 55]
#b = [91, 85, 51, 38, 33, 29, 23, 19, 17, 13, 11, 14,  2,  1]


ch = "abcdefghijklmnopqrstuvwxyz"

N = 2

ns = []

def is_integral(x):
    return x == int(x)

runs = 0
while runs < 15:
    i = 0
    found = False
    while i < len(a) and not found:
        if is_integral(N * (a[i]/b[i])):
            print(int(N), ch[i])
            N = N * (a[i]/b[i])
            found = True
        i += 1
    runs += 1
    
