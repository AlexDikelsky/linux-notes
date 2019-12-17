#!/usr/bin/env python3

import math

def is_integral(N, a, b):
    return (N*a) % b == 0

def run(N, numer, denom, upper_bound):
    ns = [[N, 0]]
    runs = 0
    while runs < upper_bound:
        i = 0
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

def print_pows_of_2(vals):
    pows = [2**i for i in range(50)]
    for val in vals:
        if val[0] in pows:
            print(val, int(math.log(val[0], 2)))

def print_list(vals):
    for val in vals:
        print(val)

def compose(l):
    number = 1
    i = 0
    while i < len(l):
        number *= (i+2) ** l[i]
        i += 1
    return number

def decompose(n):
    factors = [[x, 0] for x in range(1, n+1)]
    k = 2
    while n > 0 and k <= n:
        #print(n, k)
        if n % k == 0:
            factors[k-1][1] += 1
            n = n // k
        else:
            k += 1
    return factors

def print_decompose(n):
    l = decompose(n)
    res = []
    for val in l:
        if val[1] is not 0:
            res.append(val)
    return res

a = [10, 3,  1]
b = [ 3, 5, 17]
#print_decompose(compose([3, 9, 0, 2]))
c = run(compose([1, 1]), a, b, 100)
#c = run(24, a, b, 100)
for p in c:
    print("N =", str(p[0]) + "\t\tFactors = " + str(print_decompose(p[0])))


