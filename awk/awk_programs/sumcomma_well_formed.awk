{ 
    if ($0 ~ /^[+-]?[0-9][0-9]?[0-9]?(,[0-9][0-9][0-9])*([.][0-9][0-9])?$/) {
	gsub(/,/, "")
	sum += $0
    }
    else {
	print "malformed", $0
    }
}

#(s ~ /^[+-]?[0-9][0-9]?[0-9]?.*/) 
END { print sum }