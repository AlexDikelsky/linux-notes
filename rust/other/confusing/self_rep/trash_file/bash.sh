#!/bin/bash

let i=0

while [ $i -lt 10 ]     #Change 10 to the number of copies you want
do
	rustc $(echo "file$i.rs")  #makes a file with variable name
	./file$i > "file$(echo $i+1 | bc).rs" #exectes resulting file and writes the output to a new file
	((i++))
done
