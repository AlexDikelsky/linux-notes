awk ' > 0 { print $1, $2 * $3 }' emp.data   #Finds where the number of hours worked is greater than 0, then finds sallary
awk '{ print 1 / $1 }' div_by_zero_test.data #Says infinity if you try to divide by 0
awk '{ print 0/0}' <any inpyt> #Yeields -nan
{ print $1, $3 } #Print fields 1 and 3
{ print NF, $1, $NF } #Print the number of fields, the first field, and the last field
{ print NR, $0 } #Prepend the line number to each thing
{ printf("pay for %s is $%.2f\n", $1, $2 * $3) } 
    #%s is just the string
    #$%.2f says "Print a dollar sign, then
    #		    Print the second argument,
    #		    With 2 decimal places
    #		    newline

{ printf("%-8s $%6.2f\n", $1, $2*$3) }  

#Select where second argument >= 5
$2 >= 5 #All you need is this
$1 == "Susie" #Print where $1 is susie
$2 >= 4 || $3 <= 20 #Where 4 <= x <= 20

#Other sutff
NF != 3  #Number of fields is not 3

'BEGIN { print "NAME    RATE  HOURS"; print "" } { print }' emp.data 

'$3 > 15 { emp = emp + 1}; END { print emp }'

#Compute average pay for each employee
awk '{ pay = pay + $2 * $3 }
END { 
    print "Total Pay=", pay
    print "Avg pay=", pay/NR
}
' emp.data  #Note that emp.data has to be on this line if you're
	    #using tmux because it finishes executing before it
	    #readns that filename

#Compute employee with max pay per hour
awk 'BEGIN { max = 0 }  
    max < $2 { max = $2; maxemp = $1 }
    END { print max, maxemp } ' emp.data
#Don't need this first line, note that variables are 
    #implicitly set to 0 if uninitialized

#Compute min pay 
awk '
    BEGIN { min = 0 }  #Here you do need the first line
    min > $2 { min = $2; minemp = $1 }
    END { print min, minemp} ' emp.data

#You need the first line here because if the min is acutally 0,
#it will just print out the empty string

awk '
    { names = names " " $1 }
    END { print names }
    ' emp.data

#Counts lines, words, chars
#The whole second part is on 1 line
awk '
    { chars = chars + length($0); fields = NF + fields }
    END { printf( "lines %s, words %s, chars %s\n", NR, fields, chars) }' emp.data

#awk ' #THIS METHOD DOESN"T WORK
#    { chars = chars + length($0); fields = NF + fields }
#    END { printf( "lines %s, words %s, chars %s\n",\
#	NR,\
#	fields,\
#	chars\
#    ) \
#}' emp.data

#This finds the avg of emps where salary > 6
#n is tne number of emps with sal > 6
awk '
    $2 > 6 { n = n + 1; pay = pay + $2*$3 } 
    END { 
	if (n > 0)
	    print n, "emps, total pay is", pay, "avg pay", pay/n
	else
	    print "no emps with wage > 6 per hour"
    }' emp.data




