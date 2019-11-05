##Number of lines in input file
#awk 'END { print NR }' emp.data
#
##Print third line
#awk 'NR == 3 { print }' emp.data
#
##Print last field for each line
#awk '{ print $NF }' emp.data
#
##Print last field of the last input line
##This is how tme book says to do it
#awk '{ last = $NF }; END { print $NF }' emp.data
##This way also works
#awk 'END { print $NF }' emp.data
#
##Print where fields == 3
#awk 'NF == 3 { print }' emp.data
#
##where last field > 4
#awk '$NF > 4 { print }' emp.data
#
##Print total fields in all input lines
#awk '{ fields = fields + NF }; END { print fields }' emp.data
#
##Print where lines contain Beth
#awk '/Beth/ { n = n + 1 } END { print n }' emp.data
#
##Print largest third field and the line that contains it
#awk 'max < $3 { max = $3; max_row = $0 }
#    END { print max " " max_row }' emp.data
#
##Print where line has 1 or more fields
#awk 'NF > 0 { print }' emp.data
#
##Print where len > 80
#awk 'length($0) > 80 { print }'

##Print num times the next line is equal to itself
#awk 'BEGIN { n = 0 } 
#    { prev_line = cur_line }
#    { cur_line = $0 }
#    prev_line == cur_line { n = n + 1 }
#    END { print n }
#    ' dupes.data
#
##Print num fields where the next line is equal to the current line
#awk '{ print NF, $0 }' dupes.data

##Print the first 2 fields, in opposite order, of every line
#awk '{ print $2 " " $1 }' emp.data

##Swap the first and second fields, then the rest of the line
#awk '{ temp = $1; $1 = $2; $2 = temp; print $0 }' emp.data
##This means that you can change what 
##the acutal arguments 'look like' from inside a program

##Print every line with the first field replaced by the line num
#awk '{ $1 = NR; print }' emp.data

##Print ever line after erasing the second field
#awk '{ $2 = ""; print }' emp.data

##Reverse the order of the fields for each line
#awk '{ for (i=NF; i>0; i=i-1) print $i }' alphabet.txt

##Print sum of all fields
#awk '{ for (i=1; i<=NF; i=i+1) sum = sum + $i } END { print sum }' nums.txt

##Print every line, but replace their values with their absolute val
awk '
{ for (i=1; i<=NF; i=i+1) 
    {
	if ($i > 0) {
	    $i=$i
	} else {
	$i=-$i 
    }
}
print 
}' neg.data
