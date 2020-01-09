#!/usr/bin/env python3

#THis has translations form haskell into python
#so I'm sure I'm undersanding what's happening

def maximum(x):
    if x == []:
        return "Empty list"
    if len(x) == 1:
        return x[0]
    else:
        if x[0] > maxTail(x):
            return x[0]
        else:
            return maxTail(x)

def maxTail(x):
    return maximum(x[1:])

print(maximum([2,3,4,0]))
