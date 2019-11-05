#v=P(1+r)^(t)
#P = $1
#R = $2
#number of times to run = $3

{ t = 1
    while ( t < $3 ) {
	printf("\tt=%s, $=%.2f\n", t, $1*(1+$2)^t)
	t += 1
    } 
}


