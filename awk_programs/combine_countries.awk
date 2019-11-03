#/Asia/ { population["Asia"] += $3 }
#/Europe/ { population["Europe"] += $3 }
#END { print population["Asia"], population["Europe"] }

BEGIN {FS = "\t" }
{ populations[$4] += $3 } 
END { for (name in populations)
    print name, pop[name]
    }
