#!/usr/bin/env python3
import math

def is_integral(N, a, b):
    return (N*a) % b == 0

def run(N, numer, denom, upper_bound):
    ns = []
    runs = 0
    while runs < upper_bound:
        i = 0
        found = False
        while i < len(numer) and not found:
            if is_integral(N, numer[i], denom[i]):
                #print(N, runs)
                ns.append([N, runs])
                N = (N * numer[i]) // denom[i]
                found = True
            i += 1
        runs += 1
    return ns

pows = [2**i for i in range(50)]

n = 2
a = [17, 78, 19, 23, 29, 77, 95, 77,  1, 11, 13, 15, 1, 55]
b = [91, 85, 51, 38, 33, 29, 23, 19, 17, 13, 11,  2, 7,  1]

nsdf = run(n, a, b, 1000000)

for val in nsdf:
    if val[0] in pows:
        print(val, int(math.log(val[0], 2)))
