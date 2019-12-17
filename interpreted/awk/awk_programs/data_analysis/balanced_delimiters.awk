BEGIN {
    open_a   = "ao"
    close_a  = "ac"
    string_a = "a opened"
    open_b   = "bo"
    close_b  = "bc"
    string_b = "b opened"
    size     = 0
}


/^ao/ { 
    stack[size+1] = string_a     #It's probably possible to do this by just doing
    size++			 #stack[size++]
}

/^ac/ { 
    if (stack[size] != string_a) {
	print "ac misplaced on line" NR
    }
    else {
	delete stack[size] 
	size--
    }
}

/^bo/ { 
    stack[size+1] = string_b     #It's probably possible to do this by just doing
    size++			 #stack[size++]
}

/^bc/ { 
    if (stack[size] != string_b) {
	print "bc misplaced on line" NR
    }
    else {
	delete stack[size] 
	size--
    }
}

END {
    if (size == 0) {
	print "Correct number of delimiters"
    } else {
	print "Unclosed delimiters"
    }
}
