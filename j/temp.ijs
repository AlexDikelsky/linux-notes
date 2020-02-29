NB. y is a named argument for the second half of the function
centigrade =: 3 : 0
t1 =. y - 32
t2 =. t1 * 5
t3 =. t2 % 9
)
NB. Note that the =. makes these local to the verb being defined

NB. You can't do it this way because it always does monad, then dyad

NB. minus =: 3 : 0
NB. x - y
NB. :
NB. - y
NB. )

NB. This works fine
minus =: 3 : 0
- y
:
x - y
)

NB. The previous definition of y doesn't do anything here,
NB. but it's still probably a bad idea to define y as a global var
y =: 'i''m a string'
abc =: 3 : 0
y
)


