#(p, r, t)
{
    for (t = 1; t <= $3; t=t+1) {
	printf("\tt=%s, $%.2f\n", t, $1*(1+$2)^t)
    }
}
