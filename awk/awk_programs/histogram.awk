    { x[int($1/10)]++ }
END { for (i=0; i<10; i++) 
	printf(" %2d - %2d: %3d %s\n",
	       10*i, 10*i+9, x[i], rep(x[i], "*"))
	printf("100:      %3d %s\n", x[10], rep(x[10], "*"))
    }

function rep(n, s, t) {
    scale = .5
    while (n > 0)
	t = t s
	n -= 1 * 1/scale
    return t
}

#awk ' #random number generator
#BEGIN { for (i=1; i<=200; i++)
#    print int(101*rand())
#}
#' |
#awk -f awk_programs/histogram.awk
