#on the first row, make all the col_summations be the things there
#provided they are a number
NR==1 { nfld = NF
    for (i=1; i<=NF; i++)
	numcol[i] = isnum($i)
}

#For every row, if it was a int in the first thing,
#Add more to it
#Also ignore blank lines
#If you remove numcol, you just add everything you 
#tried to remove at NR==1
    { 
	if (length($0) != 0) {
	    for (i=1; i<=NF; i++)
		if (numcol[i])
		    sum[i] += $i
	}
    }

#Iterate through all the fields, including text fields
#If it is numerical, print it out
#If it is not, print 2 dashes
END { 
    for (i=1; i<= nfld; i++) {
	if (numcol[i])
	    printf("%g", sum[i])
	else
	    printf("--")
	printf(i < nfld ? "\t" : "\n")
    }
}

function isnum(n) { return n ~ /^[+-]?[0-9]+([eE][+-][0-9]+)?$/}


