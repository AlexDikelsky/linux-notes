function max(m, n) {
    return (n>m) ? n : m  
}
#You can evaluate boolean expressions very concisely




{  #Test code
    for (i=0; i<5; ++i) {
	for (j=0; j<5; ++j) {
	    print i, j, max(i, j)
	}
    }
}
