function fizzbuzz(max) {
    #If i is divisible by 3, print fizz
    #If i is divisible by 5, print buzz
    #If I is divisible by both, print fizzbuzz
    for (i=1; i<=max;++i) {
	result = (i % 5) ? ((i % 3) ? i : "buzz") : ((i % 3) ? "fizz" : "fizzbuzz")
	print result
		
    }
}

{ fizzbuzz($1) }
