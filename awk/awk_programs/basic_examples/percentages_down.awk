    { x[NR] = $1; sum += $1 }
END { if (sum != 0)
	for (i=1; i<NR; i++) 
	    printf("%10.2f %5.1f\n", x[i], 100*x[i]/sum)
    }
