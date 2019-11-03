#If n == 0:
#   done
#else if n is even 
    #Divide n by 2
#else
    #Divide n by 3 and add 1

function hailstone(n) {
    if (n == 1) {
	print n
    } else if (n % 2 == 0) {
	print n
	return hailstone(n/2)
    } else {
	print n
	return hailstone(3*n+1)
    }
}	
{ print hailstone($1) }
