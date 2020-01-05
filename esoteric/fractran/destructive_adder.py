#!/usr/bin/env python3

import math
import math_methods as mm

a = [2]
b = [3]
#print_decompose(compose([3, 9, 0, 2]))
c = mm.run(mm.compose([2, 2]), a, b, 100)
#c = run(24, a, b, 100)
for p in c:
    print("N =", str(p[0]) + "\tFactors = " + str(mm.remove_zeros_decompose(p[0])))


