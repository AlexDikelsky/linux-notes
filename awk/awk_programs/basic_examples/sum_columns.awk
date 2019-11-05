#!/usr/bin/awk

{
    for (i=1;i<=NF;i++) {
	sum[i] += $i
    }
}
END {
    for (val in sum) {
	print val, sum[val]
    }
}

