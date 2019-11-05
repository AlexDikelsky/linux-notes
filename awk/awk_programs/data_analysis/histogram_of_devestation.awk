BEGIN { buckets = 3-1 }  
#Theres a bug which causes the last event to have their own group
NR == 1 {max_val = $1; min_val = $1; }
{ 
    x[NR] = $1  #Save evertying
    total++ 
    max_val = max(max_val, $1) #Find the total max and min
    min_val = min(min_val, $1)
}

END { 
    bucket_size = ((max_val - min_val +1) / buckets)
    for (item in x)
	#print int(x[item]/bucket_size), x[item], bucket_size
	y[int(x[item]/bucket_size)]++
    for (i=0; i<=buckets; i++)
	printf(" %2d - %2d : %3d %s\n",
	       i*bucket_size,
	       i*bucket_size + bucket_size-1,
	       y[i],
	       rep(y[i], "*"))
#    printf("      %2d: %3d %s\n",
#	   i*bucket_size,
#	   y[i],
#	   rep(y[i], "*"))
}

function rep(n, s,    t) {
    while (n-- > 0)
	t = s t
    return t
}
function max(m, n) {
    return m > n ? m : n
}
function min(m, n) {
    return m < n ? m : n
}
