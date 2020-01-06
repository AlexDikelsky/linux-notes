#!/usr/bin/env python3

import math
import math_methods as mm

a = [10, 3,  1]
b = [ 3, 5, 17]
#print_decompose(compose([3, 9, 0, 2]))
c = mm.run(mm.compose([1, 1]), a, b, 100)
#c = run(24, a, b, 100)
for p in c:
    print("N =", str(p[0]) + "\t\tFactors = " + str(mm.remove_zeros_decompose(p[0])))

print(mm.decompose(39))
