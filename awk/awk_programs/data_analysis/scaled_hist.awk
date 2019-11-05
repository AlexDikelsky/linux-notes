    { x[int($1/10)]++; sum++ }
END { 
    maxlen = 25;
    
    for (i=0; i<10;i++)
	if (x[i] > modenum) { modenum = x[i] }

    scale = maxlen/modenum

    for (i=0; i<10;i++)
	printf(" %2d - %2d: %3d %s\n",
	       i*10, i*10+9, x[i],
	       rep(int((maxlen/modenum)*x[i]), "*"))
    printf("     100: %3d %s\n",
	   x[10], rep(int((maxlen/modenum)*x[10])))
    }

function rep(n, s,   t) {
    while (n-- > 0) 
	t = t s
    return t
}
