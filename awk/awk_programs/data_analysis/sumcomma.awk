    { gsub(/,/, ""); sum += $0 }
END { print sum }
