#!/usr/bin/env python3

import math

def is_integral(N, a, b):  
    #Multiples 2 numbers (N and numerator), then checks 
    #if they are divisible by the denomenator
    return (N*a) % b == 0

def run(N, numer, denom, upper_bound):
    #N -> int
    #numer -> list of ints
    #denom -> list of ints
    #upper_bound -> number of times to run
    steps = 0
    ns = [[N, 0]]  #the 0 is the step
    runs = 0
    #Technicly don't need this, but it makes sure that programs
    #will halt
    while runs < upper_bound:  
        i = 0  #Keeps track of the current fraction
        found = False
        while i < len(numer) and not found:
            if is_integral(N, numer[i], denom[i]):
                #print(N, runs)
                N = (N * numer[i]) // denom[i]
                ns.append([N, runs+1])
                found = True
            i += 1
        runs += 1
    return ns

def print_pows_of_2(vals, maximum = 50):  
    #A lot of programs (like PRIMEGAME) "return"
    #whenever N is a power of 2, so I added this
    pows = [2**i for i in range(maximum)]
    for val in vals:
        if val[0] in pows:
            print(val, int(math.log(val[0], 2)))

def print_list(vals):
    for val in vals:
        print(val)

def compose(l):  #Takes a list of factors and times they happen, then creates a number
    number = 1
    i = 0
    while i < len(l):
        number *= (i+2) ** l[i]
        i += 1
    return number

def decompose(n):  #Makes a list of factors given a number
    #This is really verbose because it has a lot more 0s than needed
    factors = [[x, 0] for x in range(1, n+1)]  
    #0 is the power of the prime number
    k = 2
    while n > 0 and k <= n:
        #print(n, k)
        if n % k == 0:
            factors[k-1][1] += 1
            n = n // k
        else:
            k += 1
    return factors


def remove_zeros_decompose(n):
    l = decompose(n)
    res = []   #Return only elements that occur at least once
    for val in l:
        if val[1] is not 0:
            res.append(val)
    return res

