{ line[NR] = $0 }

#This doesn't work becasue I forgot the END at the start
#{ for (i=NR; i > 0; i = i-1)
#    print line[i]
#}

#END { 
#for (i=1; i < NR; i = i+1)
#    print i
#    #print line[i]
#

END { 
for (i=NR; i > 0; i = i-1)
    print line[i]
}
