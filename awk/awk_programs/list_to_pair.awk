#Write a program that
    #reads a list of item and quantity pairs
    #for each item on the list accumulates the total quantity
#At the end, print 
    #items, total quantities, sorted alphabeticly by item

    { items[$1] += $2 }
END { for (i in items) print i, items[i] | "sort" } 
