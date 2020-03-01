NB. Fast algorithm for finding powers of 2 without needing large factorials
NB. http://oeis.org/A011371
b =: monad define
if. y=0
do. 0
else. (<. y % 2) + b <. y % 2
end.
)

NB. Number of twos in x choose y
g =: dyad define
(b x) - (b y) + (b x - y)
)

NB. Can probably make the i.y thing be shorter
max_col =: monad define
>./ y g"0 (i. 1 + <. y % 2)
)


NB. Test function
NB. f =: dyad define
NB. +/2=q:y ! x
NB. )


NB. THis wasn't the right way to do it
NB. gen =: monad define
NB. i =: 1
NB. r =: ''
NB. sum =. 0
NB. cur_max =. 0
NB. while. i < y+1 do.
NB.     f =. max_col i
NB.     if. cur_max < f do. 
NB. 	cur_max =. f
NB.     end.
NB.     sum =. sum + cur_max
NB.     r =: r, cur_max
NB.     i =: i + 1
NB. end.
NB. sum
NB. )


NB. This is not as good
NB. The first half is always the same as the second half, so you can divide by 2
NB. runs =. >. y+1 % 2
NB. r =. ''
NB. i =. 0
NB. while. i < runs do.
NB.     r =. r, (y g i)
NB.     i =. i + 1
NB. end.
NB. c =: >./ r
NB. )

