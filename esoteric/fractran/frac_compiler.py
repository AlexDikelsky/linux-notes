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

# PRIMEGAME
#n = 2
#a = [17, 78, 19, 23, 29, 77, 95, 77,  1, 11, 13, 15, 1, 55]
#b = [91, 85, 51, 38, 33, 29, 23, 19, 17, 13, 11,  2, 7,  1]

#PIGAME
#n = 2
#a =  [ 365,  29,  79, 679,3159,  83, 473, 638, 434,  89,  17,  79]
#b =  [  46, 161, 575, 451, 413, 407, 371, 355, 335, 235, 209, 122]
#
#a += [  31,  41, 517, 111, 305,  23,  73,  61,  37,  19,  89,  41, 833,  53]
#b += [ 183, 115,  89,  83,  79,  73,  71,  67,  61,  59,  57,  53,  47,  43]
#
#a += [  86,  13,  23,  67,  71,  83, 475,  59,  41,   1,   1,   1,   1,  89]
#b += [  41,  38,  37,  31,  29,  19,  17,  13, 291,   7,  11,1024,  97,   1]

#POLYGAME
#a =  [583, 629, 437,  82, 615, 371,   1,  53,  43,  23, 341]
#b =  [559, 551, 527, 517, 329, 129, 115,  86,  53,  47,  46]
#
#a += [ 41,  47,  29,  37,  37, 299,  47, 161, 527, 159,   1,  1,  1]
#b += [ 43,  41,  37,  31,  31,  29,  23,  15,  19,   7,  17, 13,  3]
#
#def f_c(n, c):
#    N = c * (2**(2**(n)))
#    vec = run(N, a, b, 100)
#    d = [2**(2**(x)) for x in range(10)]
#    for val in vec:
#        if val in d:
#            print(val, int(math.log(val), 2))
#
#f_c(2, 2)

nvals = run(n, a, b, 10**5)

for val in nvals:
    if val[0] in pows:
        print(val, int(math.log(val[0], 2)))
