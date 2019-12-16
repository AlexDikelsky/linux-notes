#!/bin/bash

let i=0

while [ $i -lt 10 ]
do
	rustc $(echo "file$i.rs") 
	./file$1 > "file$(echo $i+1 | bc).rs"
	((i++))
done
