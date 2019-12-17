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


nvals = run(n, a, b, 10**5)

for val in nvals:
    if val[0] in pows:
        print(val, int(math.log(val[0], 2)))
